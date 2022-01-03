mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use clap::Parser;

use args::PngArgs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngArgs::parse();

    match args {
        PngArgs::Encode {
            file_path,
            chunk_type,
            message,
        } => commands::encode(file_path, chunk_type, message),
        PngArgs::Decode {
            file_path,
            chunk_type,
        } => commands::decode(file_path, chunk_type),
        PngArgs::Remove {
            file_path,
            chunk_type,
        } => commands::remove(file_path, chunk_type),
        PngArgs::Print { file_path } => commands::print(file_path),
    }
}
