use super::db::DB;
use super::model::Task;
use crate::cli::model::TaskPriority;
use surrealdb::sql::Datetime;

pub(crate) async fn run(
    db: &DB,
    name: String,
    priority: TaskPriority,
    description: Option<String>,
) {
    let _task: Vec<Task> = db
        .client
        .create("task")
        .content(Task {
            name,
            priority: priority.to_string(),
            description,
            created_at: Datetime::default(),
            completed_at: None,
        })
        .await
        .expect("Could not create task");

    println!("Task created successfully");
}
