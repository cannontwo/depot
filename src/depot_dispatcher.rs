#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate time;
extern crate zmq;
extern crate iron;
extern crate router;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate params;
extern crate protobuf;
extern crate hyper;

use std::sync::Mutex;
use std::collections::HashMap;
use std::str::FromStr;

use std::thread;
use std::io::{self, BufRead, Write};
use std::ops::Deref;
use std::time::Instant;
use time::Duration;

use iron::prelude::*;
use iron::status;

use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use router::Router;

use params::{Params, Value};

use uuid::Uuid;

use protobuf::Message;
use protobuf::core::parse_from_bytes;

mod proto {
    pub mod depot;
}

use proto::depot;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    body: String,
    uuid: Uuid,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    name: String,
    ip: String,
    uuid: Uuid,
    config: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    done: bool,
    ep_num: u32,
    server: Option<Uuid>,
    rewards_link: Option<String>,
    test_rewards_link: Option<String>,
    test_successes_link: Option<String>,
}

// Struct representing a config
impl Config {
    fn new(name: String, body: String) -> Config {
        Config {
            name,
            body,
            uuid: Uuid::new_v4(),
            status: Status::new()
        }
    }
}

// Struct representing a server
impl Server {
    fn new(name: String, ip: String) -> Server {
        Server {
            name,
            ip,
            uuid: Uuid::new_v4(),
            config: None
        }
    }
}

// Struct representing the status of a config
impl Status {
    fn new() -> Status {
        Status {
            done: false,
            ep_num: 0,
            server: None,
            rewards_link: None,
            test_rewards_link: None,
            test_successes_link: None
        }
    }
}

lazy_static! {
    static ref CONFIGS: Mutex<HashMap<Uuid, Config>> = Mutex::new(HashMap::new());
    static ref SERVERS: Mutex<HashMap<Uuid, Server>> = Mutex::new(HashMap::new());
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok,
                       "Hello! Check out <a href=\"/configs\">configs</a> \
                       or <a href=\"/servers\">servers</a>"));
    resp.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
    Ok(resp)
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

// Handler for deleting configs
fn delete_config(req: &mut Request) -> IronResult<Response> {
    println!("Got delete request: {:?}", req.get_ref::<Params>());

    let inputs = match req.get_ref::<Params>() {
        Ok(val) => val,
        Err(_) => panic!("Couldn't get POST parameters")
    };

    let config_id_str = match inputs.get("config_id") {
        Some(&Value::String(ref val)) => val,
        _ => return Ok(Response::with(status::NotFound))
    };

    let config_id = match Uuid::from_str(config_id_str) {
        Ok(id) => id,
        _ => return Ok(Response::with(status::NotFound))
    };

    match CONFIGS.lock() {
        Ok(mut guard) => {
            let config = guard.remove(&config_id);
            match config {
                Some(_) => Ok(Response::with(status::Ok)),
                None => Ok(Response::with(status::NotFound))
            }
        },
        Err(_) => panic!("Couldn't acquire lock")
    }
}

// Handler returning a JSON object storing all known servers.
fn get_servers(req: &mut Request) -> IronResult<Response> {
    match SERVERS.lock() {
        Ok(guard) => Ok(Response::with((status::Ok, serde_json::to_string_pretty(&guard.deref()).unwrap()))),
        Err(_) => panic!("Couldn't lock SERVERS")
    }
}

// Handler returning a JSON object representing the requested server
fn get_server(req: &mut Request) -> IronResult<Response> {
    let ref server_id = match req.extensions.get::<Router>().unwrap().find("server") {
        Some(val) => match Uuid::from_str(val) {
            Ok(id) => id,
            _ => return Ok(Response::with(status::NotFound))
        },
        None => return Ok(Response::with(status::NotFound))
    };

    match SERVERS.lock() {
        Ok(guard) => {
            let server = match guard.get(server_id) {
                Some(val) => val,
                None => return Ok(Response::with(status::NotFound))
            };
            Ok(Response::with((status::Ok, serde_json::to_string_pretty(server).unwrap())))
        },
        Err(_) => panic!("Couldn't lock SERVERS")
    }
}

// Start the web API for the dispatcher.
fn start_web_server() {
    let mut router = Router::new();

    router.get("/", hello_world, "hello_world");

    router.get("/configs", get_configs, "get_configs");
    router.post("/config/upload", upload_config, "upload_config");
    router.post("/config/delete", delete_config, "delete_config");
    router.get("/config/:config", get_config, "get_config");

    router.get("/servers", get_servers, "get_servers");
    router.get("/server/:server", get_server, "get_server");

    Iron::new(router).http("localhost:3000").unwrap();
    println!("Server started at port 3000");
}

// Handles new server connections by inserting the server into SERVERS.
fn start_secretary() {
    let context = zmq::Context::new();

    let register = context.socket(zmq::REP).unwrap();
    assert!(register.bind("tcp://*:6001").is_ok());

    loop {
        let init_bytes = register.recv_bytes(0).unwrap();
        let init: depot::ServerInit = parse_from_bytes(&init_bytes).unwrap();

        let server = Server::new(String::from(init.get_name()),
                                 String::from(init.get_ip()));
        let id = server.uuid;

        if let Ok(mut guard) = SERVERS.lock() {
            guard.insert(id, server);
        }

        let mut init_resp = depot::ServerInitResponse::new();
        init_resp.set_server_uuid(id.to_string());
        let init_resp_bytes = init_resp.write_to_bytes().unwrap();
        register.send(&init_resp_bytes, 0).unwrap();
    }
}

// Collects reports from servers.
fn start_sink() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.bind("tcp://*:5558").is_ok());

    let _ = receiver.recv_string(0).unwrap();

    let start_time = Instant::now();

    for task_nbr in 0..100 {
        let report_bytes = receiver.recv_bytes(0).unwrap();
        let report: depot::ServerReport = parse_from_bytes(&report_bytes).unwrap();

        // TODO: Instead of just printing, actually process report.
        println!("{}: {:?}", task_nbr, report);
        let _ = io::stdout().flush();
    }

    println!("Total elapsed time: {:?} msec", Duration::from_std(start_time.elapsed())
        .unwrap().num_milliseconds());

    let control = context.socket(zmq::PUB).unwrap();
    assert!(control.bind("tcp://*:5559").is_ok());
    control.send_str("kill", 0).unwrap();
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

    thread::spawn(move || {
        start_sink();
    });

    thread::spawn(move || {
        start_secretary();
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
        // TODO: Actually send configs
        let string = format!("{}: {}", i, print_string);
        sender.send_str(&string, 0).unwrap();
    }
    thread::sleep(std::time::Duration::from_secs(1));

    println!("Done")
}
