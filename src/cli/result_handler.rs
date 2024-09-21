use colored::Colorize;

enum ExitCode {
    Success = 0,
    Failure = 1,
}
pub(crate) struct ResultHandler {
    pub(crate) result_string: String,
    pub(crate) exit_code: i32,
}
pub(crate) async fn handle_result(
    result: Result<String, Box<dyn std::error::Error>>,
) -> ResultHandler {
    match result {
        Ok(res) => ResultHandler {
            result_string: res,
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
