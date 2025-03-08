use super::super::super::{
    db,
    model::{Task, TaskPriority},
};
use super::super::{add, migrate};
use crate::cli::model::TmgrVersion;

#[tokio::test]
async fn given_no_tasks_in_db_when_migrating_then_no_effect() {
    let db = db::DB::new_test().await.expect("Failed to create db");

    let res = migrate::run(&db, TmgrVersion::V2).await;
    let curr_version = format!("v{}", env!("CARGO_PKG_VERSION").split(".").next().unwrap());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        format!(
            "Successfully migrated tasks from v2 to {} schema",
            curr_version
        )
    );

    let mut tasks = db.client.query("SELECT * FROM task").await.unwrap();
    let tasks: Vec<Task> = tasks.take(0).unwrap();
    assert_eq!(tasks.len(), 0);
}

#[tokio::test]
async fn given_v2_task_in_db_when_migrating_then_task_should_be_migrated() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    // Setup test data using V2 schema
    // Set priority to low (V2). In new model low -> TaskPriority::Low -> Low
    let query = "INSERT INTO task {
	    name: 'Version 2 task',
        priority: 'low',
        description: 'Version 2 task description',
        work_note_path: '/some/path/to/note/1.md',
        created_at: '2025-01-03T20:12:13.979823Z',
        completed_at: '2025-01-04T20:12:13.979823Z'
    };";
    db.client
        .query(query)
        .await
        .expect("Failed to insert test data");

    let res = migrate::run(&db, TmgrVersion::V2).await;
    let curr_version = format!("v{}", env!("CARGO_PKG_VERSION").split(".").next().unwrap());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        format!(
            "Successfully migrated tasks from v2 to {} schema",
            curr_version
        )
    );

    let mut tasks = db.client.query("SELECT * FROM task").await.unwrap();
    dbg!(&tasks);
    let tasks: Vec<Task> = tasks.take(0).unwrap();
    let task = &tasks[0];
    assert_eq!(task.name(), "Version 2 task");
    assert_eq!(
        task.description().as_ref().unwrap(),
        "Version 2 task description"
    );
    assert_eq!(*task.priority(), TaskPriority::Low);
    assert_eq!(
        task.created_at().to_string(),
        "d'2025-01-03T20:12:13.979823Z'"
    );
    assert_eq!(
        task.completed_at().as_ref().unwrap().to_string(),
        "d'2025-01-04T20:12:13.979823Z'"
    );
}

#[tokio::test]
async fn given_all_varieties_of_v2_tasks_in_db_when_migrating_then_tasks_should_be_migrated() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    // Setup test data using V2 schema
    let query = "\
    INSERT INTO
	    task (id, name, priority, description, work_note_path, created_at, completed_at)
	    VALUES  
	        ('1', 'V2_low_task', 'low', 'V2_low_task_description', '/some/path/to/note/1.md', '2025-01-03T20:12:13.979823Z', '2025-01-04T20:12:13.979823Z'),
            ('2', 'V2_medium_task', 'medium', 'V2_medium_task_description', '/some/path/to/note/2.md', '2025-01-05T20:12:13.979823Z', '2025-01-06T20:12:13.979823Z'),
            ('3', 'V2_high_task', 'high', 'V2_high_task_description', '/some/path/to/note/3.md', '2025-01-07T20:12:13.979823Z', '2025-01-08T20:12:13.979823Z');
	";
    db.client
        .query(query)
        .await
        .expect("Failed to insert test data");

    let res = migrate::run(&db, TmgrVersion::V2).await;
    let curr_version = format!("v{}", env!("CARGO_PKG_VERSION").split(".").next().unwrap());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        format!(
            "Successfully migrated tasks from v2 to {} schema",
            curr_version
        )
    );

    let mut tasks = db.client.query("SELECT * FROM task").await.unwrap();
    let tasks: Vec<Task> = tasks.take(0).unwrap();
    assert!(
        tasks.contains(
            &Task::builder()
                .id("task:⟨1⟩".to_string())
                .name("V2_low_task".to_string())
                .priority(TaskPriority::Low)
                .description("V2_low_task_description".to_string())
                .work_note_path("/some/path/to/note/1.md".to_string())
                .created_at("2025-01-03T20:12:13.979823Z".parse().unwrap())
                .completed_at("2025-01-04T20:12:13.979823Z".parse().unwrap())
                .build()
        )
    );
    assert!(
        tasks.contains(
            &Task::builder()
                .id("task:⟨2⟩".to_string())
                .name("V2_medium_task".to_string())
                .priority(TaskPriority::Medium)
                .description("V2_medium_task_description".to_string())
                .work_note_path("/some/path/to/note/2.md".to_string())
                .created_at("2025-01-05T20:12:13.979823Z".parse().unwrap())
                .completed_at("2025-01-06T20:12:13.979823Z".parse().unwrap())
                .build()
        )
    );
    assert!(
        tasks.contains(
            &Task::builder()
                .id("task:⟨3⟩".to_string())
                .name("V2_high_task".to_string())
                .priority(TaskPriority::High)
                .description("V2_high_task_description".to_string())
                .work_note_path("/some/path/to/note/3.md".to_string())
                .created_at("2025-01-07T20:12:13.979823Z".parse().unwrap())
                .completed_at("2025-01-08T20:12:13.979823Z".parse().unwrap())
                .build()
        )
    );
}

