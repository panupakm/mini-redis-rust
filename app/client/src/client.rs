use payload::request::PING;
use payload::tlv::response::TlvResponse;
use payload::tlv::string::TlvString;
use std::io::{Result, Write};
use std::net::TcpStream;

#[derive(Default)]
pub struct Client {
    stream: Option<TcpStream>,
}

impl Client {
    pub fn new() -> Self {
        return Client::default();
    }

    pub fn connect(&mut self, address: &str, port: u16) -> Result<()> {
        let stream = TcpStream::connect(format!("{}:{}", address, port))?;
        self.stream = Some(stream);
        return Ok(());
    }

    pub fn ping(&mut self, msg: &str) -> Result<String> {
        let pong = if let Some(ref mut stream) = self.stream {
            let tlv_ping = TlvString::from_string(PING.to_string());
            stream.write_all(&tlv_ping.as_bytes().unwrap())?;

            let tlv_msg = TlvString::from_string(msg.to_string());
            stream.write_all(&tlv_msg.as_bytes().unwrap())?;

            let mut tlv_pong = TlvResponse::default();
            tlv_pong.read(stream)?;
            tlv_pong
        } else {
            TlvResponse::default()
        };

        return Ok(pong.get_message_as_string());
    }
}
