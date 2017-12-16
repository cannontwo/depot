extern crate zmq;
extern crate protobuf;
extern crate hyper;
extern crate uuid;

mod proto {
    pub mod depot;
}

use std::io::{self, Write, Read};
use std::str;

use protobuf::Message;
use protobuf::core::parse_from_bytes;

use hyper::Client;

use proto::depot;

use uuid::Uuid;

const HEARTBEAT_LIVENESS: u64 = 3;
const HEARTBEAT_INTERVAL: u64 = 1000;

fn make_slice<'a>(vector: &'a Vec<Vec<u8>>) -> Vec<&'a[u8]> {
    vector.iter().map(|x| x.as_ref() as &[u8]).collect::<Vec<&[u8]>>()
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
    assert!(statistics.bind("tcp://*:5558").is_ok());

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
        // TODO: Actually handle sent configs and send reports
        // TODO: Poll instead of blocking
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
            receiver.send_multipart(report_slice, 0).unwrap();

            return;
        }

        let string = str::from_utf8(&msg[2]).unwrap();
        println!("{}.", string);
        let _ = io::stdout().flush();
        statistics.send_str("", 0).unwrap();
    }
}
