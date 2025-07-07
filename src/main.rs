mod cliutils;
use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand, arg, builder::OsStr};
#[derive(Parser)]
#[command(name="usbasic",about = "UnixSoft BASIC CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    Init {
        #[arg(default_value = "./")]
        working_dir: String,
    },
    Build {
        #[arg()]
        entry: String,
    },
    Run {
        #[arg()]
        entry: String,
    },
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { working_dir } => {
            cliutils::generate_default_project(PathBuf::from(working_dir));
        }
        Command::Build { entry } => todo!(),
        Command::Run { entry } => todo!(),
    }
}
