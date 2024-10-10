use crate::commands::db::DB;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    // Check if record link exists to note file: https://surrealdb.com/docs/surrealql/datamodel/records
    // If not, create the note file
    // If it does, then open note file in default text editor
    todo!()
}
