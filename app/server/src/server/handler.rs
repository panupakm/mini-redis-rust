mod set;
pub use set::handle_set;

mod ping;
pub use ping::handle_ping;

use mockall::*;
use payload::request::{PING, SET};
use payload::tlv::string::TlvString;
use std::io::{self, Result, Read, Write};
use std::net::TcpStream;

pub fn handle_client(stream: TcpStream) -> io::Result<()> {
    loop {
        let mut command = TlvString::from_string("".to_string());
        command.read(&stream)?;

        match command.as_str() {
            PING => handle_ping(&stream),
            SET => handle_set(&stream),
            _ => Ok({}),
        }?;
    }
}

#[automock]
pub trait HandlerReadWrite {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

impl HandlerReadWrite for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Read::read(self, buf)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Write::write(self, buf)
    }
}
