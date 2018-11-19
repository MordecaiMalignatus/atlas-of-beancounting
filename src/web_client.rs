use chrono::prelude::*;
use reqwest::{Client, Error, Method, Request, Url};

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use constants::CURRENT_LEAGUE;
use types::poe_ninja::NinjaCurrencyOverviewResponse;
use types::pricing::{Price, PriceMessage};

type PriceCache<'a> = HashMap<&'a str, CacheEntry>;

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
                        // system know that this item isn't priced by poe.ninja
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

fn refresh_gear_cache(_cache: &mut HashMap<&str, (Price, DateTime<Local>)>) -> Result<(), Error> {
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

#[cfg(test)]
mod test {
    use super::*;

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
