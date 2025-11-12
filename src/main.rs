use clap::Parser;

use crate::args::Cli;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        args::Commands::Encode {
            path,
            chunk_type,
            message,
            output_file,
        } => commands::encode(path, chunk_type, message, output_file)?,
        args::Commands::Decode { path, chunk_type } => {
            let msg = commands::decode(path, chunk_type)?;
            println!("{}", msg);
        }
        args::Commands::Remove { path, chunk_type } => commands::remove(path, chunk_type)?,
        args::Commands::Print { path } => commands::print(path)?,
    }

    Ok(())
}
