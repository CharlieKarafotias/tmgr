use super::super::super::{
    db,
    model::{Task, TaskPriority},
};
use super::super::view;
use chrono;
use surrealdb::sql::Datetime;

#[tokio::test]
async fn given_no_existing_task_when_viewing_a_task_then_error_should_be_returned() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    let res = view::run(&db, "randomID".to_string()).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(
        res_str,
        "Task starting with id 'randomID' was not found (db error: No tasks found) (view error: Database error)"
    );
}

#[tokio::test]
async fn given_existing_task_when_wrong_id_is_passed_then_error_should_be_returned() {
    let db = db::DB::new_test().await.expect("Failed to create db");

    let task = Task::builder()
        .name("test".to_string())
        .priority(TaskPriority::Medium)
        .description("some description".to_string())
        .build();

    let _: Vec<Task> = db.client.insert("task").content(task).await.unwrap();

    let res = view::run(&db, "DefinitelyNotTheID".to_string()).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(
        res_str,
        "Task starting with id 'DefinitelyNotTheID' was not found (db error: No tasks found) (view error: Database error)"
    );
}

#[tokio::test]
async fn given_existing_tasks_when_unspecific_id_is_passed_then_error_should_be_returned() {
    let db = db::DB::new_test().await.expect("Failed to create db");

    let task = Task::builder()
        .name("test".to_string())
        .priority(TaskPriority::Medium)
        .description("some description".to_string())
        .build();
    let task2 = Task::builder()
        .name("test2".to_string())
        .priority(TaskPriority::Medium)
        .description("some description".to_string())
        .build();

    let _: Vec<Task> = db.client.insert("task").content(task).await.unwrap();
    let _: Vec<Task> = db.client.insert("task").content(task2).await.unwrap();

    let res = view::run(&db, "".to_string()).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(
        res_str,
        "Multiple tasks found, provide more characters of the id (db error: Multiple tasks found) (view error: Database error)"
    );
}

#[tokio::test]
async fn given_existing_task_when_viewing_a_task_then_all_fields_should_be_returned() {
    let db = db::DB::new_test().await.expect("Failed to create db");

    let task = Task::builder()
        .name("test".to_string())
        .priority(TaskPriority::Medium)
        .description("some description".to_string())
        .created_at(Datetime::from(
            chrono::DateTime::from_timestamp(0, 0).unwrap(),
        ))
        .build();

    let db_res: Vec<Task> = db.client.insert("task").content(task).await.unwrap();
    let id = db_res[0].id().unwrap();

    let res = view::run(&db, id.clone()).await;
    let res_str = res.unwrap().message().to_string();
    assert!(res_str.contains("test"));
    assert!(res_str.contains("Medium"));
    assert!(res_str.contains("some description"));
    assert!(res_str.contains("1970-01-01T00:00:00Z"));
    assert!(res_str.contains("In Progress"));
}
