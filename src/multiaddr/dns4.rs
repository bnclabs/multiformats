use crate::{
    multicodec::{self, Multicodec},
    Error, Result,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Dns4 {
    addr: Vec<u8>,
}

impl Dns4 {
    pub(crate) fn from_text<'a, 'b>(parts: &'a [&'b str]) -> Result<(Self, &'a [&'b str])> {
        let val = match parts {
            [addr, tail @ ..] => {
                let addr = addr.as_bytes().to_vec();
                (Dns4 { addr }, tail)
            }
            _ => err_at!(BadAddr, msg: "dns4 {:?}", parts)?,
        };

        Ok(val)
    }

    pub(crate) fn to_text(&self) -> Result<String> {
        use std::str::from_utf8;
        Ok("/dns4".to_string() + &err_at!(DecodeError, from_utf8(&self.addr))?)
    }

    pub(crate) fn decode(data: &[u8]) -> Result<(Self, &[u8])> {
        use unsigned_varint::decode::u128 as uv_decode;

        let val = {
            let (addr, data) = {
                let (n, data) = err_at!(DecodeError, uv_decode(data))?;
                let (name, data) = read_slice!(data, (n as usize), "dns4")?;
                (name.to_vec(), data)
            };

            let val = Dns4 { addr };

            (val, data)
        };

        Ok(val)
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>> {
        use unsigned_varint::encode::u128 as uv_encode;

        let mut buf = [0_u8; 19];

        let mut data = Multicodec::from_code(multicodec::DNS4)?.encode()?;
        data.extend_from_slice(uv_encode(self.addr.len() as u128, &mut buf));
        data.extend_from_slice(&self.addr);
        Ok(data)
    }

    pub fn as_str(&self) -> Result<&str> {
        use std::str::from_utf8;
        err_at!(DecodeError, from_utf8(&self.addr))
    }
}
