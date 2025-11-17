use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A CLI tool for encoding and decoding secret messages in PNG files
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
        /// Path to the PNG file to encode a message into
        path: PathBuf,
        /// The type of chunk to store the message in (4 characters)
        chunk_type: String,
        /// The message to encode into the PNG file
        message: String,
        /// Optional output file path (defaults to input file)
        output_file: Option<PathBuf>,
    },
    /// Decode a message stored in a PNG file
    Decode {
        /// Path to the PNG file to decode a message from
        path: PathBuf,
        /// The type of chunk to decode the message from (4 characters)
        chunk_type: String,
    },
    /// Remove a message from a PNG file
    Remove {
        /// Path to the PNG file to remove a message from
        path: PathBuf,
        /// The type of chunk to remove (4 characters)
        chunk_type: String,
    },
    /// Print a list of PNG chunks that can be searched for messages
    Print {
        /// Path to the PNG file to print chunks from
        path: PathBuf,
    },
}
