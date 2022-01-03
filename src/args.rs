use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub enum PngArgs {
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
    },
    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },
    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },
    Print {
        file_path: PathBuf,
    },
}
