use crate::commands::model::Task;
use crate::commands::{db, note};

#[tokio::test]
async fn given_an_existing_task_without_note_when_calling_note_should_create_md_file() {
    let db = db::DB::new_test().await;
    let new_task: Vec<Task> = db
        .client
        .insert("task")
        .content(Task::default())
        .await
        .unwrap();
    let id = new_task[0].id.clone().unwrap().replace("task:", "");
    let res = note::run(&db, id.clone()).await;

    // Should return a success message
    assert!(res.is_ok());
    // Should create a note file
}
