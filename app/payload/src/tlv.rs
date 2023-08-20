pub mod string;
pub mod response;

pub enum ValueType {
    U8,
    String,
    Response,
    Unknown(u8),
}

impl  ValueType {
    pub fn to_u8(&self) -> u8 {
        match *self {
            ValueType::U8 => 0,
            ValueType::String => 1,
            ValueType::Response => 2,
            ValueType::Unknown(n) => n,
        }
    }

    pub fn from_u8(v: u8) -> ValueType {
        match v {
            0 => ValueType::U8,
            1 => ValueType::String,
            2 => ValueType::Response,
            _ => ValueType::Unknown(v),
        }
    }
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::Unknown(255)
    }
}
