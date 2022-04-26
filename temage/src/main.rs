//Import dependencies

use clap::{Parser, Subcommand};
use colored::*;
mod lib;

//Initiate our CLI tool with clap

/// Temage CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display an image given a path to a .tm file
    Display { file: Option<String> },

    /// Open the image editor, create pixel art here!
    Editor,
}

//Our main program, it is in tokio for downloading files and running the web server hosting the editor
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // Match the command to the appropriate code to be executed
    match &args.command {
        //This will execute the code that displays a .tm file. Ex: temage <path to file>
        Commands::Display { file } => match file {
            Some(file_path) => {
                //lib::functions::read outputs the contents of a .tm file
                lib::read::read(file_path).unwrap();
            }

            //Exit if no file is provided
            None => {
                println!("{}", "No file provided.".bold().red());
                std::process::exit(0);
            }
        },
        Commands::Editor => {
            // Launches the editor asynchronously
            lib::editor::launch_editor().await;
        }
    }
}
