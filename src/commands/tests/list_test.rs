use super::super::{db, list};
use crate::cli::model::TaskPriority;
use crate::commands::model::Task;
use surrealdb::sql::Datetime;

#[tokio::test]
async fn given_no_existing_tasks_when_listing_all_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    list::run(&db, true).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 0);
}

#[tokio::test]
async fn given_no_existing_tasks_when_listing_in_progress_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    list::run(&db, false).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 0);
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
    list::run(&db, true).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test".to_string());
    assert_eq!(res[0].priority, TaskPriority::High.to_string());
    assert!(res[0].description.is_none());
}

#[tokio::test]
#[ignore]
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
    // TODO: determine way to test this (consider passing a generic buffer instead of stdout)
}
