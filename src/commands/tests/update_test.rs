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
    let db_res: Vec<Task> = db
        .client
        .insert("task")
        .content(Task::default())
        .await
        .unwrap();
    let id = db_res[0].id.clone().unwrap().replace("task:", "");

    let res = update::run(&db, id.clone(), None, None, None).await;
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
    let task = Task::builder()
        .name("test".to_string())
        .priority(TaskPriority::Medium.to_string())
        .build();

    let db_res: Vec<Task> = db.client.insert("task").content(task).await.unwrap();
    let id = db_res[0].id.clone().unwrap().replace("task:", "");

    let res = update::run(&db, id.clone(), None, Some(TaskPriority::High), None)
        .await
        .unwrap();
    let res_str = res.message();
    assert_eq!(res_str, format!("Successfully updated task '{id}'"));

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
    let task = Task::builder()
        .name("test".to_string())
        .priority(TaskPriority::Medium.to_string())
        .build();

    let db_res: Vec<Task> = db.client.insert("task").content(task).await.unwrap();
    let id = db_res[0].id.clone().unwrap().replace("task:", "");

    let res = update::run(
        &db,
        id.clone(),
        None,
        None,
        Some("new description".to_string()),
    )
    .await
    .unwrap();
    let res_str = res.message();
    assert_eq!(res_str, format!("Successfully updated task '{id}'"));

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test".to_string());
    assert_eq!(res[0].priority, TaskPriority::Medium.to_string());
    assert_eq!(res[0].description, Some("new description".to_string()));
}

#[tokio::test]
async fn given_existing_tasks_when_updating_the_name_then_only_that_field_should_be_updated() {
    let db = db::DB::new_test().await;

    let task = Task::builder()
        .name("test".to_string())
        .priority(TaskPriority::Medium.to_string())
        .description("some description".to_string())
        .build();

    let db_res: Vec<Task> = db.client.insert("task").content(task).await.unwrap();
    let id = db_res[0].id.clone().unwrap().replace("task:", "");

    let res = update::run(&db, id.clone(), Some("test2".to_string()), None, None)
        .await
        .unwrap();
    let res_str = res.message();
    assert_eq!(res_str, format!("Successfully updated task '{id}'"));

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test2".to_string());
    assert_eq!(res[0].priority, TaskPriority::Medium.to_string());
    assert_eq!(res[0].description, Some("some description".to_string()));
}

// -- END Basic update 1 param tests --

// -- Update multiple params tests --

#[tokio::test]
async fn given_existing_tasks_when_updating_multiple_fields_then_only_those_fields_should_be_updated(
) {
    let db = db::DB::new_test().await;

    let task = Task::builder()
        .name("test".to_string())
        .priority(TaskPriority::Medium.to_string())
        .build();

    let db_res: Vec<Task> = db.client.insert("task").content(task).await.unwrap();
    let id = db_res[0].id.clone().unwrap().replace("task:", "");

    let res = update::run(
        &db,
        id.clone(),
        None,
        Some(TaskPriority::High),
        Some("new description".to_string()),
    )
    .await
    .unwrap();

    let res_str = res.message();
    assert_eq!(res_str, format!("Successfully updated task '{id}'"));

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].name, "test".to_string());
    assert_eq!(res[0].priority, TaskPriority::High.to_string());
    assert_eq!(res[0].description, Some("new description".to_string()));
}

// -- END Update multiple params tests --
