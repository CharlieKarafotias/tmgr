use super::super::super::{db, model::Task};
use super::super::list;

#[tokio::test]
async fn given_no_existing_tasks_when_listing_all_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let res = list::run(&db, true).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(res_str.contains("task:"), false);
}

#[tokio::test]
async fn given_no_existing_tasks_when_listing_in_progress_tasks_then_no_tasks_should_be_returned() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let res = list::run(&db, false).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(res_str.contains("task:"), false);
}

#[tokio::test]
async fn given_existing_tasks_when_listing_all_tasks_then_all_tasks_should_be_returned() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let _: Vec<Task> = db
        .client
        .insert("task")
        .content(Task::default())
        .await
        .unwrap();
    let res = list::run(&db, true).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.contains("Low"));
}

#[tokio::test]
async fn given_existing_tasks_when_listing_in_progress_tasks_then_only_in_progress_tasks_should_be_returned()
 {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let task1 = Task::builder().name("in progress task".to_string()).build();
    let task2 = Task::builder()
        .name("Completed task".to_string())
        .completed_at(Default::default())
        .build();
    let _: Vec<Task> = db.client.insert("task").content(task1).await.unwrap();
    let _: Vec<Task> = db.client.insert("task").content(task2).await.unwrap();

    let res = list::run(&db, false).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.to_lowercase().contains("in progress task"));
    assert_eq!(res_str.to_lowercase().contains("Completed task"), false);
}
