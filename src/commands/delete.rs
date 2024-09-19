use crate::commands::db::DB;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        "BEGIN TRANSACTION;\
        let $res = (SELECT * from task WHERE string::starts_with(<string> id, \"task:{id}\"));\
        IF count($res) == 0 {{ THROW \"Task starting with id '{id}' was not found\"}};\
        IF count($res) != 1 {{ THROW \"Multiple tasks found, provide more characters of the id\"}};\
        let $task_to_delete = $res[0];\
        DELETE $task_to_delete.id;\
        COMMIT TRANSACTION;"
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

    Ok(format!("Successfully deleted task starting with id '{id}'"))
}
