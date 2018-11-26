use chrono::prelude::*;
use chrono::Duration;
use reqwest::{Client, Error, Method, Request, Url};

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use constants::{CURRENT_LEAGUE, POE_NINJA_ENDPOINT_TEMPLATES};
use types::poe_ninja::NinjaCurrencyOverviewResponse;
use types::pricing::{Price, PriceMessage};

type PriceCache = HashMap<String, CacheEntry>;

struct CacheEntry {
    price: Price,
    expiration: DateTime<Local>,
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

/// Refresh the passed cache with new poe.ninja data.  This will hit up all the
/// endpoints in poe.ninja and move all the response data into the cache --
/// currently sequentially. This would benefit greatly from throwing a rayon
/// par_iter() on there, even though I'm not sure how well Reqwest handles
/// that. Definitely needs to be made either asynchronous or parallel.
fn refresh_gear_cache(cache: &mut PriceCache) -> Result<(), Error> {
    let client = Client::new();
    let prices: Vec<Price> = POE_NINJA_ENDPOINT_TEMPLATES
        .to_vec()
        .iter()
        .map(|url| url.replace("{}", CURRENT_LEAGUE))
        .map(|url| client.get(&url).send())
        .filter(|result| result.is_ok())
        .map(|res| res.expect("Can't unwrap Request when refreshing gear cache"))
        .filter(|resp| resp.status().is_success())
        .map(|mut resp| {
            resp.json::<NinjaCurrencyOverviewResponse>()
                .expect("Should be able to parse a poe.ninja response.")
        }).flat_map(|resp| resp.lines.into_iter().map(|line| Price::from(line)))
        .collect();

    println!("Fetched {} prices, updating cache...", prices.len());

    let _ = prices
        .into_iter()
        .map(|price| CacheEntry {
            price,
            expiration: calculate_expiration_date(Local::now()),
        }).map(|entry| cache.insert(entry.price.currency_name.clone(), entry));

    Ok(())
}

fn calculate_expiration_date(now: DateTime<Local>) -> DateTime<Local> {
    let offset = Duration::hours(1);
    now.checked_add_signed(offset)
        .expect("The heat death of universe is near, date addition would overflow")
}

#[cfg(test)]
mod test {
    use super::*;

    // Needs new tests, I obsoleted them all :(
}
