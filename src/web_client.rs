use chrono::prelude::*;
use chrono::Duration;
use reqwest::{Client, Error};

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use constants::{CURRENT_LEAGUE, POE_NINJA_ENDPOINT_TEMPLATES};
use types::poe_ninja::NinjaCurrencyOverviewResponse;
use types::pricing::{Price, PriceMessage};

type PriceCache = HashMap<String, Price>;

pub struct PriceBot {
    response_channel: Sender<PriceMessage>,
    request_channel: Receiver<PriceMessage>,
    price_cache: PriceCache,
    cache_expiration: DateTime<Local>,
}

impl PriceBot {
    pub fn new(sender: Sender<PriceMessage>, receiver: Receiver<PriceMessage>) -> PriceBot {
        // Will be invalid immediately, because the cache is empty, and get
        // fixed on first request.
        let cache_expiration = Local::now();
        let price_cache = HashMap::new();

        PriceBot {
            response_channel: sender,
            request_channel: receiver,
            price_cache,
            cache_expiration,
        }
    }

    /// Run the price bot. This will lock in an endless loop, so do it in a
    /// thread. During this, the Bot will listen to requests on the Receiver
    /// given to it, and send its responses to the Sender you gave it. The
    /// possilble messages are Enum variants of `PriceMessage`, and obviously
    /// constructing a Response variant and sending it to the bot will panic the
    /// bot. Don't be a smartass. :)
    pub fn run(&mut self) -> ! {
        loop {
            match self.request_channel.recv() {
                Ok(o) => match o {
                    PriceMessage::Get { item } => self.respond_to_price_request(item),
                    PriceMessage::InvalidateCache => self.invalidate_cache(),
                    PriceMessage::Response { .. } => {
                        panic!("How is a Response on the request channel?");
                    }
                },
                Err(e) => panic!("Error when receiving price request: {}", e),
            }
        }
    }

    fn respond_to_price_request(&mut self, item: String) -> () {
        if self.cache_expiration > Local::now() {
            self.send_price_response(item)
        } else {
            match refresh_price_cache() {
                Ok(cache) => {
                    self.price_cache = cache;
                    self.send_price_response(item)
                }
                Err(e) => {
                    println!(
                        "[PriceBot] Can't update cache, contuing with old. Error: {}",
                        e
                    );
                    self.send_price_response(item)
                }
            }
        }
    }

    fn send_price_response(&self, item: String) -> () {
        let price = match self.price_cache.get(&item) {
            Some(price) => (*price).clone(),
            // Send back dummy for display purposes. It still will appear, we
            // just don't have a price for it.
            None => Price {
                name: item.clone(),
                chaos_equivalent: 0.0,
            },
        };

        match self
            .response_channel
            .send(PriceMessage::Response { item, price })
        {
            Ok(()) => {}
            Err(e) => panic!(
                "[PriceBot] Can't send pricing response,\
                 error: {}",
                e
            ),
        }
    }

    fn invalidate_cache(&mut self) -> () {
        match refresh_price_cache() {
            Ok(c) => {
                self.price_cache = c;
                self.cache_expiration = calculate_expiration_date(Local::now());
                println!("[PriceBot] Invalidated and refreshed cache");
            }
            Err(e) => println!(
                "Can't refresh cache while invalidating, using old\
                 cache instead.\nError: {}",
                e
            ),
        }
    }
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
        ).flat_map(|resp| resp.lines.into_iter().map(Price::from))
        .collect();

    println!("Fetched {} prices, updating cache...", prices.len());
    let mut cache = PriceCache::new();
    prices.into_iter().for_each(|price| {
        cache.insert(price.name.clone(), price);
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

    #[test]
    fn should_update_cache_expiry() {
        use std::sync::mpsc;
        use std::thread;

        let mut cache: PriceCache = HashMap::new();
        // Create deliberately invalid starting data.  If the cache is
        // invalidated correctly, the outside world will never see this.
        cache.insert(
            "Exalted Orb".to_string(),
            Price {
                name: "Exalted Orb".to_string(),
                chaos_equivalent: -111111.0,
            },
        );

        let (sender, receiver_bot) = mpsc::channel();
        let (sender_bot, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut price_bot = PriceBot {
                response_channel: sender_bot,
                request_channel: receiver_bot,
                cache_expiration: Local::now(),
                price_cache: cache,
            };
            price_bot.run()
        });

        match sender.send(PriceMessage::Get {
            item: "Exalted Orb".to_string(),
        }) {
            Ok(()) => {}
            Err(e) => panic!("Can't send Price/Get Message: {}", e),
        };

        match receiver.recv() {
            Ok(o) => match o {
                PriceMessage::Response { item, price } => {
                    assert_eq!(item, "Exalted Orb".to_string());
                    assert!(price.chaos_equivalent > 0.0);
                }
                _ => panic!("Not a response"),
            },
            _ => panic!("Can't read from channel after updating Price and asking again."),
        }
    }
}
