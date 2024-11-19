use crate::commands::db::DB;
use comfy_table::Table;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let t = db.select_task_by_partial_id(&id).await?;

    let mut table = Table::new();
    table
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["Key", "Value"])
        .add_row(vec!["id", &t.get_id()?])
        .add_row(vec!["name", &t.name])
        .add_row(vec!["priority", &t.priority])
        .add_row(vec!["description", &t.description.unwrap_or_default()])
        .add_row(vec![
            "work_note_path",
            &t.work_note_path.unwrap_or_default(),
        ])
        .add_row(vec!["created_at", &t.created_at.to_string()])
        .add_row(vec![
            "completed_at",
            &t.completed_at
                .map(|s| s.to_string())
                .unwrap_or("In progress".to_string()),
        ]);

    Ok(table.to_string())
}