#[tokio::test]
async fn given_mix_of_v2_v3_tasks_in_db_when_migrating_then_v2_tasks_should_be_migrated() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    // Setup test data using V2 schema
    let query = "\
    INSERT INTO
	    task (id, name, priority, description, work_note_path, created_at, completed_at)
	    VALUES  
	        ('1', 'V2_low_task', 'low', 'V2_low_task_description', '/some/path/to/note/1.md', '2025-01-03T20:12:13.979823Z', '2025-01-04T20:12:13.979823Z');
	";
    db.client
        .query(query)
        .await
        .expect("Failed to insert test data");
    add::run(
        &db,
        "V3 task".to_string(),
        Some(TaskPriority::Medium),
        Some("V3 desc".to_string()),
    )
    .await
    .expect("Failed to insert test data with commands::add::run");

    let res = migrate::run(&db, TmgrVersion::V2).await;
    let curr_version = format!("v{}", env!("CARGO_PKG_VERSION").split(".").next().unwrap());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        format!(
            "Successfully migrated tasks from v2 to {} schema",
            curr_version
        )
    );

    let mut tasks = db.client.query("SELECT * FROM task").await.unwrap();
    let tasks: Vec<Task> = tasks.take(0).unwrap();
    let mut count_low = 0;
    let mut count_medium = 0;
    for task in tasks {
        match task.priority() {
            TaskPriority::Low => count_low += 1,
            TaskPriority::Medium => count_medium += 1,
            _ => panic!("Unexpected task priority"),
        }
    }

    assert_eq!(count_low, 1);
    assert_eq!(count_medium, 1);
}

#[tokio::test]
async fn given_v3_task_in_db_when_migrating_then_no_effect() {
    let db = db::DB::new_test().await.expect("Failed to create db");
    add::run(
        &db,
        "V3 task".to_string(),
        Some(TaskPriority::Medium),
        Some("V3 desc".to_string()),
    )
    .await
    .expect("Failed to insert test data with commands::add::run");

    let res = migrate::run(&db, TmgrVersion::V2).await;
    let curr_version = format!("v{}", env!("CARGO_PKG_VERSION").split(".").next().unwrap());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        format!(
            "Successfully migrated tasks from v2 to {} schema",
            curr_version
        )
    );

    let mut tasks = db.client.query("SELECT * FROM task").await.unwrap();
    let tasks: Vec<Task> = tasks.take(0).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].name(), "V3 task".to_string());
    assert_eq!(*tasks[0].priority(), TaskPriority::Medium);
    assert_eq!(
        *tasks[0].description().as_ref().unwrap(),
        "V3 desc".to_string()
    );
}
