use std::net::TcpListener;
use std::thread::{self};

pub struct Server {
    address: String,
    port: u16,
}

mod handler;

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        return Server {
            address: address,
            port: port,
        }
    }

    pub fn start(&self) {
        let listener = match TcpListener::bind(format!("{}:{}", self.address, self.port)) {
            Ok(listener) => {
                listener
            }
            Err(_e) => {
                panic!("failed to bind")
            }
        };
    
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Connected");
                    thread::spawn(move || { 
                        handler::handle_client(stream).unwrap();
                    }
                    );
                }
                Err(e) => {
                    panic!("failed to accept {}", e)
                }
            };
        }
        println!("Server closed");
    }
}