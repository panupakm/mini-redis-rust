use payload::tlv::ValueType;
use payload::tlv::response::TlvResponse;
use payload::tlv::string::TlvString;
use std::io::{self, Write, Read};


pub fn handle_ping<T: Write + Read>(mut stream: T) -> io::Result<()> {
    let mut pong = TlvString::from_string("".to_string());
    pong.read(&mut stream)?;

    let bytes = pong.get_value_bytes();
    let response = TlvResponse::from_bytes(ValueType::String, &bytes.to_vec());
    stream.write_all(&mut response.as_bytes().unwrap())?;
    Ok(())
}
