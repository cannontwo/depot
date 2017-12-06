extern crate zmq;
extern crate protobuf;
extern crate hyper;

mod proto {
    pub mod depot;
}

use std::io::{self, Write, Read};

use protobuf::Message;
use protobuf::core::parse_from_bytes;

use hyper::Client;

use proto::depot;

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect("tcp://localhost:5558").is_ok());

    let register = context.socket(zmq::REQ).unwrap();
    assert!(register.connect("tcp://localhost:6001").is_ok());


    let client = Client::new();
    let res = client.get("http://ip.42.pl/raw").send();

    let mut ip_string = String::new();
    if let Ok(mut inner) = res {
        inner.read_to_string(&mut ip_string).unwrap();
    } else {
        ip_string = String::from("Could not find IP");
    }

    let mut init = depot::ServerInit::new();
    init.set_name(String::from("localhost"));
    init.set_ip(ip_string.clone());

    let init_bytes = init.write_to_bytes().unwrap();
    register.send(&init_bytes, 0).unwrap();

    let init_resp_bytes = register.recv_bytes(0).unwrap();
    let init_resp: depot::ServerInitResponse = parse_from_bytes(&init_resp_bytes).unwrap();
    println!("Assigned UUID {}", init_resp.get_server_uuid());

    loop {
        // TODO: Actually handle sent configs and send reports
        let string = receiver.recv_string(0).unwrap().unwrap();
        println!("{}.", string);
        let _ = io::stdout().flush();
        sender.send_str("", 0);
    }
}
