use std::net::TcpStream;
use std::io::{Result, Write, Read, BufReader, BufRead};
use std::ops::RemAssign;

#[derive(Default)]
pub struct Client {
    stream: Option<TcpStream>,
}

impl Client {
    pub fn new() -> Self {
        return Client::default();
    }

    pub fn connect(&mut self, address: &str, port: u16) -> Result<()> {
        let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;        
        self.stream = Some(stream);
        return Ok(());
    }

    pub fn ping(&mut self) -> Result<String> {
        let msg = if let Some(ref mut stream) = self.stream {
            stream.write_all(b"PING\r\n")?;

            let mut buffer: Vec<u8> = Vec::with_capacity(512);

            let mut reader = BufReader::new(self.stream.as_ref().unwrap());
            reader.read_until(b'\n',&mut buffer)?;
            String::from_utf8(buffer).unwrap()
        } else {
            "".to_string()
        };

        return Ok(msg);
    }
}
