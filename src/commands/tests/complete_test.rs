#![allow(unused_imports)]

use crate::cli::model::TaskPriority;
use crate::commands::model::Task;
use crate::commands::{complete, db};

#[tokio::test]
async fn given_no_existing_tasks_when_completing_a_task_then_no_task_should_be_completed() {
    let db = db::DB::new_test().await;
    let res = complete::run(&db, "test".to_string()).await;
    assert!(res.is_err());
    let res_str = res.unwrap_err().to_string();
    assert_eq!(res_str, "Task with name 'test' not found");
}

#[tokio::test]
async fn given_existing_tasks_when_completing_a_task_then_the_task_should_be_completed() {
    let db = db::DB::new_test().await;
    let _: Option<Task> = db
        .client
        .insert(("task", "in progress task"))
        .content(Task {
            name: "in progress task".to_string(),
            priority: TaskPriority::High.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();
    let res = complete::run(&db, "in progress task".to_string()).await;
    // assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(
        res_str,
        "Successfully updated task 'in progress task' to completed".to_string()
    );

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_ne!(res[0].completed_at, None);
}

#[tokio::test]
async fn given_two_tasks_with_similar_names_when_completing_task_by_name_then_the_task_with_the_exact_name_should_be_completed(
) {
    let db = db::DB::new_test().await;
    let _: Option<Task> = db
        .client
        .insert(("task", "one task"))
        .content(Task {
            name: "one task".to_string(),
            priority: TaskPriority::High.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();
    let _: Option<Task> = db
        .client
        .insert(("task", "one more task"))
        .content(Task {
            name: "one more task".to_string(),
            priority: TaskPriority::Low.to_string(),
            description: None,
            created_at: Default::default(),
            completed_at: None,
        })
        .await
        .unwrap();

    let res = complete::run(&db, "one task".to_string()).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    assert_eq!(
        res_str,
        "Successfully updated task 'one task' to completed".to_string()
    );

    let res: Vec<Task> = db.client.select("task").await.unwrap();
    assert_eq!(res.len(), 2);
    let first = &res[0];
    let second = &res[1];
    let mut completed_count = 0;
    if first.completed_at.is_some() {
        completed_count += 1;
    }
    if second.completed_at.is_some() {
        completed_count += 1;
    }
    assert_eq!(completed_count, 1);
}
