use std::net::TcpStream;
use std::io::{Result, Write, BufReader, BufRead};
use payload::request::PING;
use payload::tlv::string::TlvString;

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

            let tlv_ping = TlvString::new(PING.to_string());
            stream.write_all(&tlv_ping.as_bytes().unwrap())?;
            
            let tlv_msg = TlvString::new(msg.to_string());
            stream.write_all(&tlv_msg.as_bytes().unwrap())?;

            //let mut buffer: Vec<u8> = Vec::with_capacity(512);
            // let mut reader = BufReader::new(self.stream.as_ref().unwrap());

            let mut tlv_pong = TlvString::new("".to_string());
            tlv_pong.read(stream)?;

            tlv_pong.as_str().to_string()
        } else {
            "".to_string()
        };

        return Ok(pong.to_string());
    }
}
