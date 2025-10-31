use crate::chunk_type::ChunkType;
use std::{
    error::Error,
    fmt,
    io::{BufReader, Read},
};

/// Represents a PNG chunk with length, type, data, and CRC.
#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    /// Creates a new Chunk with the given type and data.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data
            .len()
            .try_into()
            .expect("chunk too large, length needs to fit in u32");

        let crc_algo = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut digest = crc_algo.digest();
        digest.update(&chunk_type.bytes());
        digest.update(data.as_slice());
        let crc = digest.finalize();

        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    /// Returns the length of the chunk data.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Returns the chunk type.
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Returns the chunk data as a byte slice.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the CRC checksum.
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Attempts to interpret the data as a UTF-8 string.
    pub fn data_as_string(&self) -> crate::Result<String> {
        Ok(str::from_utf8(&self.data)?.to_owned())
    }

    /// Returns the chunk as a byte vector.
    pub fn as_bytes(&self) -> Vec<u8> {
        let length_bytes: [u8; 4] = self.length.to_be_bytes();
        let crc_bytes: [u8; 4] = self.crc.to_be_bytes();

        let mut bytes = vec![];
        bytes.extend_from_slice(&length_bytes);
        bytes.extend(self.chunk_type.bytes());
        bytes.extend(&self.data);
        bytes.extend_from_slice(&crc_bytes);

        bytes
    }
}

/// Attempts to create a Chunk from a byte slice.
impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn Error>;

    fn try_from(chunk_data: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(chunk_data);

        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        // read length
        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);

        // read chunk type
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        // read data
        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;

        // read CRC
        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        // check CRC
        let crc_algo = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut digest = crc_algo.digest();
        digest.update(&chunk_type.bytes());
        digest.update(&data);
        let calculated_crc = digest.finalize();

        if crc != calculated_crc {
            return Err("CRC mismatch".into());
        }

        Ok(Chunk::new(chunk_type, data))
    }
}

/// Formats the Chunk for display.
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
