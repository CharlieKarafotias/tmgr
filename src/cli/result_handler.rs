use crate::cli::model::CommandResult;
use colored::Colorize;
use std::error::Error;

enum ExitCode {
    Success = 0,
    Failure = 1,
}
pub(crate) struct ResultHandler {
    pub(crate) result_string: String,
    pub(crate) exit_code: i32,
}
pub(crate) async fn handle_result<T>(
    result: Result<CommandResult<T>, Box<dyn Error>>,
) -> ResultHandler {
    match result {
        Ok(res) => ResultHandler {
            result_string: res.message().to_string(),
            exit_code: ExitCode::Success as i32,
        },
        Err(err) => {
            let response = format!("{}: {err}", "error".red());
            ResultHandler {
                result_string: response,
                exit_code: ExitCode::Failure as i32,
            }
        }
    }
}
