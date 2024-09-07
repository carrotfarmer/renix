use clap::Parser;
use std::path;

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
}

fn main() {
    let args = Cli::parse();

    println!("path: {:?}", args.path);
    println!("prefix: {:?}", args.prefix);
    println!("suffix: {:?}", args.suffix);

    println!("hello, world!");
}
