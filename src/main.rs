mod file;
mod utils;

use std::path;

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "renix")]
#[command(version = "1.0")]
#[command(about = "fast batch renaming utility", long_about = None)]
struct Cli {
    name: Option<String>,

    // path to dir containing files to be renamed
    #[clap(
        short = 'd',
        long,
        help = "Path to directory containing files to be renamed"
    )]
    path: path::PathBuf,

    // Prefix arg
    #[clap(short = 'p', long, help = "Add a prefix to the files")]
    prefix: Option<String>,

    // Suffix arg
    #[clap(short = 's', long, help = "Add a suffix to the files")]
    suffix: Option<String>,

    #[clap(long = "no-table", help = "Do not print table")]
    no_table: bool,

    #[clap(short = 'r', long = "remove", help = "Remove a prefix and/or suffix")]
    remove: bool,
}

fn main() {
    let args = Cli::parse();

    // validate path
    if !args.path.exists() {
        utils::perror("path does not exist");
        std::process::exit(1);
    }

    if !args.path.is_dir() {
        utils::perror("path is not a directory");
        std::process::exit(1);
    }

    if !args.path.read_dir().unwrap().next().is_some() {
        utils::perror("directory is empty");
        std::process::exit(1);
    }

    if !args.no_table {
        let table = file::print_files(file::get_files(&args.path));
        println!("{}", table);
    }

    if args.remove {
        file::remove_suffix_prefix(&file::get_files(&args.path), &args.prefix, &args.suffix);
    } else {
        file::add_suffix_prefix(&file::get_files(&args.path), &args.prefix, &args.suffix);
    }

    println!("{}", "Files renamed successfully!".green());

    println!("hello, world!");
}
