use crate::cli::model::TaskPriority;
use crate::commands::db::DB;

pub(crate) async fn run(
    _db: &DB,
    _id: String,
    _name: Option<String>,
    _priority: Option<TaskPriority>,
    _description: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    todo!("Update command not yet implemented")
}
