use super::super::{add, db};
use crate::cli::model::TaskPriority;
use crate::commands::model::Task;

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_then_one_task_should_write_to_db() {
    let db = db::DB::new_test().await;
    add::run(&db, "test".to_string(), TaskPriority::High, None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
}
