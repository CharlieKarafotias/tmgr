use surrealdb::sql::Datetime;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) priority: String,
    pub(crate) description: Option<String>,
    pub(crate) created_at: Datetime,
    pub(crate) completed_at: Option<Datetime>,
}
