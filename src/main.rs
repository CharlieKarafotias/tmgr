mod cli;
mod commands;
mod db;
mod model;

use cli::parser;

fn main() {
    let exit_code: i32 = parser::run();
    std::process::exit(exit_code);
}
