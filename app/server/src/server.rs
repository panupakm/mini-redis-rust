use std::net::TcpListener;
use std::thread;

pub struct Server {
    address: String,
    port: u16,
}

mod handler;

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        return Server { address, port };
    }

    pub fn start(&self) {
        let listener = match TcpListener::bind(format!("{}:{}", self.address, self.port)) {
            Ok(listener) => listener,
            Err(_e) => {
                panic!("failed to bind")
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Connected");
                    thread::spawn(move || {
                        if let Err(e) = handler::handle_client(stream) {
                            println!("Error occurred: {}", e);
                        }
                    });
                }
                Err(e) => {
                    panic!("failed to accept {}", e)
                }
            };
        }
        println!("Server closed");
    }
}
