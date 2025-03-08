use crate::cli::result_handler::handle_result;
use colored::Colorize;
use std::error::Error;

#[tokio::test]
async fn given_ok_result_when_handling_result_then_passed_in_string_should_be_returned() {
    let result = Ok("ok".to_string());
    let handler = handle_result(result).await;
    assert_eq!(handler.result_string(), "ok".to_string());
}

#[tokio::test]
async fn given_ok_result_when_handling_result_then_exit_code_0_should_be_returned() {
    let result = Ok("ok".to_string());
    let handler = handle_result(result).await;
    assert_eq!(handler.exit_code(), 0);
}

#[tokio::test]
async fn given_error_result_when_handling_result_then_error_string_should_be_returned() {
    let result: Result<String, Box<dyn Error>> = Err("An error occurred:".into());
    let handler = handle_result(result).await;
    let expected = format!("{}: An error occurred:", "error".red());
    assert_eq!(handler.result_string(), expected);
}

#[tokio::test]
async fn given_error_result_when_handling_result_then_exit_code_1_should_be_returned() {
    let result: Result<String, Box<dyn Error>> = Err("An error occurred:".into());
    let handler = handle_result(result).await;
    assert_eq!(handler.exit_code(), 1);
}
