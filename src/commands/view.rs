use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let query =
        format!("SELECT * from task WHERE string::starts_with(<string> id, \"task:{id}\");");
    let mut db_res = db.client.query(query).await?;

    let tasks: Vec<Task> = db_res.take(0)?;

    if tasks.is_empty() {
        return Err(format!("Task starting with id '{id}' was not found").into());
    }

    if tasks.len() > 1 {
        return Err("Multiple tasks found, provide more characters of the id".into());
    }

    Ok(format!("{}", tasks[0]))
}
