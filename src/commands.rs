#![allow(dead_code)]

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::png::Png;
use crate::Result;
use std::fs;
use std::path::Path;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let contents = from_file(args.file_path)?;
    let mut png = Png::try_from(&contents[..])?;
    png.append_chunk(Chunk::new(args.chunk_type, args.message.into_bytes()));

    match args.output_file {
        Some(output_file) => {
            to_file(output_file, &png.as_bytes())?;
        }
        None => {}
    };

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let contents = from_file(args.file_path)?;
    let png = Png::try_from(&contents[..])?;
    match png.chunk_by_type(&args.chunk_type.to_string()) {
        Some(chunk) => {
            println!("{}", chunk.data_as_string()?);
            Ok(())
        }
        None => Err("Chunk not found.".into()),
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let contents = from_file(&args.file_path)?;
    let mut png = Png::try_from(&contents[..])?;
    if png.remove_chunk(&args.chunk_type.to_string()).is_err() {
        return Err("Chunk not found".into());
    }

    to_file(&args.file_path, &png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let contents = from_file(&args.file_path)?;
    let png = Png::try_from(&contents[..])?;
    png.chunks().iter().for_each(|chunk| println!("{}", chunk));
    Ok(())
}

fn from_file<P: AsRef<Path>>(file: P) -> Result<Vec<u8>> {
    fs::read(file.as_ref()).map_err(|e| e.into())
}

fn to_file<P: AsRef<Path>>(file: P, contents: &[u8]) -> Result<()> {
    fs::write(file.as_ref(), contents).map_err(|e| e.into())
}
