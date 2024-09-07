use crate::commands::db::DB;
use std::env::current_exe;

pub(crate) async fn run(db: &DB) -> Result<String, Box<dyn std::error::Error>> {
    let mut res = String::new();
    res.push_str("File locations:\n");
    res.push_str(&format!(
        "  tmgr executable: {:?}\n",
        current_exe()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "Unable to determine executable location".to_string())
    ));
    res.push_str(&format!(
        "  database: {}\n",
        DB::get_db_file_path().display()
    ));
    res.push_str("General statistics:\n");
    let task_count = get_number_of_tasks(db).await;
    if let Ok(task_count) = task_count {
        res.push_str(&format!("  completed tasks: {}\n", task_count.completed));
        res.push_str(&format!(
            "  in progress tasks: {}\n",
            task_count.in_progress
        ));
        res.push_str(&format!("  total tasks: {}\n", task_count.total));
    } else {
        res.push_str(
            "  completed tasks: unable to determine number of tasks in current database\n",
        );
        res.push_str(
            "  in progress tasks: unable to determine number of tasks in current database\n",
        );
        res.push_str("  total tasks: unable to determine number of tasks in current database\n");
    }

    Ok(res)
}

struct TaskCount {
    completed: i32,
    in_progress: i32,
    total: i32,
}

async fn get_number_of_tasks(db: &DB) -> Result<TaskCount, Box<dyn std::error::Error>> {
    let mut db_res = db
        .client
        .query("SELECT count() as total, count(completed_at != None) as completed  FROM task GROUP BY total;")
        .await
        .map_err(|_| "unable to determine number of tasks in current database".to_string())?;

    let total: Option<i32> = db_res.take("total")?;
    let completed: Option<i32> = db_res.take("completed")?;

    let mut task_count = TaskCount {
        completed: 0,
        in_progress: 0,
        total: 0,
    };
    if let Some(total) = total {
        task_count.total = total;
    }
    if let Some(completed) = completed {
        task_count.completed = completed;
    }

    task_count.in_progress = task_count.total - task_count.completed;
    Ok(task_count)
}
