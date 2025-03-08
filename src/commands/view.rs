use super::db::DB;
use comfy_table::{ContentArrangement::Dynamic, Table};
use std::error::Error;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn Error>> {
    let t = db.select_task_by_partial_id(&id).await?;

    // TODO: refactor with List command (see todo comment in list function)
    let mut table = Table::new();
    table
        .set_content_arrangement(Dynamic)
        .set_header(vec!["Key", "Value"])
        .add_row(vec!["id", &t.id()?])
        .add_row(vec!["name", t.name()])
        .add_row(vec!["priority", t.priority()])
        .add_row(vec![
            "description",
            t.description().as_ref().unwrap_or(&"None".to_string()),
        ])
        .add_row(vec![
            "work_note_path",
            t.work_note_path().as_ref().unwrap_or(&"None".to_string()),
        ])
        .add_row(vec!["created_at", t.created_at().to_string().as_str()])
        .add_row(vec![
            "completed_at",
            t.completed_at()
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or("In Progress".to_string())
                .as_str(),
        ]);

    Ok(table.to_string())
}
