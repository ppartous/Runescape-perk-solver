use std:: process;
use colored::*;

pub fn print_error(err: &str) -> ! {
    eprintln!("{}{} {err}", "error".red().bold(), ":".bold());
    process::exit(0)
}

