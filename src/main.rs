extern crate iron;
extern crate router;
extern crate rustc_serialize;

use router::Router;
use iron::prelude::*;
use iron::status::Status;
use rustc_serialize::json;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

mod memory;
mod helpers;
mod temperature;
mod load;


fn main() {
    let mut router = Router::new();
    router.get("/temperature", |_: &mut Request| -> IronResult<Response> {
        let temp = temperature::temperature();
        match temp {
            Err(err) => Ok(Response::with((Status::InternalServerError, err))),
            Ok(val) => Ok(Response::with((Status::Ok, json::encode(&val).unwrap())))
        }
    }, "temperature");

    router.get("/memory", |_: &mut Request| -> IronResult<Response> {
        let mem = memory::read_usage();
        match mem {
            Err(err) => Ok(Response::with((Status::InternalServerError, err))),
            Ok(val) => Ok(Response::with((Status::Ok, json::encode(&val).unwrap())))
        }
    }, "memory");

    router.get("/load", |_: &mut Request| -> IronResult<Response> {
        let mem = load::load();
        match mem {
            Err(err) => Ok(Response::with((Status::InternalServerError, err))),
            Ok(val) => Ok(Response::with((Status::Ok, json::encode(&val).unwrap())))
        }
    }, "load");

    let router = env::var("BIND_STR").map_err(|e| e.to_string())
        .and_then(|s| SocketAddr::from_str(s.as_str()).map_err(|e| e.to_string()))
        .and_then(|s| Iron::new(router).http(s).map_err(|e| e.to_string()));


    match router {
        Err(err) => println!("Error: {}", err),
        Ok(_) => { /* Router unwrapped */ }
    }
}
