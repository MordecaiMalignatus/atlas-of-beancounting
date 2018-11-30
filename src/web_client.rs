use chrono::prelude::*;
use chrono::Duration;
use reqwest::{Client, Error};

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use constants::{CURRENT_LEAGUE, POE_NINJA_ENDPOINT_TEMPLATES};
use types::poe_ninja::NinjaCurrencyOverviewResponse;
use types::pricing::{Price, PriceMessage};

type PriceCache = HashMap<String, CacheEntry>;

#[derive(Debug)]
struct CacheEntry {
    price: Price,
    expiration: DateTime<Local>,
}

fn spawn_price_bot(recv: &Receiver<PriceMessage>, sender: &Sender<PriceMessage>) -> ! {
    let mut price_cache: PriceCache = HashMap::new();

    loop {
        match recv.recv() {
            Ok(o) => match o {
                PriceMessage::Get { item } => match query_cache(&mut price_cache, &item) {
                    Some(price) => match sender.send(PriceMessage::Response { item, price }) {
                        Ok(()) => {}
                        Err(e) => panic!("Could not send price response: {}", e),
                    },
                    None => {
                        match sender.send(PriceMessage::Response {
                            item: item.clone(),
                            price: Price { name: item, chaos_equivalent: 0.0
                        }}) {
                            Ok(()) => {},
                            Err(e) => panic!("Could not send price response: {}", e),
                        }
                    }
                },
                PriceMessage::Response { .. } => {
                    panic!("How is a Response on the request channel?");
                }

                PriceMessage::InvalidateCache => match refresh_price_cache() {
                    Ok(c) => price_cache = c,
                    Err(e) => {
                        println!("Can't refresh cache while invalidating, using old cache instead.\nError: {}", e)
                    }
                },
            },

            Err(e) => panic!("Error when receiving price request: {}", e),
        }
    }
}

fn query_cache(_cache: &mut PriceCache, _key: &str) -> Option<Price> {
    let _now = Local::now();
    unimplemented!()
}

/// Refresh the passed cache with new poe.ninja data.  This will hit up all the
/// endpoints in poe.ninja and move all the response data into the cache --
/// currently sequentially. This would benefit greatly from throwing a rayon
/// par_iter() on there, even though I'm not sure how well Reqwest handles
/// that. Definitely needs to be made either asynchronous or parallel.
fn refresh_price_cache() -> Result<PriceCache, Error> {
    let client = Client::new();
    let prices: Vec<Price> = POE_NINJA_ENDPOINT_TEMPLATES
        .to_vec()
        .iter()
        .map(|url| url.replace("{}", CURRENT_LEAGUE))
        .map(|url| client.get(&url).send())
        .map(|res| res.expect("Can't unwrap Request when refreshing gear cache"))
        .map(
            |mut resp| match resp.json::<NinjaCurrencyOverviewResponse>() {
                Ok(result) => result,
                Err(e) => panic!(
                    "Can't parse response into poe.ninja type, got {:?} instead, error: {}",
                    resp, e
                ),
            },
        ).flat_map(|resp| resp.lines.into_iter().map(|line| Price::from(line)))
        .collect();

    println!("Fetched {} prices, updating cache...", prices.len());
    let mut cache = PriceCache::new();
    let _ = prices
        .into_iter()
        .map(|price| CacheEntry {
            price,
            expiration: calculate_expiration_date(Local::now()),
        }).for_each(|entry| {
            cache.insert(entry.price.name.clone(), entry);
        });

    Ok(cache)
}

fn calculate_expiration_date(now: DateTime<Local>) -> DateTime<Local> {
    let offset = Duration::hours(1);
    now.checked_add_signed(offset)
        .expect("The heat death of universe is near, date addition would overflow")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn should_update_cache() {
        let cache = refresh_price_cache().unwrap();

        println!("{:?}", cache);
        assert!(cache.len() > 0;)
    }
    // Needs new tests, I obsoleted them all :(
}
