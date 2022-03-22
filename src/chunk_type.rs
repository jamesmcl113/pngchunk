use std::fmt;
use std::str::FromStr;

use crate::Error;
use crate::Result;

#[derive(Debug)]
pub struct ChunkType {
    data: [u8; 4],
}

impl ChunkType {
    pub fn new(data: [u8; 4]) -> Self {
        ChunkType { data }
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }

    pub fn is_valid(&self) -> bool {
        self.data.iter().all(|x| x.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        ChunkType::get_bit_at(self.data[0], 5).unwrap() == false
    }

    pub fn is_public(&self) -> bool {
        ChunkType::get_bit_at(self.data[1], 5).unwrap() == false
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        ChunkType::get_bit_at(self.data[2], 5).unwrap() == false
    }

    pub fn is_safe_to_copy(&self) -> bool {
        ChunkType::get_bit_at(self.data[3], 5).unwrap() == true
    }

    fn get_bit_at(byte: u8, n: u8) -> Result<bool> {
        if n < 8 {
            Ok(byte & (1 << n) != 0)
        } else {
            Err("n cannot be greater than 7".into())
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        Ok(Self::new(value))
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("String must be 4 bytes long.".into());
        }

        let bytes: Vec<u8> = s.bytes().collect();

        let chunk: ChunkType = ChunkType::new([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if chunk.data.iter().all(|x| x.is_ascii_alphabetic()) {
            Ok(chunk)
        } else {
            Err("Chunk type can only contain alphabetic ascii".into())
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data))
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter().zip(other.data.iter()).all(|(a, b)| a == b)
    }
}

impl Eq for ChunkType {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
