use std::{path::PathBuf, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

// Encode a message stored in a PNG file
pub fn encode(
    path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
) -> crate::Result<()> {
    let mut png = Png::from_file(&path)?;

    let chunk_type = ChunkType::from_str(&chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);

    let path = match output_file {
        Some(output_path) => output_path,
        None => path,
    };

    std::fs::write(path, png.as_bytes())?;

    Ok(())
}

// Decode a message stored in a PNG file
pub fn decode(path: PathBuf, chunk_type: String) -> crate::Result<String> {
    let png = Png::from_file(&path)?;
    let chunk = png
        .chunk_by_type(&chunk_type)
        .ok_or(format!("chunk {} not found", chunk_type))?;

    let msg = chunk.data_as_string()?;

    Ok(msg)
}

// Remove a message from a PNG file
pub fn remove(path: PathBuf, chunk_type: String) -> crate::Result<()> {
    let mut png = Png::from_file(&path)?;
    png.remove_first_chunk(&chunk_type)?;

    Ok(())
}

// Print a list of PNG chunks that can be searched for messages
pub fn print(path: PathBuf) -> crate::Result<()> {
    let png = Png::from_file(&path)?;
    let chunks = png.chunks();

    for c in chunks {
        println!("- {}", c);
    }

    Ok(())
}
