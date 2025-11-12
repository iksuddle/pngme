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
    /// Encode a message stored in a PNG file
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    // Decode a message stored in a PNG file
    Decode {
        path: PathBuf,
        chunk_type: String,
    },
    /// Remove a message from a PNG file
    Remove {
        path: PathBuf,
        chunk_type: String,
    },
    /// Print a list of PNG chunks that can be searched for messages
    Print {
        path: PathBuf,
    },
}
