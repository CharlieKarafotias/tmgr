use super::{db::DB, model::Task};
use comfy_table::{ContentArrangement::Dynamic, Table};
use std::error::Error;

pub(crate) async fn run(db: &DB, all: bool) -> Result<String, Box<dyn Error>> {
    let tasks: Vec<Task> = if all {
        db.client.select("task").await?
    } else {
        let query = "SELECT * FROM task WHERE completed_at IS None";
        db.client.query(query).await?.take(0).unwrap()
    };

    let mut table = Table::new();
    table.set_content_arrangement(Dynamic).set_header(vec![
        "id",
        "name",
        "priority",
        "description",
        "created_at",
        "completed_at",
    ]);

    // TODO: should have a function implemented for task that allows returning of these values
    // can also filter out specific fields
    tasks.iter().for_each(|t| {
        table.add_row(vec![
            t.id().unwrap_or("ID not found".to_string()).as_str(),
            t.name(),
            t.priority(),
            t.description().as_ref().unwrap_or(&"None".to_string()),
            t.created_at().to_string().as_str(),
            t.completed_at()
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or("In Progress".to_string())
                .as_str(),
        ]);
    });

    Ok(table.to_string())
}
