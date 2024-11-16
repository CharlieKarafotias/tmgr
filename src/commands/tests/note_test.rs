use crate::commands::model::Task;
use crate::commands::{db, note};
use std::path::Path;

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
    let res = note::run(&db, id.clone())
        .await
        .expect("Error creating note");

    // Should create a note file
    assert!(Path::new(&res).exists());

    // Should be a markdown file
    assert!(res.ends_with(".md"));

    // Clean up by removing the note file
    std::fs::remove_file(&res)
        .expect(format!("Failed to delete the note file at path: {res}").as_str());
}
