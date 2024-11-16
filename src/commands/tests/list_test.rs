use super::super::{db, list};
use crate::commands::model::Task;

#[tokio::test]
async fn given_no_existing_tasks_when_listing_all_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    let res = list::run(&db, true).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(!res_str.contains("task:"));
}

#[tokio::test]
async fn given_no_existing_tasks_when_listing_in_progress_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    let res = list::run(&db, false).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(!res_str.contains("task:"));
}

#[tokio::test]
async fn given_existing_tasks_when_listing_all_tasks_then_all_tasks_should_be_returned() {
    let db = db::DB::new_test().await;
    let _: Vec<Task> = db
        .client
        .insert("task")
        .content(Task::default())
        .await
        .unwrap();
    let res = list::run(&db, true).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("a new task"));
    assert!(res_str.to_lowercase().contains("high"));
}

#[tokio::test]
async fn given_existing_tasks_when_listing_in_progress_tasks_then_only_in_progress_tasks_should_be_returned(
) {
    let db = db::DB::new_test().await;
    let mut task1 = Task::default();
    task1.name = "in progress task".to_string();
    let mut task2 = Task::default();
    task2.name = "Completed task".to_string();
    let _: Vec<Task> = db.client.insert("task").content(task1).await.unwrap();
    let _: Vec<Task> = db.client.insert("task").content(task2).await.unwrap();

    let res = list::run(&db, false).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("in progress task"));
    assert!(!res_str.to_lowercase().contains("Completed task"));
}
