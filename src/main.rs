use clap::Parser;
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best Tooler Ever")]

struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
        } else {
            print!("{}", "path does not exist".red())
        }
    }

    println!("{}", path.display());
}
