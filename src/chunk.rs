use core::fmt;

use crate::{chunk_type::ChunkType, Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug)]
pub struct Chunk {
    m_length: u32,
    m_type: ChunkType,
    m_chunk_data: Vec<u8>,
    m_crc: u32,
}

impl Chunk {
    pub const MIN_CHUNK_LENGTH: usize = 12;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let m_length = data.len() as u32;

        let combined = [&chunk_type.bytes()[..], &data[..]].concat();
        let m_crc = Chunk::calculate_crc(combined);

        Self {
            m_length,
            m_type: chunk_type,
            m_chunk_data: data,
            m_crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.m_length
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.m_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.m_chunk_data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.m_crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        if let Ok(s) = String::from_utf8(self.m_chunk_data.clone()) {
            return Ok(s);
        }
        Err("String is not valid utf-8.".into())
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self
            .m_length
            .to_be_bytes()
            .into_iter()
            .chain(self.m_type.bytes().into_iter())
            .chain(self.data().iter().cloned())
            .chain(self.m_crc.to_be_bytes().into_iter())
            .collect();

        bytes
    }

    fn calculate_crc(bytes: Vec<u8>) -> u32 {
        let crc: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        digest.update(&bytes);
        digest.finalize()
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < Chunk::MIN_CHUNK_LENGTH {
            return Err("Chunk must contain atleast 12 bytes.".into());
        }

        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&value[0..4]);
        let m_length = u32::from_be_bytes(buf);

        buf.fill(0);
        buf.copy_from_slice(&value[4..8]);
        let m_type = ChunkType::try_from(buf)?;

        let m_chunk_data: Vec<u8> = match value.len() {
            Chunk::MIN_CHUNK_LENGTH => vec![], // empty chunk data field
            _ => value[8..value.len() - 4].into_iter().cloned().collect(),
        };

        let m_crc = Chunk::calculate_crc([&m_type.bytes()[..], &m_chunk_data].concat());

        let crc_to_test = &value[8 + m_chunk_data.len()..];
        if crc_to_test.len() != 4 {
            panic!(
                "Incorrect number of bytes left in value: Got {}",
                crc_to_test.len()
            );
        }

        buf.fill(0);
        buf.copy_from_slice(crc_to_test);
        let crc_to_test = u32::from_be_bytes(buf);

        if crc_to_test != m_crc {
            return Err(format!("CRC invalid: Got {}, should be {}", crc_to_test, m_crc).into());
        }

        Ok(Chunk {
            m_length,
            m_type,
            m_chunk_data,
            m_crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
