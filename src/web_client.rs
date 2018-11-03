use std::io::{self, Write, Error};
// use hyper::{Request, Method, Client};
// use hyper::rt::{self, Future, Stream};
use types::pricing::Price;
use types::item::Item;

pub fn get_price(item: Item) -> Result<Price, Error> {
    unimplemented!()
}
