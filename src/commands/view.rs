use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: For some reason this query doesn't work. Looks like query0 returns QueryNotExecuted
    // see: https://docs.rs/surrealdb/latest/surrealdb/error/enum.Db.html#variant.QueryNotExecuted
    let query = format!(
        "BEGIN TRANSACTION;\
        let $res = (SELECT * from task WHERE string::starts_with(<string> id, \"task:{id}\"));\
        IF (count($res) == 0) {{ THROW \"Task starting with id '{id}' was not found\"}};\
        IF (count($res) != 1) {{ THROW \"Multiple tasks found, provide more characters of the id\"}};\
        RETURN $res[0];\
        COMMIT TRANSACTION;"
    );
    let mut db_res = db.client.query(query).await?;
    let errors = db_res.take_errors();
    dbg!(&errors);
    let err = errors
        .iter()
        .find(|(_, e)| e.to_string().starts_with("An error occurred:"));
    if let Some((_, e)) = err {
        let err = e.to_string().replace("An error occurred: ", "");
        return Err(err.into());
    }

    let task: Option<Task> = db_res.take(0)?;
    if let Some(task) = task {
        Ok(task.to_string())
    } else {
        Err(format!("Task starting with id '{id}' was not found SECONDARY").into())
    }
    // let task: Option<Task> = db_res.take(0)?;
    // if let Some(task) = task {
    //     Ok(task.to_string())
    // } else {
    //     let errors = db_res.take_errors();
    //     dbg!(&errors);
    //     if !errors.is_empty() {
    //         let err = errors
    //             .iter()
    //             .find(|(_, e)| e.to_string().starts_with("An error occurred:"))
    //             .map(|(_, e)| e.to_string().replace("An error occurred: ", ""))
    //             .unwrap_or_else(|| "An unknown error occurred".to_string());
    //         Err(err.into())
    //     } else {
    //         Err("How did you get here".into())
    //     }
    // }
}
