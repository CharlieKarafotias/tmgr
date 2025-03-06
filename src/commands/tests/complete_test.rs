use crate::cli::model::TaskPriority;
use crate::commands::model::Task;
use crate::commands::{complete, db};

#[tokio::test]
async fn given_no_existing_tasks_when_completing_a_task_then_no_task_should_be_completed() {
    let db = db::DB::new_test().await;
    let res = complete::run(&db, "randomID".to_string()).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(res_str, "Task starting with id 'randomID' was not found");
}

#[tokio::test]
async fn given_existing_tasks_when_completing_a_task_then_the_task_should_be_completed() {
    let db = db::DB::new_test().await;
    let new_task: Vec<Task> = db
        .client
        .insert("task")
        .content(Task::default())
        .await
        .unwrap();
    let id = new_task[0].id.clone().unwrap().replace("task:", "");
    let res = complete::run(&db, id.clone()).await;

    let res_str = res.unwrap();
    assert_eq!(
        res_str,
        format!("Successfully updated task '{id}' to completed")
    );

    let check_task_completed: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(check_task_completed.len(), 1);
    assert_ne!(check_task_completed[0].completed_at, None);
}

#[tokio::test]
async fn given_existing_tasks_when_completing_a_task_then_the_other_parts_of_the_task_should_stay_the_same()
 {
    let db = db::DB::new_test().await;
    let task = Task::builder()
        .name("task to complete".to_string())
        .priority(TaskPriority::Medium.to_string())
        .description("This is a description of the task".to_string())
        .build();

    let new_task: Vec<Task> = db.client.insert("task").content(task).await.unwrap();

    let id = new_task[0].id.clone().unwrap().replace("task:", "");
    let res = complete::run(&db, id.clone()).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(
        res_str,
        format!("Successfully updated task '{id}' to completed")
    );

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    let first = &res[0];
    assert!(first.completed_at.is_some());
    assert_eq!(first.name, "task to complete".to_string());
    assert_eq!(first.priority, TaskPriority::Medium.to_string());
    assert_eq!(
        first.description,
        Some("This is a description of the task".to_string())
    );
}
