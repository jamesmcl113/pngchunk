use args::PngArgs;
use structopt::StructOpt;

mod args;
pub mod chunk;
pub mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let opt = PngArgs::from_args();
    match opt {
        PngArgs::Encode(args) => commands::encode(args)?,
        PngArgs::Decode(args) => commands::decode(args)?,
        PngArgs::Remove(args) => commands::remove(args)?,
        PngArgs::Print(args) => commands::print_chunks(args)?,
    }
    Ok(())
}
