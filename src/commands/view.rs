use crate::commands::db::DB;
use crate::commands::model::Task;
use comfy_table::Table;

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

    let mut table = Table::new();
    let t = tasks.into_iter().next().unwrap();
    table
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec![
            "id",
            "name",
            "priority",
            "description",
            "created_at",
            "completed_at",
        ])
        .add_row(vec![
            &t.id.unwrap_or("Unable to determine ID".to_string()),
            &t.name,
            &t.priority,
            &t.description.unwrap_or_default(),
            &t.created_at.to_string(),
            &t.completed_at
                .map(|s| s.to_string())
                .unwrap_or("In progress".to_string()),
        ]);

    Ok(table.to_string())
}
