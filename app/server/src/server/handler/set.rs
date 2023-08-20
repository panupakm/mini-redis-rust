use super::HandlerReadWrite;
use payload::tlv::response::TlvResponse;
use payload::tlv::string::TlvString;
use std::io::{self, Read, Result, Write};

pub fn handle_set<T: Read + Write>(mut stream: T) -> Result<()> {
    let mut key = TlvString::new();
    key.read(&mut stream)?;

    let mut value = TlvString::new();
    value.read(&mut stream)?;

    let response = TlvResponse::from_string(String::from("OK"));
    stream.write_all(&mut response.as_bytes().unwrap())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        ReadWrite {}
        impl Read for ReadWrite {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
        }
        impl Write for ReadWrite {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;
        }
    }

    #[test]
    fn test_handle_set() {
        let mut mock = MockReadWrite::new();
        let mut data_vec = vec![
            vec![1],
            vec![0, 0, 0, 3],
            b"key".to_vec(),
            vec![1],
            vec![0, 0, 0, 5],
            b"value".to_vec(),
        ];
        let size = data_vec.len();
        mock.expect_write().returning(move |_| Ok(size));
        mock.expect_read().returning(move |buf| {
            let next = data_vec.remove(0);
            buf.copy_from_slice(&next);
            return Ok(next.len());
        });

        let result = handle_set(mock);
        assert!(result.is_ok());
    }
}
