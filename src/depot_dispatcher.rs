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
use std::io::{self, Write};
use std::ops::Deref;
use std::time::Instant;
use std::str;

use iron::prelude::*;
use iron::status;

use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use router::Router;

use params::{Params, Value};

use uuid::Uuid;

use protobuf::core::parse_from_bytes;

mod proto {
    pub mod depot;
}

use proto::depot;

const HEARTBEAT_LIVENESS: u64 = 3;
const HEARTBEAT_INTERVAL: u64 = 10000;

#[derive(Serialize, Deserialize, Debug)]
enum ServerStatus {
    ONLINE = 0,
    OFFLINE = 1,
}

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
    status: ServerStatus,
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
    fn new(name: String, ip: String, uuid: Uuid) -> Server {
        Server {
            name,
            ip,
            uuid,
            config: None,
            status: ServerStatus::ONLINE,
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
    static ref EXPIRIES: Mutex<HashMap<Uuid, Instant>> = Mutex::new(HashMap::new());
    static ref READY_CONFIGS: Mutex<Vec<Uuid>> = Mutex::new(Vec::new());
    static ref READY_SERVERS: Mutex<Vec<Uuid>> = Mutex::new(Vec::new());
}

fn make_slice<'a>(vector: &'a Vec<Vec<u8>>) -> Vec<&'a[u8]> {
    vector.iter().map(|x| x.as_ref() as &[u8]).collect::<Vec<&[u8]>>()
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
            if let Ok(mut list_guard) = READY_CONFIGS.lock() {
                list_guard.push(config.uuid);
            }
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
fn get_servers(_: &mut Request) -> IronResult<Response> {
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

// Collects reports from servers.
fn start_sink() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::SUB).unwrap();
    assert!(receiver.connect("tcp://localhost:5558").is_ok());
    assert!(receiver.set_subscribe("".as_bytes()).is_ok());

    loop {
        let report_bytes = receiver.recv_bytes(0).unwrap();
        let report: depot::ServerReport = parse_from_bytes(&report_bytes).unwrap();

        // TODO: Instead of just printing, actually process report.
        println!("Received publication: {:?}", report);
        let _ = io::stdout().flush();
    }
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

// Function to purge expired workers
fn purge_workers() {
    if let Ok(expiry_guard) = EXPIRIES.lock() {
        if let Ok(mut list_guard) = READY_SERVERS.lock() {
            let now = std::time::Instant::now();

            // Update statuses
            for id in list_guard.iter() {
                if *expiry_guard.get(id).unwrap() < now {
                    if let Ok(mut server_guard) = SERVERS.lock() {
                        let entry = server_guard.get_mut(id).unwrap();
                        entry.status = ServerStatus::OFFLINE;
                    } else {
                        panic!("Couldn't lock SERVERS");
                    }
                }
            }

            // Purge from ready list
            list_guard.retain(|id| *expiry_guard.get(id).unwrap() > now);
        } else {
            panic!("Couldn't lock READY_SERVERS");
        }
    } else {
        panic!("Couldn't lock EXPIRIES");
    }
}

// Function called to initialize a worker and send response.
fn worker_ready(identity: &Uuid, init: Option<&depot::ServerInit>) {
    if let Ok(mut guard) = SERVERS.lock() {
        if !guard.contains_key(identity) {
            assert!(init.is_some());
            let init = init.unwrap();
            // We don't know about this server, so naively add to collections
            let server = Server::new(String::from(init.get_name()),
                                     String::from(init.get_ip()),
                                     identity.clone());
            guard.insert(identity.clone(), server);

            // Insert at the end of ready list
            if let Ok(mut list_guard) = READY_SERVERS.lock() {
                list_guard.push(identity.clone());
            } else {
                panic!("Couldn't lock READY_SERVERS");
            }

            // Create expiry time
            if let Ok(mut expiry_guard) = EXPIRIES.lock() {
                expiry_guard.insert(identity.clone(), std::time::Instant::now() +
                    std::time::Duration::from_millis(HEARTBEAT_INTERVAL));
            } else {
                panic!("Couldn't lock EXPIRIES");
            }
        } else {
            // Move this server to the end of the list.
            if let Ok(mut list_guard) = READY_SERVERS.lock() {
                list_guard.retain(|x| *x != *identity);
                list_guard.push(identity.clone())
            } else {
                panic!("Couldn't lock READY_SERVERS");
            }

            // Refresh timer
            if let Ok(mut expiry_guard) = EXPIRIES.lock() {
                let entry = expiry_guard.get_mut(identity).unwrap();
                *entry = std::time::Instant::now() +
                    std::time::Duration::from_millis(HEARTBEAT_INTERVAL);
            } else {
                panic!("Couldn't lock EXPIRIES");
            }
        }
    } else {
        panic!("Couldn't lock SERVERS");
    }
}

fn main() {
    thread::spawn(move || {
        start_web_server();
    });

    thread::spawn(move || {
        start_sink();
    });

    let hello_config_id = make_config("Hello", "Hello, world");
    if let Ok(mut guard) = READY_CONFIGS.lock() {
        guard.push(hello_config_id);
    }

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
    let dispatcher = context.socket(zmq::ROUTER).unwrap();
    assert!(dispatcher.bind("tcp://*:5557").is_ok());

    loop {
        let is_readable;
        {
            let mut items = [dispatcher.as_poll_item(zmq::POLLIN)];
            let rc = zmq::poll(&mut items, HEARTBEAT_INTERVAL as i64).unwrap();
            if rc == -1 {
                break;
            }

            is_readable = items[0].is_readable();
        }

        if is_readable {
            let msg = dispatcher.recv_multipart(0).unwrap();
            assert_eq!(msg.len(), 4);
            let identity = Uuid::from_bytes(&msg[0]).unwrap();
            println!("\tMessage received from worker {}", identity);
            let type_part: depot::TypeSignifier = parse_from_bytes(&msg[2]).unwrap();

            dispatcher.send_multipart(&make_slice(&msg), 0).unwrap();

            // Message should be identity | | TypeSignifier | content

            match type_part.get_field_type() {
                depot::ServerMessageType::INIT => {
                    let init: depot::ServerInit = parse_from_bytes(&msg[3]).unwrap();
                    let identity = Uuid::from_bytes(&msg[0]).unwrap();
                    worker_ready(&identity, Some(&init));
                },
                depot::ServerMessageType::REPORT => {
                    println!("Got heartbeat from {:?}", msg[0]);
                    worker_ready(&identity, None);
                },
                _ => {
                    println!("Received unexpected message type {:?}", type_part.get_field_type());
                    return;
                }
            }
        }

        // Purge dead servers
        println!("Purging");
        purge_workers();
    }
}
