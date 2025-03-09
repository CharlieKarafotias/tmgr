use super::super::super::{
    db,
    model::{Task, TaskPriority},
};
use super::super::add;

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_then_one_task_should_write_to_db() {
    let db = db::DB::new_test().await.unwrap();
    let _ = add::run(&db, "test".to_string(), Some(TaskPriority::High), None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name(), "test");
    assert_eq!(*res[0].priority(), TaskPriority::High);
    assert!(res[0].description().is_none());
}

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_with_description_then_one_task_should_write_to_db()
 {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let _ = add::run(
        &db,
        "test".to_string(),
        Some(TaskPriority::High),
        Some("some description".to_string()),
    )
    .await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert!(res[0].description().is_some());
    assert_eq!(
        res[0].description().clone().unwrap(),
        "some description".to_string()
    );
}

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_with_low_priority_then_one_task_with_low_priority_should_write_to_db()
 {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let _ = add::run(&db, "test".to_string(), Some(TaskPriority::Low), None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(*res[0].priority(), TaskPriority::Low);
}

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_with_medium_priority_then_one_task_with_medium_priority_should_write_to_db()
 {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let _ = add::run(&db, "test".to_string(), Some(TaskPriority::Medium), None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(*res[0].priority(), TaskPriority::Medium);
}

#[tokio::test]
async fn given_the_add_command_when_adding_a_new_task_then_the_command_should_return_success_message()
 {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let res = add::run(&db, "test".to_string(), Some(TaskPriority::Medium), None).await;
    assert!(res.is_ok());
    assert!(res.unwrap().message().contains("created successfully"));
}

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_with_only_name_then_one_task_with_default_priority_should_write_to_db()
 {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let _ = add::run(&db, "test".to_string(), None, None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(*res[0].priority(), TaskPriority::Low);
}
