use crate::cli::model::TaskPriority;
use crate::commands::db::DB;

pub(crate) async fn run(
    db: &DB,
    id: String,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    if name.is_none() && priority.is_none() && description.is_none() {
        return Err("No fields to update".into());
    }

    let query = format!(
        "BEGIN TRANSACTION;\
        let $res = (SELECT * from task WHERE string::starts_with(<string> id, \"task:{id}\"));\
        IF count($res) == 0 {{ THROW \"Task starting with id '{id}' was not found\"}};\
        IF count($res) != 1 {{ THROW \"Multiple tasks found, provide more characters of the id\"}};\
        let $task_to_update = $res[0];\
        UPDATE $task_to_update.id SET {};\
        COMMIT TRANSACTION;",
        generate_update_query(name, priority, description)
    );

    let mut db_res = db.client.query(query).await?;
    let errors = db_res.take_errors();
    if !errors.is_empty() {
        let err = errors
            .iter()
            .find(|(_, e)| e.to_string().starts_with("An error occurred:"))
            .map(|(_, e)| e.to_string().replace("An error occurred: ", ""))
            .unwrap_or_else(|| "An unknown error occurred".to_string());
        return Err(err.into());
    }

    Ok(format!("Successfully updated task starting with id '{id}'"))
}

pub(crate) fn generate_update_query(
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) -> String {
    let mut query = String::new();
    if let Some(name) = name {
        query += &format!("name = \"{name}\", ");
    }
    if let Some(priority) = priority {
        query += &format!("priority = \"{priority}\", ");
    }
    if let Some(description) = description {
        query += &format!("description = \"{description}\", ");
    }
    query = query.trim_end_matches(&[',', ' ']).to_string();
    query
}
