#![allow(unused_imports)]
use crate::cli::model::TaskPriority;
use crate::commands::{db, delete, model::Task};

#[tokio::test]
async fn given_no_existing_tasks_when_deleting_a_task_then_no_task_should_be_deleted() {
    let db = db::DB::new_test().await;
    let res = delete::run(&db, "test".to_string()).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(res_str, "Task with name 'test' not found");
}

#[tokio::test]
async fn given_existing_tasks_when_deleting_a_task_then_the_task_should_be_deleted() {
    let db = db::DB::new_test().await;
    let _: Option<Task> = db
        .client
        .insert(("task", "test"))
        .content(Task {
            name: "test".to_string(),
            priority: TaskPriority::Medium.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();
    let res = delete::run(&db, "test".to_string()).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(res_str, "Successfully deleted task 'test'".to_string());

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 0);
}
