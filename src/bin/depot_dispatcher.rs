#[macro_use]
extern crate lazy_static;
extern crate zmq;

use std::sync::Mutex;
use std::collections::HashMap;

use std::{thread, time};
use std::io::{self, BufRead};


lazy_static! {
    static ref CONFIGS: Mutex<HashMap<&'static str, &'static str>> = Mutex::new(HashMap::new());
}

fn main() {
    match CONFIGS.lock() {
        Ok(mut guard) => guard.insert("Hello", "Hello, world"),
        Err(_) => panic!("Couldn't lock CONFIGS")
    };

    let print_string = match CONFIGS.lock() {
        Ok(guard) => *(match guard.get("Hello") {
            Some(val) => val,
            None => panic!("Couldn't find 'Hello' key")
        }),
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