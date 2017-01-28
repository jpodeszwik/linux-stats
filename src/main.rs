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
mod uptime;

struct RequestLogger {
    router: Router
}

impl iron::middleware::Handler for RequestLogger {
    fn handle(&self, r: &mut Request) -> IronResult<Response> {
        let resp = self.router.handle(r);
        match resp {
            Err(err) => {
                println!("ERROR; method: {}; url: {}", r.method, r.url);
                Err(err)
            },
            Ok(resp) => {
                let status = resp.status.unwrap();
                println!("OK; method: {}; url: {}, status: {}", r.method, r.url, status);
                Ok(resp)
            }
        }
    }
}

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

    router.get("/uptime", |_: &mut Request| -> IronResult<Response> {
        let uptime = uptime::uptime();
        match uptime {
            Err(err) => Ok(Response::with((Status::InternalServerError, err))),
            Ok(val) => Ok(Response::with((Status::Ok, json::encode(&val).unwrap())))
        }
    }, "uptime");

    router.any("/*", |_: &mut Request| -> IronResult<Response> {
        Ok(Response::with((Status::NotFound, "Page not found")))
    }, "default");

    let request_wrapper = RequestLogger { router: router };

    let listener = env::var("BIND_STR").map_err(|e| e.to_string())
        .and_then(|s| SocketAddr::from_str(s.as_str()).map_err(|e| e.to_string()))
        .and_then(|s| Iron::new(request_wrapper).http(s).map_err(|e| e.to_string()));


    match listener {
        Err(err) => println!("Error: {}", err),
        Ok(_) => { /* Router unwrapped */ }
    }
}
