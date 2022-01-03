use std::fmt::Display;

use crate::chunk_type::{self, ChunkType};
use crate::error::Error;
use crc::Crc;

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc_ck = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        Self {
            chunk_type: chunk_type.clone(),
            data: data.clone(),
            crc: crc_ck.checksum(
                chunk_type
                    .bytes()
                    .iter()
                    .chain(data.iter())
                    .cloned()
                    .collect::<Vec<u8>>()
                    .as_slice(),
            ),
        }
    }
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, Error> {
        match std::str::from_utf8(self.data.as_slice()) {
            Ok(data) => Ok(data.to_owned()),
            Err(_) => Err(Error::DataNotUTF8),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        u32::to_be_bytes(self.length())
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data().iter())
            .chain(u32::to_be_bytes(self.crc()).iter())
            .copied()
            .collect()
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk {{\n\tLength: {}\n\tType: {}\n\tData: {} bytes\n\tCRC: {}\n}}",
            self.length(),
            self.chunk_type(),
            self.data().len(),
            self.crc()
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(Error::ChunkTooSmall);
        }
        let mut buf_bytes: [u8; 4] = Default::default();
        buf_bytes.copy_from_slice(&bytes[0..4]);
        let length = u32::from_be_bytes(buf_bytes);
        buf_bytes.copy_from_slice(&bytes[4..8]);
        let chunk_type = ChunkType::try_from(buf_bytes)?;
        if (bytes.len() as u32) < length + 12 {
            return Err(Error::InvalidChunkSize {
                bytes_recv: bytes.len(),
                length_field: length,
            });
        }
        let data_end_offset: usize = 8 + length as usize;
        let data = bytes[8..data_end_offset].to_vec();
        buf_bytes.copy_from_slice(&bytes[data_end_offset..(data_end_offset + 4)]);
        let crc = u32::from_be_bytes(buf_bytes);
        let crc_ck = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        if crc != crc_ck.checksum(&bytes[4..data_end_offset]) {
            return Err(Error::InvalidCRC);
        }
        Ok(Self {
            chunk_type,
            data,
            crc,
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
