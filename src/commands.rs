use std::path::PathBuf;
use std::str::FromStr;

use crate::chunk_type::ChunkType;
use crate::error::Error as PngError;
use crate::png::{Chunk, Png};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn encode(file_path: PathBuf, chunk_type: String, message: String) -> Result<()> {
    let bytes = std::fs::read(&file_path)?;
    let mut png = Png::try_from(bytes.as_slice())?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(&chunk_type)?,
        message.into_bytes(),
    ));
    std::fs::write(file_path, png.as_bytes())?;
    Ok(())
}

pub fn decode(file_path: PathBuf, chunk_type: String) -> Result<()> {
    let bytes = std::fs::read(file_path)?;
    let png = Png::try_from(bytes.as_slice())?;
    if let Some(chunk) = png.chunk_by_type(&chunk_type) {
        println!("{}", chunk.data_as_string()?);
        Ok(())
    } else {
        Err(Box::new(PngError::NoChunkOfGivenTypeFound))
    }
}

pub fn remove(file_path: PathBuf, chunk_type: String) -> Result<()> {
    let bytes = std::fs::read(&file_path)?;
    let mut png = Png::try_from(bytes.as_slice())?;
    png.remove_chunk(&chunk_type)?;
    std::fs::write(file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(file_path: PathBuf) -> Result<()> {
    let bytes = std::fs::read(file_path)?;
    let png = Png::try_from(bytes.as_slice())?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
