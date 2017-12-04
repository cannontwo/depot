extern crate zmq;

use std::io::{self, Write};

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect("tcp://localhost:5558").is_ok());

    loop {
        let string = receiver.recv_string(0).unwrap().unwrap();
        println!("{}.", string);
        let _ = io::stdout().flush();
        sender.send_str(&"", 0).unwrap();
    }
}
