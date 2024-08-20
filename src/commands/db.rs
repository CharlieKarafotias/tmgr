use surrealdb::{engine::any::Any, Surreal};
pub(in crate::commands) struct DB {
    client: Surreal<Any>,
}

impl DB {
    pub(in crate::commands) fn new() -> Self {
        // if env has testing flag, use memory db else use file db
        todo!("DB not yet implemented")
    }
}
