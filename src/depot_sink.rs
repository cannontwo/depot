extern crate zmq;
extern crate time;

use std::io::{self, Write};
use std::time::Instant;
use time::Duration;

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.bind("tcp://*:5558").is_ok());

    let _ = receiver.recv_string(0).unwrap();

    let start_time = Instant::now();

    for task_nbr in 0..100 {
        let print_string = receiver.recv_string(0).unwrap().unwrap();
        println!("{}: {}", task_nbr, print_string);
        let _ = io::stdout().flush();
    }

    println!("Total elapsed time: {:?} msec", Duration::from_std(start_time.elapsed())
        .unwrap().num_milliseconds());

    let control = context.socket(zmq::PUB).unwrap();
    assert!(control.bind("tcp://*:5559").is_ok());
    control.send_str("kill", 0).unwrap();
}