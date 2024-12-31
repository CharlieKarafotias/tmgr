use crate::cli::model::TaskPriority;
use crate::commands::{add, db, model::Task};

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_then_one_task_should_write_to_db() {
    let db = db::DB::new_test().await;
    let _ = add::run(&db, "test".to_string(), TaskPriority::High, None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test".to_string());
    assert_eq!(res[0].priority, TaskPriority::High.to_string());
    assert!(res[0].description.is_none());
}

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_with_description_then_one_task_should_write_to_db(
) {
    let db = db::DB::new_test().await;
    let _ = add::run(
        &db,
        "test".to_string(),
        TaskPriority::High,
        Some("some description".to_string()),
    )
    .await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert!(res[0].description.is_some());
}

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_with_low_priority_then_one_task_with_low_priority_should_write_to_db(
) {
    let db = db::DB::new_test().await;
    let _ = add::run(&db, "test".to_string(), TaskPriority::Low, None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].priority, TaskPriority::Low.to_string());
}

#[tokio::test]
async fn given_no_existing_tasks_when_adding_a_new_task_with_medium_priority_then_one_task_with_medium_priority_should_write_to_db(
) {
    let db = db::DB::new_test().await;
    let _ = add::run(&db, "test".to_string(), TaskPriority::Medium, None).await;
    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].priority, TaskPriority::Medium.to_string());
}

#[tokio::test]
async fn given_the_add_command_when_adding_a_new_task_then_the_command_should_return_success_message(
) {
    let db = db::DB::new_test().await;
    let res = add::run(&db, "test".to_string(), TaskPriority::Medium, None).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert!(res_str.message().contains("created successfully"));
}
