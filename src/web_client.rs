use chrono::prelude::*;
use chrono::Duration;
use reqwest::{Client, Error, Method, Request, Url};

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use constants::CURRENT_LEAGUE;
use types::poe_ninja::NinjaCurrencyOverviewResponse;
use types::pricing::{Price, PriceMessage};

type PriceCache = HashMap<String, CacheEntry>;

struct CacheEntry {
    price: Price,
    expiration: DateTime<Local>,
    // This is the "kind" of an item according to poe.ninja, like "Currency" or "Fragment".
    source_category: String,
}

fn spawn_price_bot(recv: Receiver<PriceMessage>, sender: Sender<PriceMessage>) -> ! {
    let mut price_cache: PriceCache = HashMap::new();

    loop {
        match recv.recv() {
            Ok(o) => match o {
                PriceMessage::Get { item: item } => match query_cache(&mut price_cache, &item) {
                    Some(price) => match sender.send(PriceMessage::Response { item, price }) {
                        Ok(()) => {}
                        Err(e) => panic!("Could not send price response: {}", e),
                    },
                    None => {
                        // TODO: Send back dummy response to let the rest of the
                        // system know that this item isn't priced by poe.ninja,
                        // or has "low confidence" pricing.
                    }
                },
                PriceMessage::Response { .. } => {
                    panic!("How is a Response on the request channel?");
                }
            },

            Err(e) => panic!("Error when receiving price request: {}", e),
        }
    }
}

fn query_cache(_cache: &mut PriceCache, _key: &str) -> Option<Price> {
    let _now = Local::now();
    unimplemented!()
}

/*
Cache update can be implemented in one function, since the "source_category" is
contained in the cahce. This turns it into a recursive function:

- iterate through entries, find the first expired
- Update the category belonging to that item, yield updated cache, recurse
- If nothing is expired at some point, you can just return.

Concerns about this:

- This would be a sequential update for something that is parallalizeable, and
  easily so. I could make a list for all the URLs, run out all the requests,
  turn it into `CacheEntry` structs, and then sequentially insert into the
  cache.
- Everything expires at roughly the same time anyway (due to on-start
  information grabbing) making a bulk-update function very convenient.
- A scheduled bulk update could be done on a separate thread and then just sent
  to the pricing actor, obviating the need to do it here, and just slotting the
  new cache into place.
*/

fn refresh_gear_cache(_cache: &mut PriceCache) -> Result<(), Error> {
    Ok(())
}

fn refresh_currency_cache(cache: &mut PriceCache) -> Result<(), Error> {
    // TODO: Find some way to not construct the client per request and pass it
    // here somehow.
    let raw_prices = get_currency_prices(&Client::new())?;
    let prices: Vec<Price> = raw_prices
        .lines
        .into_iter()
        .map(|line| line.into())
        .collect();

    let _ = prices
        .into_iter()
        .map(|price| CacheEntry {
            price,
            source_category: "currency".to_string(),
            expiration: calculate_expiration_date(Local::now()),
        }).map(|cache_entry| cache.insert(cache_entry.price.currency_name.clone(), cache_entry));

    Ok(())
}

fn get_currency_prices(client: &Client) -> Result<NinjaCurrencyOverviewResponse, Error> {
    let url = create_request_url("currencyoverview", "Currency");
    let request = Request::new(Method::GET, url);
    let mut response = client.execute(request)?;

    match response.status().is_success() {
        true => {
            let body: NinjaCurrencyOverviewResponse = response.json()?;
            Ok(body)
        }
        false => unimplemented!(),
    }
}

fn create_request_url(endpoint: &str, kind: &str) -> Url {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let string = format!(
        "https://poe.ninja/api/data/{}?league={}&type={}&date={}",
        endpoint, CURRENT_LEAGUE, kind, today
    );

    Url::parse(&string).unwrap()
}

fn calculate_expiration_date(now: DateTime<Local>) -> DateTime<Local> {
    let offset = Duration::hours(1);
    now.checked_add_signed(offset)
        .expect("The heat death of universe is near, date addition would overflow")
}

#[cfg(test)]
mod test {
    use super::*;

    mod cache_update {
        use super::*;

        #[test]
        fn should_update_currency_cache() {
            let mut cache = PriceCache::new();
            refresh_currency_cache(&mut cache);
            // T'is a magic number test that's valid as of Delve, it's the
            // number of items returned by the poe.ninja api when queried for
            // the currency overview.
            assert_eq!(cache.len(), 45);
        }
    }

    #[test]
    fn should_deserialize_correctly() {
        // This can be considered successful if serde does not fail while trying
        // to deserialize. It's also a good way to watch for API changes.
        match get_currency_prices(&Client::new()) {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(e) => {
                println!("error happened: {}", e);
                assert!(false);
            }
        }
    }
}
