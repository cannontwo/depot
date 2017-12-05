#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate zmq;
extern crate iron;
extern crate router;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate params;

use std::sync::Mutex;
use std::collections::HashMap;
use std::str::FromStr;

use std::{thread, time};
use std::io::{self, BufRead};
use std::ops::Deref;

use iron::prelude::*;
use iron::status;

use router::Router;

use params::{Params, Value};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    body: String,
    uuid: Uuid
}

impl Config {
    fn new(name: String, body: String) -> Config {
        Config {
            name,
            body,
            uuid: Uuid::new_v4()
        }
    }
}

lazy_static! {
    static ref CONFIGS: Mutex<HashMap<Uuid, Config>> = Mutex::new(HashMap::new());
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

// Handler for listing all configs
fn get_configs(_: &mut Request) -> IronResult<Response> {
    match CONFIGS.lock() {
        Ok(guard) => Ok(Response::with((status::Ok, serde_json::to_string_pretty(&guard.deref()).unwrap()))),
        Err(_) => panic!("Couldn't lock CONFIGS")
    }
}

// Handler for displaying a single config
fn get_config(req: &mut Request) -> IronResult<Response> {
    let ref config_id = match req.extensions.get::<Router>().unwrap().find("config") {
        Some(val) => match Uuid::from_str(val) {
            Ok(id) => id,
            _ => return Ok(Response::with(status::NotFound))
        },
        None => return Ok(Response::with(status::NotFound))
    };

    match CONFIGS.lock() {
        Ok(guard) => {
            let config = match guard.get(config_id) {
                Some(val) => val,
                None => return Ok(Response::with(status::NotFound))
            };
            Ok(Response::with((status::Ok, serde_json::to_string_pretty(config).unwrap())))
        },
        Err(_) => panic!("Couldn't lock CONFIGS")
    }
}

// Handler for uploading configs
fn upload_config(req: &mut Request) -> IronResult<Response> {
    println!("Got upload request: {:?}", req.get_ref::<Params>());

    let inputs = match req.get_ref::<Params>() {
        Ok(val) => val,
        Err(_) => panic!("Couldn't get POST parameters")
    };

    let name = match inputs.get("name") {
        Some(&Value::String(ref val)) => val,
        _ => return Ok(Response::with(status::NotFound))
    };

    let body = match inputs.get("body") {
        Some(&Value::String(ref val)) => val,
        _ => return Ok(Response::with(status::NotFound))
    };

    match CONFIGS.lock() {
        Ok(mut guard) => {
            let config = Config::new(name.to_string(), body.to_string());
            guard.insert(config.uuid, config);
        },
        Err(_) => panic!("Couldn't acquire lock")
    }

    Ok(Response::with(status::Ok))
}

// Start the web frontend for the dispatcher.
fn start_web_server() {
    let mut router = Router::new();

    router.get("/", hello_world, "hello_world");
    router.get("/configs", get_configs, "get_configs");
    router.post("/config/upload", upload_config, "upload_config");
    router.get("/config/:config", get_config, "get_config");

    Iron::new(router).http("localhost:3000").unwrap();
    println!("Server started at port 3000");
}

// Make a config with the input name and body
fn make_config(name: &'static str, body: &'static str) -> Uuid {
    let config = Config::new(String::from(name), String::from(body));
    let id = config.uuid;
    match CONFIGS.lock() {
        Ok(mut guard) => guard.insert(id, config),
        Err(_) => panic!("Couldn't lock CONFIGS")
    };

    id
}

fn main() {
    thread::spawn(move || {
        start_web_server();
    });

    let hello_config_id = make_config("Hello", "Hello, world");

    let print_string = match CONFIGS.lock() {
        Ok(guard) => match guard.get(&hello_config_id) {
            Some(val) => format!("{:?}", val),
            None => panic!("Couldn't find 'Hello' key")
        },
        Err(_) => panic!("Couldn't lock CONFIGS")
    };
    println!("{}", print_string);

    // ZMQ Stuff
    let context = zmq::Context::new();
    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.bind("tcp://*:5557").is_ok());

    let sink = context.socket(zmq::PUSH).unwrap();
    assert!(sink.connect("tcp://localhost:5558").is_ok());

    println!("Press Enter when the workers are ready: ");
    let stdin = io::stdin();
    stdin.lock().lines().next();
    println!("Sending tasks to workers...");

    sink.send_str("0", 0).unwrap();

    for i in 0..100 {
        let string = format!("{}: {}", i, print_string);
        sender.send_str(&string, 0).unwrap();
    }
    thread::sleep(time::Duration::from_secs(1));

    println!("Done")
}
