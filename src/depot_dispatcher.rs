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
use std::ops::Deref;
use std::time::Instant;
use std::str;

use iron::prelude::*;
use iron::modifiers::Redirect;
use iron::{Iron, Request, Response, IronResult, Chain, Url, status};
use iron::AfterMiddleware;

use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use router::Router;

use params::{Params, Value};

use uuid::Uuid;

use protobuf::core::parse_from_bytes;
use protobuf::Message;

mod proto {
    pub mod depot;
}

use proto::depot;

const HEARTBEAT_LIVENESS: u64 = 3;
const HEARTBEAT_INTERVAL: u64 = 1000;

const DEFAULT_BODY_STRING: &str = "agent:
    discount_factor: 0.98
    buffer_size: 1000000
    batch_size: 64
    num_motion_planned: 0
    num_demonstrations: 0
    num_joints: 6
    exploration_rate: 0.01
    tau: 0.05
    actor_learning_rate: 0.0001
    critic_learning_rate: 0.001
    use_random_goal: True
    planning_group: manipulator
    critic_hidden_layers:
    actor_hidden_layers:
        - 300

experiment:
    name: default
    computer_name: unspecified
    num_episodes: 5000
    episode_length: 100
    slack_webhook: https://hooks.slack.com/services/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
    num_tests: 100
    test_frequency: 50
";

struct CorsMiddleware;

impl AfterMiddleware for CorsMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(hyper::header::AccessControlAllowOrigin::Any);
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

// Struct describing the expiration conditions for a server
struct Expiry {
    instant: Instant,
    liveness: u64,
}

