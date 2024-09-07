#![allow(unused_imports)]

use crate::cli::model::TaskPriority;
use crate::commands::{db, model::Task, update};

// -- No params tests --
#[tokio::test]
async fn given_no_existing_tasks_when_updating_a_task_with_no_params_then_error_should_be_returned()
{
    let db = db::DB::new_test().await;
    let res = update::run(&db, "test".to_string(), None, None, None).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(res_str, "No fields to update");
}

#[tokio::test]
async fn given_a_task_when_updating_a_task_with_no_params_then_should_return_no_update_fields_error(
) {
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
    let res = update::run(&db, "test".to_string(), None, None, None).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(res_str, "No fields to update");
}

// -- END No params tests --

// -- Basic update 1 param tests --
#[tokio::test]
async fn given_existing_tasks_when_updating_a_priority_field_then_only_that_field_should_be_updated(
) {
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
    let res = update::run(
        &db,
        "test".to_string(),
        None,
        Some(TaskPriority::High),
        None,
    )
    .await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(res_str, "Successfully updated task 'test'".to_string());

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test".to_string());
    assert_eq!(res[0].priority, TaskPriority::High.to_string());
    assert!(res[0].description.is_none());
}

#[tokio::test]
async fn given_existing_tasks_when_updating_a_description_field_then_only_that_field_should_be_updated(
) {
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
    let res = update::run(
        &db,
        "test".to_string(),
        None,
        None,
        Some("new description".to_string()),
    )
    .await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(res_str, "Successfully updated task 'test'".to_string());

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test".to_string());
    assert_eq!(res[0].priority, TaskPriority::Medium.to_string());
    assert_eq!(res[0].description, Some("new description".to_string()));
}

// -- END Basic update 1 param tests --

// -- Update multiple params tests --

#[tokio::test]
async fn given_existing_tasks_when_updating_multiple_fields_then_only_those_fields_should_be_updated(
) {
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
    let res = update::run(
        &db,
        "test".to_string(),
        None,
        Some(TaskPriority::High),
        Some("new description".to_string()),
    )
    .await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(res_str, "Successfully updated task 'test'".to_string());

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test".to_string());
    assert_eq!(res[0].priority, TaskPriority::High.to_string());
    assert_eq!(res[0].description, Some("new description".to_string()));
}

// -- END Update multiple params tests --

// -- Name update tests --
#[tokio::test]
async fn given_existing_tasks_when_updating_the_name_then_the_old_task_should_be_deleted_and_a_new_task_should_be_created(
) {
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
    let res = update::run(
        &db,
        "test".to_string(),
        Some("test2".to_string()),
        Some(TaskPriority::High),
        Some("new description".to_string()),
    )
    .await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(
        res_str,
        "Successfully updated task 'test' -> 'test2'".to_string()
    );

    let db_res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(db_res.len(), 1);
    assert_eq!(db_res[0].name, "test2".to_string());
    assert_eq!(db_res[0].priority, TaskPriority::High.to_string());
    assert_eq!(db_res[0].description, Some("new description".to_string()));
}

#[tokio::test]
async fn given_existing_tasks_when_updating_only_the_name_then_only_that_field_should_be_updated() {
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
    let res = update::run(
        &db,
        "test".to_string(),
        Some("test2".to_string()),
        None,
        None,
    )
    .await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(
        res_str,
        "Successfully updated task 'test' -> 'test2'".to_string()
    );

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test2".to_string());
    assert_eq!(res[0].priority, TaskPriority::Medium.to_string());
    assert_eq!(res[0].description, Some("some description".to_string()));
}

#[tokio::test]
async fn given_no_existing_task_when_updating_a_task_with_name_then_error_should_be_returned() {
    let db = db::DB::new_test().await;
    let res = update::run(
        &db,
        "test".to_string(),
        Some("diff name".to_string()),
        None,
        None,
    )
    .await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(res_str, "Task with name 'test' not found");
}

// -- END Name update tests --
