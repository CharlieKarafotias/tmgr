use surrealdb::engine::any::connect;
use surrealdb::{engine::any::Any, Surreal};

pub(in crate::commands) struct DB {
    pub(in crate::commands) client: Surreal<Any>,
}

impl DB {
    pub(in crate::commands) async fn new() -> Self {
        // if env has testing flag, use memory db else use file db
        let client = if cfg!(test) {
            connect("mem://")
                .await
                .expect("Could not connect to memory database")
        } else {
            connect("file://tmgr_db")
                .await
                .expect("Could not connect to file database")
        };
        client
            .use_ns("tmgr_ns")
            .use_db("tmgr_db")
            .await
            .expect("Could not set namespace and database");
        Self { client }
    }
}
