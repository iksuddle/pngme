use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode {
        path: PathBuf,
        chunk_type: String,
    },
    Remove {
        path: PathBuf,
        chunk_type: String,
    },
    Print {
        path: PathBuf,
    },
}
