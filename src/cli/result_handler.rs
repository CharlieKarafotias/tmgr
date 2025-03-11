use super::super::model::TmgrError;
use colored::Colorize;

enum ExitCode {
    Success = 0,
    Failure = 1,
}
pub(in crate::cli) struct ResultHandler {
    result_string: String,
    exit_code: i32,
}

impl ResultHandler {
    pub(in crate::cli) fn result_string(&self) -> &str {
        &self.result_string
    }

    pub(in crate::cli) fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

pub(in crate::cli) async fn handle_result(result: Result<String, TmgrError>) -> ResultHandler {
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
