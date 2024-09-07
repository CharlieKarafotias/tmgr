#![allow(unused_imports)]

use crate::cli::model::TaskPriority;
use crate::commands::model::Task;
use crate::commands::{db, status};

#[tokio::test]
async fn given_no_existing_tasks_when_running_status_command_then_no_tasks_should_be_reported() {
    let db = db::DB::new_test().await;
    let res = status::run(&db).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("completed tasks: 0"));
    assert!(res_str.to_lowercase().contains("in progress tasks: 0"));
    assert!(res_str.to_lowercase().contains("total tasks: 0"));
}

#[tokio::test]
async fn given_existing_tasks_when_running_status_command_then_tasks_should_be_reported() {
    let db = db::DB::new_test().await;
    let _: Vec<Task> = db
        .client
        .insert("task")
        .content(Task {
            name: "test".to_string(),
            priority: TaskPriority::Medium.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();
    let res = status::run(&db).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("total tasks: 1"));
    assert!(res_str.to_lowercase().contains("in progress tasks: 1"));
    assert!(res_str.to_lowercase().contains("completed tasks: 0"));
}

#[tokio::test]
async fn given_a_completed_task_when_running_status_command_then_the_task_should_be_reported_correctly(
) {
    let db = db::DB::new_test().await;
    let _: Vec<Task> = db
        .client
        .insert("task")
        .content(Task {
            name: "test".to_string(),
            priority: TaskPriority::Medium.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: Some(Default::default()),
        })
        .await
        .unwrap();
    let res = status::run(&db).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("total tasks: 1"));
    assert!(res_str.to_lowercase().contains("in progress tasks: 0"));
    assert!(res_str.to_lowercase().contains("completed tasks: 1"));
}
