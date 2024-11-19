use crate::commands::db::DB;
use crate::commands::model::Task;
use comfy_table::Table;

pub(crate) async fn run(db: &DB, all: bool) -> Result<String, Box<dyn std::error::Error>> {
    let tasks: Vec<Task> = if all {
        db.client.select("task").await?
    } else {
        let query = "SELECT * FROM task WHERE completed_at IS None";
        db.client.query(query).await?.take(0).unwrap()
    };

    let mut table = Table::new();
    table
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec![
            "id",
            "name",
            "priority",
            "description",
            "created_at",
            "completed_at",
        ]);

    for t in tasks {
        table.add_row(vec![
            &t.get_id()?,
            &t.name,
            &t.priority,
            &t.description.unwrap_or_default(),
            &t.created_at.to_string(),
            &t.completed_at
                .map(|s| s.to_string())
                .unwrap_or("In progress".to_string()),
        ]);
    }

    Ok(table.to_string())
}
