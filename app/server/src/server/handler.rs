use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[0..bytes_read])?;
    }
}
