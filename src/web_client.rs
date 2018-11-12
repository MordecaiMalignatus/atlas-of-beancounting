use std::io::{self, Write};
use types::item::Item;
use types::pricing::Price;
use reqwest::Error;
use reqwest::get;

// pub fn get_price() -> () {
//     rt::run(get_request());
// }

pub fn get_request() -> Result<(), Error> {
    let url =
        "http://poe.ninja/api/data/currencyoverview?league=Delve&type=Currency&date=2018-11-12";

    let mut resp = get(url)?;
    println!("Body: \n\n{}", resp.text()?);
    Ok(())
}
