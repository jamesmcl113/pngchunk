#![allow(dead_code)]

use crate::chunk_type::ChunkType;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum PngArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(StructOpt, Debug)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(StructOpt, Debug)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(StructOpt, Debug)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
