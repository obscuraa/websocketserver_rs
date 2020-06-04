use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc;

use ws::{connect, listen, CloseCode, Sender, Handler, Message, Result};

struct Server {
    out: Sender, 
}

pub struct WebSocket {
    out: Sender,
}

impl WebSocket {
    pub fn send(&self, msg: u32) {
        self.out.send(format!("{}", msg)).unwrap();
    }
}


impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server recieved message {}", msg);
        self.out.send(msg)
    }   
}

pub fn run(tx_send: mpsc::Sender<WebSocket>) {
    let server = thread::spawn(move || {
        listen("127.0.0.1:3002", move |out| {
            println!("Web socket spawning...");
            let websocket = WebSocket{
                out: out.clone(),
            };
            tx_send.send(websocket).unwrap();
            Server {out}
        }).unwrap()
    });

    sleep(Duration::from_millis(10));
}
