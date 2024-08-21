use surrealdb::engine::any::connect;
use surrealdb::{engine::any::Any, Surreal};

pub(crate) struct DB {
    pub(crate) client: Surreal<Any>,
}

impl DB {
    pub(crate) async fn new() -> Self {
        // if env has testing flag, use memory db else use file db
        let client = connect("file://tmgr_db")
            .await
            .expect("Could not connect to file database");
        client
            .use_ns("tmgr_ns")
            .use_db("tmgr_db")
            .await
            .expect("Could not set namespace and database");
        Self { client }
    }

    pub(crate) async fn new_test() -> Self {
        let client = connect("mem://")
            .await
            .expect("Could not connect to memory database");
        client
            .use_ns("tmgr_ns")
            .use_db("tmgr_db")
            .await
            .expect("Could not set namespace and database");
        Self { client }
    }
}
