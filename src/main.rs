use clap::{Parser, Subcommand};

use commands::python::PythonCommand;

pub mod commands;
pub mod config;
pub mod templates;
pub mod utils;

#[derive(Parser)]
#[command(
    name = "templify",
    version = "0.1.0",
    author = "Your Name",
    about = "Effortlessly generates and manages coding templates"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Python(PythonCommand),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Python(python_cmd) => python_cmd.execute(),
    }
}
