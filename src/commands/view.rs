use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use crate::commands::model::Task;
use comfy_table::Table;

pub(crate) async fn run(
    db: &DB,
    id: String,
) -> Result<CommandResult<Task>, Box<dyn std::error::Error>> {
    let t = db.select_task_by_partial_id(&id).await?;

    let mut table = Table::new();
    table
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["Key", "Value"])
        .add_row(vec!["id", t.get_id()?.as_str()])
        .add_row(vec!["name", t.name.as_str()])
        .add_row(vec!["priority", t.priority.as_str()])
        .add_row(vec![
            "description",
            t.description.as_deref().unwrap_or_default(),
        ])
        .add_row(vec![
            "work_note_path",
            t.work_note_path.as_deref().unwrap_or_default(),
        ])
        .add_row(vec!["created_at", t.created_at.to_string().as_str()])
        .add_row(vec![
            "completed_at",
            t.completed_at
                .clone()
                .map(|s| s.to_string())
                .unwrap_or("In progress".to_string())
                .as_str(),
        ]);

    Ok(CommandResult::new(table.to_string(), t))
}
