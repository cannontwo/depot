#[macro_use]
extern crate lazy_static;

extern crate zmq;
extern crate protobuf;
extern crate hyper;
extern crate uuid;
extern crate yaml_rust;

mod proto {
    pub mod depot;
}

use std::io::{self, Write, Read};
use std::sync::Mutex;
use std::str;
use std::thread;

use protobuf::Message;
use protobuf::core::parse_from_bytes;

use hyper::Client;

use proto::depot;

use uuid::Uuid;

use yaml_rust::YamlLoader;

const HEARTBEAT_LIVENESS: u64 = 3;
const HEARTBEAT_INTERVAL: u64 = 1000;

lazy_static! {
    static ref CURRENT_EP_NUM: Mutex<u32> = Mutex::new(0);
    static ref CURRENT_MAX_EP_NUM: Mutex<u32> = Mutex::new(0);
    static ref CURRENT_CONFIG: Mutex<Option<depot::ServerConfig>> = Mutex::new(None);
}

fn make_slice<'a>(vector: &'a Vec<Vec<u8>>) -> Vec<&'a[u8]> {
    vector.iter().map(|x| x.as_ref() as &[u8]).collect::<Vec<&[u8]>>()
}

fn do_work(num_eps: u32) {
    for i in 0..num_eps {
        println!("Doing work episode {}", i);
        *CURRENT_EP_NUM.lock().unwrap() = i;
        thread::sleep_ms(10);
    }
}

fn send_report(socket: &zmq::Socket, identity: &Uuid) {
    let mut report_msg: Vec<Vec<u8>> = vec!();
    report_msg.push("".as_bytes().to_vec());
    let mut type_part = depot::TypeSignifier::new();
    type_part.set_field_type(depot::ServerMessageType::REPORT);
    report_msg.push(type_part.write_to_bytes().unwrap());

    // Make report
    let mut report_part = depot::ServerReport::new();
    let mut config_done = false;
    report_part.set_server_uuid(identity.to_string());
    report_part.set_ep_num(*CURRENT_EP_NUM.lock().expect("E: Couldn't lock ep_num"));
    if let Ok(mut current_config) = CURRENT_CONFIG.lock() {
        match *current_config {
            Some(ref config) => {
                report_part.set_has_config(true);
                report_part.set_config_uuid(config.get_uuid().to_string());
                if *CURRENT_MAX_EP_NUM.lock().unwrap() - 1 == *CURRENT_EP_NUM.lock().unwrap() {
                    report_part.set_done(true);
                    config_done = true;
                } else {
                    report_part.set_done(false);
                }
            },
            None => report_part.set_has_config(false),
        }
    } else {
        panic!("Couldn't lock current config");
    }

    if config_done {
        *CURRENT_CONFIG.lock().unwrap() = None;
        *CURRENT_EP_NUM.lock().unwrap() = 0;
    }

    report_msg.push(report_part.write_to_bytes().unwrap());
    let report_slice: &[&[u8]] = &make_slice(&report_msg);
    socket.send_multipart(report_slice, 0).unwrap();
}

fn main() {
    let context = zmq::Context::new();
    let identity = Uuid::new_v4();

    // Set up sockets
    let receiver = context.socket(zmq::DEALER).unwrap();
    receiver.set_identity(identity.as_bytes()).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    let statistics = context.socket(zmq::PUB).unwrap();
    statistics.set_identity(identity.as_bytes()).unwrap();
    assert!(statistics.connect("tcp://localhost:5558").is_ok());

    println!("I: Starting worker {} ({:?})", identity, identity.as_bytes());

    // Get IP address
    let client = Client::new();
    let res = client.get("http://ip.42.pl/raw").send();

    let mut ip_string = String::new();
    if let Ok(mut inner) = res {
        inner.read_to_string(&mut ip_string).unwrap();
    } else {
        ip_string = String::from("Could not find IP");
    }

    // Send init message to register with dispatcher
    let mut init_msg: Vec<Vec<u8>> = vec!();
    init_msg.push("".as_bytes().to_vec());
    let mut type_part = depot::TypeSignifier::new();
    type_part.set_field_type(depot::ServerMessageType::INIT);
    init_msg.push(type_part.write_to_bytes().unwrap());

    let mut init = depot::ServerInit::new();
    init.set_name(String::from("localhost"));
    init.set_ip(ip_string.clone());
    let init_bytes = init.write_to_bytes().unwrap();
    init_msg.push(init_bytes);
    let init_slice: &[&[u8]] = &(init_msg.iter().map(|x| x.as_ref() as &[u8]).collect::<Vec<&[u8]>>());
    receiver.send_multipart(init_slice, 0).unwrap();

    // Receive and process configs
    loop {
        let is_readable;
        {
            let mut items = [receiver.as_poll_item(zmq::POLLIN)];
            let rc = zmq::poll(&mut items, HEARTBEAT_INTERVAL as i64).unwrap();
            if rc == -1 {
                break;
            }

            is_readable = items[0].is_readable();
        }

        if is_readable {
            let msg = receiver.recv_multipart(0).unwrap();
            println!("D: received message {:?}", msg);
            let type_part: depot::TypeSignifier = parse_from_bytes(&msg[1]).unwrap();

            if type_part.get_field_type() != depot::ServerMessageType::CONFIG {
                println!("E: found unexpected message type {:?}", type_part.get_field_type());

                let mut report_msg: Vec<Vec<u8>> = vec!();
                report_msg.push("".as_bytes().to_vec());
                let mut type_part = depot::TypeSignifier::new();
                type_part.set_field_type(depot::ServerMessageType::REPORT);
                report_msg.push(type_part.write_to_bytes().unwrap());
                report_msg.push("".as_bytes().to_vec());
                let report_slice: &[&[u8]] = &make_slice(&report_msg);
                statistics.send_multipart(report_slice, 0).unwrap();

                return;
            }

            let config_msg: depot::ServerConfig = parse_from_bytes(&msg[2]).unwrap();
            if let Ok(mut current_config) = CURRENT_CONFIG.lock() {
                if current_config.is_none() {
                    *current_config = Some(config_msg.clone());
                    *CURRENT_EP_NUM.lock().unwrap() = 0;
                } else {
                    panic!("Got new config when already assigned config");
                }
            } else {
                panic!("Couldn't lock current config");
            }

            println!("D: Got config message with name {}", config_msg.get_name());
            let configs_maybe = YamlLoader::load_from_str(config_msg.get_body());
            if let Ok(configs) = configs_maybe {
                let config = &configs[0];
                println!("D: Parsed YAML config body: {:?}", config);

                if let Some(num_episodes) = config["experiment"]["num_episodes"].as_i64() {
                    let num_episodes = num_episodes as u32;
                    let ep_length = config["experiment"]["episode_length"].as_i64().unwrap();
                    println!("Got config with {} episodes of length {}", num_episodes, ep_length);
                    *CURRENT_MAX_EP_NUM.lock().unwrap() = num_episodes;
                    thread::spawn(move || do_work(num_episodes as u32));
                } else {
                    println!("E: Config body did not contain expected fields");
                    // TODO: Re-send init or something to signal readiness, instead of using worker_ready on heartbeats
                }
            } else {
                println!("E: Received message with non-YAML config body");
            }
        }

        send_report(&statistics, &identity);
    }
}
