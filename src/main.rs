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

    let bind_str = env::var("BIND_STR");

    match bind_str {
        Err(_) => println!("Could not read bind str"),
        Ok(val) => {
            match SocketAddr::from_str(val.as_str()) {
                Err(_) => println!("Could not parse BIND_STR"),
                Ok(val) => {
                    Iron::new(router).http(val).unwrap();
                    println!("Ok!")
                }
            }
        }
    }
}
