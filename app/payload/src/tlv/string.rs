use std::io::{Result, BufWriter, Write, Read, Error, ErrorKind};

use super::ValueType;
#[derive(Default)]
pub struct TlvString(String);

impl TlvString {
    pub fn from_string(value: String) -> TlvString {
        TlvString(value)
    }

    pub fn new() -> TlvString {
        TlvString::default()
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let size = self.0.len() as u32;
        let buffer = vec![0; size as usize];
        let mut writer = BufWriter::new(buffer);

        let vtype = ValueType::String.to_u8();
        writer.write(&vtype.to_be_bytes())?;
        writer.write(&size.to_be_bytes())?;
        writer.write(self.0.as_bytes())?;

        Ok(writer.buffer().to_owned())
    }

    pub fn get_value_bytes(&self) -> &[u8]{
        self.0.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0.as_bytes()).unwrap()
    }

    pub fn write(&self, mut w: impl Write) {
        let bytes = self.as_bytes().unwrap();
        w.write_all(&bytes).unwrap();
    }

    pub fn read(&mut self, mut r: impl Read) -> Result<usize> {

        let mut buf = vec![0; 4];
        r.read_exact(&mut buf[..1])?;
        let vtype = u8::from_be_bytes(buf[..1].try_into().unwrap());
        if vtype != ValueType::String.to_u8() {
            return Err(Error::new(ErrorKind::Other, "Something went wrong"));
        }

        r.read_exact(&mut buf[..4])?;
        let size = u32::from_be_bytes(buf[0..4].try_into().unwrap());

        buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;

        self.0 = String::from_utf8(buf).unwrap();

        Ok(1 + 4 + size as usize)
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_as_bytes() {
        let s = TlvString::from_string("hello".to_string());
        assert_eq!(
            s.as_bytes().unwrap(),
            vec![1, 0, 0, 0, 5, 104, 101, 108, 108, 111]
        );
    }

    #[test]
    fn test_new() {
        let s = TlvString::from_string("hello".to_string());
        assert_eq!(s.0, "hello");
    }

    #[test]
    fn test_write() {
        let s = TlvString::from_string("hello".to_string());
        let mut buffer = Vec::new();
        s.write(&mut buffer);
        assert_eq!(buffer, vec![1, 0, 0, 0, 5, 104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_read() {
        let buf: Vec<u8> = vec![1, 0, 0, 0, 5, 104, 101, 108, 108, 111];
        let mut tlv_string = TlvString::from_string("".to_string());
        tlv_string.read(buf.as_slice()).unwrap();
        assert_eq!(tlv_string.as_bytes().unwrap(), vec![1, 0, 0, 0, 5, 104, 101, 108, 108, 111]);
    }
}
