use super::db::DB;
use crate::cli::model::TaskPriority;

pub(crate) fn run(name: String, priority: TaskPriority, description: Option<String>) {
    let db = DB::new();
    todo!("Add command not yet implemented")
    // add task to db
    // TODO: create a safe_run function that will check if error from db and wrap gracefully
    // db.client.create("task").content(name, priority, description, Local::now(), None).await;
}
