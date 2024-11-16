use crate::commands::model::Task;
use crate::commands::{
    db,
    note::{self, path_from_id},
};
use std::fs::File;
use std::io::{self, BufRead, Write};
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
    let res = note::run(&db, id.clone(), false)
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

#[tokio::test]
async fn when_creating_note_should_write_correct_header() {
    let db = db::DB::new_test().await;

    let new_task: Vec<Task> = db
        .client
        .insert("task")
        .content(Task::default())
        .await
        .unwrap();
    let id = new_task[0].id.clone().unwrap().replace("task:", "");
    let res = note::run(&db, id.clone(), false)
        .await
        .expect("Error creating note");

    let file = File::open(&res).expect("Failed to open the note file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    assert_eq!(lines[0], format!("# Task {id} - a new task"));
    assert_eq!(lines[1], "");
    assert_eq!(lines[2], "## , high");
    assert_eq!(lines[3], "");
    assert_eq!(lines[4], "# Notes");
    assert_eq!(lines[5], "");

    // Clean up by removing the note file
    std::fs::remove_file(&res)
        .expect(format!("Failed to delete the note file at path: {res}").as_str());
}

#[tokio::test]
async fn given_an_existing_task_with_note_when_calling_note_should_not_create_md_file() {
    let db = db::DB::new_test().await;

    // Create a temp file
    let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(b"hello world").unwrap();

    let mut task = Task::default();
    task.work_note_path = Some(temp_file.path().to_string_lossy().to_string());

    let task: Vec<Task> = db.client.insert("task").content(task).await.unwrap();

    let id = task[0].id.clone().unwrap().replace("task:", "");
    let res = note::run(&db, id.clone(), false)
        .await
        .expect("Error creating note");

    // Should return the path to the note file
    assert_eq!(res, temp_file.path().to_string_lossy().to_string());

    // content of the note file should be the same as the temp file
    let content = std::fs::read_to_string(temp_file.path()).expect("Failed to read temp file");
    assert_eq!(content, "hello world");
}

#[test]
fn should_return_filename_with_id_and_md_extension() {
    let id = "123";
    let path = path_from_id(id);
    assert!(path
        .to_str()
        .unwrap()
        .ends_with(format!("{}.md", id).as_str()));
}
