use chrono::prelude::*;
use reqwest::{Client, Error, Method, Request, Url};

use constants::CURRENT_LEAGUE;
use types::item::Item;
use types::poe_ninja::NinjaCurrencyOverviewResponse;

fn get_currency_prices(client: &Client) -> Result<NinjaCurrencyOverviewResponse, Error> {
    let url = create_request_url("currencyoverview", "currency");
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
        "https://poe.ninja/api/data/{}?league={}&type=Currency&date={}",
        endpoint, CURRENT_LEAGUE, today
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
