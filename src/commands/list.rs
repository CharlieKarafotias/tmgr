use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(db: &DB, all: bool) -> Result<String, Box<dyn std::error::Error>> {
    #[allow(clippy::needless_late_init)]
    let tasks: Vec<Task>;
    if all {
        tasks = db.client.select("task").await?;
    } else {
        let query = "SELECT * FROM task WHERE completed_at IS None";
        tasks = db.client.query(query).await?.take(0).unwrap();
    };

    // TODO: add table implementation here instead of this
    Ok(tasks
        .into_iter()
        .map(|t| {
            format!(
                "ID: {:?}: {} - {} ({})",
                t.id,
                t.name,
                t.description.unwrap_or_default(),
                t.priority
            )
        })
        .collect::<Vec<String>>()
        .join("\n"))
}
