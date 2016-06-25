extern crate iron;

use iron::prelude::{Iron, IronResult, Request, Response};
use iron::status;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
}

fn main() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let addr = "0.0.0.0:".to_string() + &port;
    Iron::new(handler).http(&*addr).unwrap();
}
