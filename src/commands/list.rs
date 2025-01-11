use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use crate::commands::model::Task;
// TODO: update to use ratatui instead, then remove comfy-table dep
use comfy_table::Table;

pub(crate) async fn run(
    db: &DB,
    all: bool,
) -> Result<CommandResult<Vec<Task>>, Box<dyn std::error::Error>> {
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

    for t in &tasks {
        table.add_row(vec![
            t.get_id()?.as_str(),
            t.name.as_str(),
            t.priority.as_str(),
            t.description.as_deref().unwrap_or_default(),
            t.created_at.to_string().as_str(),
            t.completed_at
                .clone()
                .map(|s| s.to_string())
                .unwrap_or("In progress".to_string())
                .as_str(),
        ]);
    }

    Ok(CommandResult::new(table.to_string(), tasks))
}