impl Expiry {
    fn new() -> Expiry {
        Expiry {
            instant: Instant::now() + std::time::Duration::from_millis(HEARTBEAT_INTERVAL),
            liveness: HEARTBEAT_LIVENESS,
        }
    }
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
    static ref EXPIRIES: Mutex<HashMap<Uuid, Expiry>> = Mutex::new(HashMap::new());
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
    loop {
        let lock = CONFIGS.try_lock();
        if let Ok(guard) = lock {
            return Ok(Response::with((status::Ok, serde_json::to_string_pretty(&guard.deref()).unwrap())))
        }
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

    loop {
        let lock = CONFIGS.try_lock();
        if let Ok(guard) = lock {
            let config = match guard.get(config_id) {
                Some(val) => val,
                None => return Ok(Response::with(status::NotFound))
            };

            return Ok(Response::with((status::Ok, serde_json::to_string_pretty(config).unwrap())))
        }
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

    // Prevent deadlock using try_lock
    loop {
        let lock = CONFIGS.try_lock();
        if let Ok(mut guard) = lock {
            loop {
                let list_lock = READY_CONFIGS.try_lock();
                if let Ok(mut list_guard) = list_lock {
                    let config = Config::new(name.to_string(), body.to_string());
                    list_guard.insert(0, config.uuid);
                    guard.insert(config.uuid, config);

                    let url = Url::parse("http://localhost:3000").unwrap();
                    return Ok(Response::with((status::Found, Redirect(url.clone()))));
                } else {
                    break;
                }
            }
        }
    }
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

    loop {
        let lock = CONFIGS.try_lock();
        if let Ok(mut guard) = lock {
            let config = guard.remove(&config_id);
            match config {
                Some(_) => return Ok(Response::with(status::Ok)),
                None => return Ok(Response::with(status::NotFound))
            }
        }
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

    let mut chain = Chain::new(router);
    let cors_middleware = CorsMiddleware {};
    chain.link_after(cors_middleware);

    Iron::new(chain).http("localhost:4000").unwrap();
    println!("Server started at port 4000");
}

// Collects reports from servers.
fn start_sink() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::SUB).unwrap();
    assert!(receiver.bind("tcp://*:5558").is_ok());
    assert!(receiver.set_subscribe("".as_bytes()).is_ok());

    loop {
        let msg = receiver.recv_multipart(0).unwrap();
        println!("D: Sink received msg: {:?}", msg);
        assert_eq!(msg.len(), 3);
        let type_part: depot::TypeSignifier = parse_from_bytes(&msg[1]).unwrap();

        // Message should be identity | | TypeSignifier | content

        match type_part.get_field_type() {
            depot::ServerMessageType::REPORT => {
                let report: depot::ServerReport = parse_from_bytes(&msg[2]).unwrap();
                let identity = Uuid::from_str(report.get_server_uuid()).unwrap();

                println!("D: Got report from worker {:?}", identity);
                println!("D: Worker {:?} is on ep_num {}", identity, report.get_ep_num());

                if report.get_has_config() {
                    // Refresh timer, but don't add to ready list
                    worker_live(&identity, false);

                    let config_uuid = Uuid::from_str(report.get_config_uuid()).unwrap();
                    println!("D: Worker {:?} has config {:?}", identity, config_uuid);

                    // Update server
                    if let Ok(mut servers_guard) = SERVERS.lock() {
                        if let Some(server) = servers_guard.get_mut(&identity) {
                            server.config = Some(config_uuid);
                        }
                    }

                    // Update config
                    if let Ok(mut configs_guard) = CONFIGS.lock() {
                        if let Some(server_config) = configs_guard.get_mut(&config_uuid) {
                            let status: &mut Status = &mut server_config.status;
                            status.server = Some(identity.clone());
                            status.ep_num = report.get_ep_num();
                            status.done = report.get_done();
                        }
                    }
                } else {
                    println!("D: Worker {:?} has no config", identity);

                    // Refresh timer and add to ready list
                    worker_live(&identity, true);

                    // Update server
                    if let Ok(mut servers_guard) = SERVERS.lock() {
                        if let Some(server) = servers_guard.get_mut(&identity) {
                            server.config = None;
                        }
                    }
                }
            },
            _ => {
                println!("D: Sink Received unexpected message type {:?}", type_part.get_field_type());
            }
        }
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
    // Deadlock prevented in other threads, not considered in main
    if let Ok(mut expiry_guard) = EXPIRIES.lock() {
        if let Ok(mut server_guard) = SERVERS.lock() {
            let now = std::time::Instant::now();

            // Update statuses
            for (id, server) in server_guard.iter_mut() {
                let expiry: &mut Expiry = expiry_guard.get_mut(&id).unwrap();
                if expiry.instant < now && server.status != ServerStatus::OFFLINE {
                    if expiry.liveness == 0 {
                        println!("D: Discarding worker {}", id);
                        server.status = ServerStatus::OFFLINE;

                        if let Some(config_uuid) = server.config {
                            if let Ok(mut config_guard) = CONFIGS.lock() {
                                if let Some(config) = config_guard.get_mut(&config_uuid) {
                                    // If server doing config went down and config was not done, reclaim
                                    if !config.status.done {
                                        // Reset config status
                                        config.status = Status::new();
                                        if let Ok(mut config_list_guard) = READY_CONFIGS.lock() {
                                            config_list_guard.push(config.uuid.clone())
                                        }
                                        println!("D: Reclaimed config {:?}", config.uuid);
                                    }
                                }
                            } else {
                                panic!("Couldn't lock CONFIGS");
                            }
                        }
                    } else {
                        println!("I: Reducing liveness of worker {}", id);
                        expiry.liveness -= 1;
                        expiry.instant = now + std::time::Duration::from_millis(HEARTBEAT_INTERVAL);
                    }
                }
            }
            if let Ok(mut list_guard) = READY_SERVERS.lock() {
                // Purge from ready list
                list_guard.retain(|id| {
                    let expiry = expiry_guard.get(id).unwrap();
                    expiry.instant > now || expiry.liveness != 0
                });
            } else {
                panic!("Couldn't lock READY_SERVERS");
            }
        } else {
            panic!("Couldn't lock SERVERS");
        }
    } else {
        panic!("Couldn't lock EXPIRIES");
    }
}

fn send_configs(socket: &zmq::Socket) {
    if let Ok(mut config_guard) = READY_CONFIGS.lock() {
        if config_guard.len() == 0 {
            println!("D: No configs to send");
            return;
        }

        if let Ok(mut server_guard) = READY_SERVERS.lock() {
            if config_guard.len() == 0 {
                println!("D: No ready servers");
                return;
            }

            if let Ok(cmap_guard) = CONFIGS.lock() {
                if let Ok(smap_guard) = SERVERS.lock() {
                    // Loop and send configs to ready servers
                    loop {
                        if let Some(config_uuid) = config_guard.pop() {
                            if let Some(server_uuid) = server_guard.pop() {
                                let mut config_msg: Vec<Vec<u8>> = vec!();
                                if let Some(config) = cmap_guard.get(&config_uuid) {
                                    let server: &Server = smap_guard.get(&server_uuid).unwrap();

                                    // TODO: Update server, config information and test in sink

                                    // Set identity
                                    config_msg.push(server.uuid.as_bytes().to_vec());

                                    // Blank space
                                    config_msg.push("".as_bytes().to_vec());

                                    // Set message type
                                    let mut type_part = depot::TypeSignifier::new();
                                    type_part.set_field_type(depot::ServerMessageType::CONFIG);
                                    config_msg.push(type_part.write_to_bytes().unwrap());

                                    // Set config part
                                    let mut config_part = depot::ServerConfig::new();
                                    config_part.set_name(config.name.clone());
                                    config_part.set_body(config.body.clone());
                                    config_part.set_uuid(config.uuid.to_string());
                                    config_msg.push(config_part.write_to_bytes().unwrap());

                                    let config_slice: &[&[u8]] = &make_slice(&config_msg);
                                    socket.send_multipart(config_slice, 0).unwrap();
                                } else {
                                    println!("Found config in ready list which has been deleted, skipping");
                                    continue;
                                }
                            } else {
                                println!("D: Out of ready servers");
                                config_guard.push(config_uuid);
                                break;
                            }
                        } else {
                            println!("D: Out of configs");
                            break;
                        }
                    }
                    println!("D: Done sending configs");
                } else {
                    panic!("Couldn't lock SERVERS");
                }
            } else {
                panic!("Couldn't lock CONFIGS");
            }
        } else {
            panic!("Couldn't lock READY_SERVERS");
        }
    } else {
        panic!("Couldn't lock READY_CONFIGS");
    }
}

// Function called to refresh liveness of a worker.
fn worker_live(identity: &Uuid, add: bool) {
    // Move this server to the end of the list.
    if let Ok(mut list_guard) = READY_SERVERS.lock() {
        list_guard.retain(|x| *x != *identity);
        if add {
            list_guard.push(identity.clone());
        }
    } else {
        panic!("Couldn't lock READY_SERVERS");
    }

    // Refresh timer
    if let Ok(mut expiry_guard) = EXPIRIES.lock() {
        let entry: &mut Expiry = expiry_guard.get_mut(identity).unwrap();
        entry.instant = std::time::Instant::now() +
            std::time::Duration::from_millis(HEARTBEAT_INTERVAL);
        entry.liveness = HEARTBEAT_LIVENESS;
    } else {
        panic!("Couldn't lock EXPIRIES");
    }
}

// Function called to initialize a worker and send response.
fn worker_ready(identity: &Uuid, init: Option<&depot::ServerInit>) {
    // Deadlock prevented in other threads, this function should only be called from main
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
                let new_expiry = Expiry::new();
                expiry_guard.insert(identity.clone(), new_expiry);
            } else {
                panic!("Couldn't lock EXPIRIES");
            }
        } else {
            worker_live(identity, true);
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

    let hello_config_id = make_config("Hello", DEFAULT_BODY_STRING);
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
            println!("D: Message received from worker {}", identity);
            let type_part: depot::TypeSignifier = parse_from_bytes(&msg[2]).unwrap();

            // Message should be identity | | TypeSignifier | content

            match type_part.get_field_type() {
                depot::ServerMessageType::INIT => {
                    let init: depot::ServerInit = parse_from_bytes(&msg[3]).unwrap();
                    let identity = Uuid::from_bytes(&msg[0]).unwrap();
                    worker_ready(&identity, Some(&init));
                },
                depot::ServerMessageType::REPORT => {
                    println!("D: Got report from {:?}", msg[0]);
                    worker_ready(&identity, None);
                },
                _ => {
                    println!("W: Received unexpected message type {:?}", type_part.get_field_type());
                    return;
                }
            }
        }

        // Purge dead servers
        println!("D: Purging");
        purge_workers();

        // Send config if available
        println!("D: Sending configs");
        send_configs(&dispatcher);
    }
}
