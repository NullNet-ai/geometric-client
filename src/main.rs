mod shape;

use crate::shape::Shape;
use file_monitor::FileMonitor;
use geometric_server::GeometricGrpcInterface;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[tokio::main]
async fn main() {
    let mut client = GeometricGrpcInterface::new("localhost", 50052).await;

    let config_pair = Arc::new((Mutex::new(String::new()), Condvar::new()));
    let config_pair_2 = config_pair.clone();
    let file_monitor = FileMonitor::new(
        "../geometric-client/input/input.txt".to_string(),
        config_pair_2,
    );

    thread::spawn(move || {
        file_monitor.monitor();
    });

    loop {
        drop(config_pair.1.wait(config_pair.0.lock().unwrap()).unwrap());
        let file = config_pair.0.lock().unwrap().clone();
        let lines = file.lines();
        for line in lines {
            let Some(shape) = parse_line(line) else {
                panic!("Invalid operation: {line}");
            };
            shape.call_server(&mut client).await;
        }
    }
}

fn parse_line(line: &str) -> Option<Shape> {
    let (op, rest) = line.split_once(' ')?;
    match op {
        "square" => Some(Shape::Square(rest.parse::<f32>().ok()?)),
        "circle" => Some(Shape::Circle(rest.parse::<f32>().ok()?)),
        _ => None,
    }
}
