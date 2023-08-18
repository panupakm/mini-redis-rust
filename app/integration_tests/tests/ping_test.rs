use client::client::Client;
use server::server::Server;
use std::thread::{self};
use std::time::Duration;
use pretty_assertions::assert_eq;


#[test]
fn test_connect_to_server() {
    const ADDRESS: &str = "127.0.0.1";
    const PORT: u16 = 7878;
    let redis_serve = Server::new(ADDRESS.to_string(), PORT);
    thread::spawn(move || {
        redis_serve.start();
    });

    thread::sleep(Duration::from_secs(1));

    let mut redis_client = Client::new();
    assert!(redis_client.connect(ADDRESS, PORT).is_ok());
    match redis_client.ping() {
        Ok(msg) => {
            println!("asdf {}", msg);
        }
        Err(e) => assert!(false, "{:?}", e),
    }
}
