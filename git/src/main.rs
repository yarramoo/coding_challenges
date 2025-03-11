use std::io;
use std::fs::create_dir;
use std::fs::File;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[arg(help = "Path to initialise git repo")]
        path: Option<String>,
    },
}

fn run(cmd: &Command) -> io::Result<()> {
    match cmd {
        Command::Init { path } => init_repo(path),
    };
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}

fn init_repo(path: &Option<String>) -> io::Result<()> {
    // check for existing repo
    // make the .git dir
    // fill initialise .git dir
    Ok(())
}

