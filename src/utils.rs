use colored::*;

pub fn perror(msg: &str) {
    eprintln!("{}: {}", "error".red().bold(), msg.red());
    return;
}
