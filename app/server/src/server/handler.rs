use std::io::{self, Write};
use std::net::TcpStream;
use payload::tlv::string::TlvString;
use payload::request::PING;

pub fn handle_client(stream: TcpStream) -> io::Result<()> {
    // let mut buf = [0; 1024];
    loop {

        let mut command = TlvString::new("".to_string());
        command.read(&stream)?;

        match command.as_str() {
            PING => handle_ping(&stream),
            _ => Ok({}), 
        }?;

        // let bytes_read = stream.read(&mut buf)?;
        // if bytes_read == 0 {
        //     return Ok(());
        // }
        // stream.write(&buf[0..bytes_read])?;
    }
}

fn handle_ping(mut stream: &TcpStream) -> io::Result<()> {
    let mut pong = TlvString::new("".to_string());
    pong.read(&mut stream)?;

    let bytes = pong.as_bytes().unwrap();
    stream.write_all(&bytes)?;
    Ok(())
}