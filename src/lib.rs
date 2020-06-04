use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc;

pub mod websocket;

pub fn run() {
    let (tx_send, rx_recieve) = mpsc::channel();
    websocket::run(tx_send);
    let mut counter = 0;
    let socket = rx_recieve.recv().unwrap();

    loop{
        socket.send(counter);
        counter += 1;
        sleep(Duration::from_secs(3));
    };
}