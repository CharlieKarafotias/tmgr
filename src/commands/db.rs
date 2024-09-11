use std::path::PathBuf;
use surrealdb::engine::any::connect;
use surrealdb::{engine::any::Any, Surreal};

pub(crate) struct DB {
    pub(crate) client: Surreal<Any>,
}

impl DB {
    pub(crate) async fn new() -> Self {
        let client = connect(format!("file://{}", Self::get_db_file_path().display()))
            .await
            .expect("Could not create/connect to file database");
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

    pub(crate) fn get_db_file_path() -> PathBuf {
        let exe_path = std::env::current_exe().expect("Could not get executable path");
        let dir_path = exe_path
            .parent()
            .expect("Could not get executable directory");
        dir_path.join("tmgr_db")
    }
}
