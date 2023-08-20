use std::{
    io::{BufWriter, Error, ErrorKind, Read, Result, Write},
    mem::size_of,
};

use super::ValueType;

#[derive(Default)]
pub struct TlvResponse {
    value_type: ValueType,
    value: Vec<u8>,
    error_message: Vec<u8>,
}

impl TlvResponse {
    pub fn from_string(value: String) -> Self {
        TlvResponse {
            value_type: ValueType::String,
            value: value.into_bytes(),
            error_message: Vec::new(),
        }
    }

    pub fn get_message_as_string(&self) -> String {
        String::from_utf8(self.value.to_owned()).unwrap()
    }

    pub fn from_bytes(value_type: ValueType, value: &Vec<u8>) -> Self {
        TlvResponse {
            value_type,
            value: value.to_owned(),
            error_message: Vec::new(),
        }
    }

    fn get_bytes_size(&self) -> u32 {
        (2 * size_of::<u8>() + 2 * size_of::<u32>() + self.value.len() + self.error_message.len())
            as u32
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let size = self.get_bytes_size();
        let buffer = vec![0; size as usize];
        let mut writer = BufWriter::new(buffer);

        writer.write(&ValueType::Response.to_u8().to_be_bytes())?;
        // Write value to response buffer
        writer.write(&self.value_type.to_u8().to_be_bytes())?;
        writer.write(&(self.value.len() as u32).to_be_bytes())?;
        writer.write(&self.value)?;

        // Write error message to response buffer
        writer.write(&(self.error_message.len() as u32).to_be_bytes())?;
        if self.error_message.len() > 0 {
            writer.write(&self.error_message)?;
        }

        Ok(writer.buffer().to_owned())
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }

    pub fn write(&self, mut w: impl Write) {
        let bytes = self.as_bytes().unwrap();
        w.write_all(&bytes).unwrap();
    }

    pub fn read(&mut self, mut r: impl Read) -> Result<usize> {
        let mut buf = vec![0; 4];
        r.read_exact(&mut buf[..1])?;
        let vtype = u8::from_be_bytes(buf[..1].try_into().unwrap());
        if vtype != ValueType::Response.to_u8() {
            return Err(Error::new(ErrorKind::Other, "Something went wrong"));
        }

        let mut total = 1;
        r.read_exact(&mut buf[..1])?;
        let vtype = u8::from_be_bytes(buf[..1].try_into().unwrap());
        match ValueType::from_u8(vtype) {
            ValueType::String => {
                self.value_type = ValueType::String;
                total += 1;
                r.read_exact(&mut buf[..4])?;
                total += 4;
                let size = u32::from_be_bytes(buf[..4].try_into().unwrap()) as usize;
                self.value = vec![0; size];
                total += size;
                r.read_exact(&mut self.value)?;
            }
            _ => {}
        }

        r.read_exact(&mut buf[..4])?;
        total += 4;
        let size = u32::from_be_bytes(buf[..4].try_into().unwrap()) as usize;
        total += size;
        if size > 0 {
            self.error_message = vec![0; size];
            r.read_exact(&mut self.error_message)?;
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_as_bytes() {
        let s = TlvResponse::from_string("hello".to_string());
        assert_eq!(
            s.as_bytes().unwrap(),
            vec![2, 1, 0, 0, 0, 5, 104, 101, 108, 108, 111, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_new() {
        let s = TlvResponse::from_string("hello".to_string());
        assert_eq!(std::str::from_utf8(&s.value).unwrap(), "hello");
    }

    #[test]
    fn test_write() {
        let s = TlvResponse::from_string("hello".to_string());
        let mut buffer = Vec::new();
        s.write(&mut buffer);
        assert_eq!(
            buffer,
            vec![2, 1, 0, 0, 0, 5, 104, 101, 108, 108, 111, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_read() {
        let buf: Vec<u8> = vec![2, 1, 0, 0, 0, 5, 104, 101, 108, 108, 111, 0, 0, 0, 0];
        let mut response = TlvResponse::from_string(String::new());
        assert_eq!(response.read(buf.as_slice()).unwrap(), buf.len());
        assert_eq!(
            response.as_bytes().unwrap(),
            vec![2, 1, 0, 0, 0, 5, 104, 101, 108, 108, 111, 0, 0, 0, 0]
        );
    }
}
