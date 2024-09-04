use crate::commands::db::DB;

pub(crate) async fn run(db: &DB, name: String) -> Result<String, Box<dyn std::error::Error>> {
    // ~ compares 2 values for equality using fuzzy match
    // https://surrealdb.com/docs/surrealdb/surrealql/operators#match
    let query = format!(
        "UPDATE task SET completed_at = time::now() WHERE completed_at IS None AND name ~ '{}'",
        name
    );
    // TODO: Should add a check; if 2 tasks are close, give user the option to choose
    db.client.query(&query).await?;

    // TODO: if no task was updated, it would be nice to have different message
    Ok("Successfully updated task to completed".to_string())
}
