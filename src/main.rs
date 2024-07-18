mod parser;
mod utils;
use parser::run_cli;

#[tokio::main]
async fn main() {
    run_cli().await
}
