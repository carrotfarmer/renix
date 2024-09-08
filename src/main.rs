mod file;
mod utils;

use std::path;

use clap::Parser;
use colored::*;

#[derive(Parser)]
#[command(name = "renix")]
#[command(version = "1.0")]
#[command(about = "fast batch renaming utility", long_about = None)]
struct Cli {
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

    #[clap(short = 'c', long = "case", help = "Change case of file names")]
    case: Option<String>,

    #[clap(long = "replace", help = "Replace a substring in the file names")]
    replace: Option<String>,

    #[clap(short = 'e', long = "exclude", help = "Exclude certain files")]
    exclude: Option<Vec<path::PathBuf>>,
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

    if args.exclude.is_some() {
        let exclude = args.exclude.clone().unwrap();
        for path in exclude {
            if !path.exists() {
                utils::perror("exclude path does not exist");
                std::process::exit(1);
            }
        }
    }

    let mut new_file_paths = if args.remove {
        let exclude = &args.exclude;

        file::remove_suffix_prefix(
            &file::get_files(&args.path),
            &args.prefix,
            &args.suffix,
            &exclude,
        )
    } else {
        let exclude = &args.exclude;

        file::add_suffix_prefix(
            &file::get_files(&args.path),
            &args.prefix,
            &args.suffix,
            &exclude,
        )
    };

    if args.case.is_some() {
        let case = args.case.unwrap();
        new_file_paths = file::change_case(&new_file_paths, &case, &args.exclude);
    }

    if args.replace.is_some() {
        let replace = args.replace.unwrap();
        let parts: Vec<&str> = replace.split(',').collect();
        if parts.len() != 2 {
            utils::perror("invalid replace argument");
            std::process::exit(1);
        }

        new_file_paths =
            file::replace_substring(&new_file_paths, parts[0], parts[1], &args.exclude);
    }

    if !args.no_table {
        let table = file::print_table(&file::get_files(&args.path), new_file_paths.clone());
        println!("{}", table);
    }

    println!("{}", "Do you want to rename the files? (y/n)".yellow());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if !input.trim().eq_ignore_ascii_case("y") || input.trim().is_empty() {
        println!("{}", "Exiting...".yellow());
        std::process::exit(0);
    }

    for (old, new) in file::get_files(&args.path)
        .iter()
        .zip(new_file_paths.iter())
    {
        std::fs::rename(old, new).unwrap();
    }

    println!("{}", "Files renamed successfully!".green());
}
