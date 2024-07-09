use colored::*;

pub fn print_err<E>(msg: E)
where
    E: std::fmt::Display,
{
    eprintln!("{}: {}", "ERROR".red(), msg);
    std::process::exit(1);
}
