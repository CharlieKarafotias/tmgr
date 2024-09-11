#![allow(unused_imports)]

use crate::cli::model::TaskPriority;
use crate::commands::model::Task;
use crate::commands::{db, view};

#[tokio::test]
async fn given_no_existing_task_when_viewing_a_task_then_error_should_be_returned() {
    let db = db::DB::new_test().await;
    let res = view::run(&db, "test".to_string()).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(res_str, "Task with name 'test' not found");
}

#[tokio::test]
async fn given_existing_task_when_viewing_a_task_then_all_fields_should_be_returned() {
    let db = db::DB::new_test().await;
    let _: Option<Task> = db
        .client
        .insert(("task", "test"))
        .content(Task {
            name: "test".to_string(),
            priority: TaskPriority::Medium.to_string(),
            description: Some("some description".to_string()),
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();
    let res = view::run(&db, "test".to_string()).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.contains("Name: \"test\""));
    assert!(res_str.contains("Priority: \"medium\""));
    assert!(res_str.contains("Description: \"some description\""));
    assert!(res_str.contains("created_at: "));
    assert!(res_str.contains("completed_at: "));
}
