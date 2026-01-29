use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    fmt::format,
    fs,
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::{
    settings::{object::Columns, Color, Style},
    Table, Tabled,
};

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    len_bits: u64,
    modified: String,
}

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
            // 3   for file in get_files(&path) {
            //        println!("{:?}", file)
            //    }
            let get_files = get_files(&path);
            let mut cli_table = Table::new(get_files);
            cli_table.with(Style::rounded());
            cli_table.modify(Columns::first(), Color::FG_BRIGHT_BLUE);
            //cli_table.modify(Columns::first(), Color::FG_BRIGHT_BLUE);
            // you can include a forloop to clear the matter
            //         cli_table.with(Style::rounded);
            //      cli_table.modify(Columns::first(), Alignment::right());
            println!("{}", cli_table)

        //for filw in get files{
        // println!("{:?}", cli_table)}
        } else {
            println!("{}", "path does not exist".red())
        }
    } else {
        println!("{}", "error".bright_blue());

        //  println!("{}", path.display());
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                mapdata(file, &mut data);
            }
        }
    }
    // why is this value appended at the end of a rust function
    data
}

fn mapdata(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file.file_name().into_string().unwrap_or("unknown".into()),
            e_type: {
                if meta.is_dir() {
                    EntryType::Dir
                } else {
                    EntryType::File
                }
            },
            len_bits: meta.len(),
            modified: if let Ok(modf) = meta.modified() {
                let date: DateTime<Utc> = modf.into();
                format!("{}", date.format("%a %b %Y"))
            } else {
                String::default()
            },
        });
    }
}
