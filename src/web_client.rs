use chrono::prelude::*;
use reqwest::{Client, Error, Method, Request, Url};

use constants::CURRENT_LEAGUE;
use types::item::Item;
use types::pricing::Price;

fn get_currency_prices(client: &Client) -> Result<Vec<Price>, Error> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let league = CURRENT_LEAGUE.clone();

    let endpoint = Url::parse(&format!(
        "https://poe.ninja/api/data/currencyoverview?league={}&type=Currency&date={}",
        today, league
    )).unwrap();

    let request = Request::new(Method::GET, endpoint);
    let mut response = client.execute(request)?;

    match response.status().is_success() {
        true => {
            // This won't actually work. Possible options here are either:
            // 1. Stubbing out the entire return struct from poe.ninja and
            // marking a lot of fields with #[skip_deserialize] to avoid a
            // shitton of data I have no need for.

            // 2. Using `serde_json::Value' as the main type in a Map and
            // operating on it like a hasahmap.
            let body: Vec<Price> = response.json()?;
            Ok(body)
        }
        false => unimplemented!(),
    }
}
