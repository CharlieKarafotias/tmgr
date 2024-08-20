mod cli;
mod commands;

use cli::parser;

fn main() {
    parser::run();
}
