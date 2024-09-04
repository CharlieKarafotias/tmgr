use super::super::{db, list};
use crate::cli::model::TaskPriority;
use crate::commands::model::Task;
use surrealdb::sql::Datetime;

#[tokio::test]
async fn given_no_existing_tasks_when_listing_all_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    let res = list::run(&db, true).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.is_empty());
}

#[tokio::test]
async fn given_no_existing_tasks_when_listing_in_progress_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    let res = list::run(&db, false).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.is_empty());
}

#[tokio::test]
async fn given_existing_tasks_when_listing_all_tasks_then_all_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    let _: Vec<Task> = db
        .client
        .insert("task")
        .content(Task {
            name: "test".to_string(),
            priority: TaskPriority::High.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();
    let res = list::run(&db, true).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("test"));
    assert!(res_str.to_lowercase().contains("high"));
}

#[tokio::test]
async fn given_existing_tasks_when_listing_in_progress_tasks_then_only_in_progress_tasks_should_be_returned(
) {
    let db = db::DB::new_test().await;
    let _: Vec<Task> = db
        .client
        .insert("task")
        .content(Task {
            name: "in progress task".to_string(),
            priority: TaskPriority::High.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();
    let _: Vec<Task> = db
        .client
        .insert("task")
        .content(Task {
            name: "Completed task".to_string(),
            priority: TaskPriority::High.to_string(),
            description: None,
            created_at: Datetime::default(),
            completed_at: Some(Datetime::default()),
        })
        .await
        .unwrap();
    let res = list::run(&db, false).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("in progress task"));
    assert!(!res_str.to_lowercase().contains("Completed task"));
}
