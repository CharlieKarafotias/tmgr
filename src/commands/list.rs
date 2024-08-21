use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(db: &DB, all: bool) {
    // TODO: implement all part vs in progress ones
    #[allow(clippy::needless_late_init)]
    let tasks: Vec<Task>;
    if all {
        tasks = db
            .client
            .select("task")
            .await
            .expect("Could not select tasks");
    } else {
        let query = "SELECT * FROM task WHERE completed_at IS None";
        tasks = db
            .client
            .query(query)
            .await
            .expect("Could not select tasks")
            .take(0)
            .unwrap();
    };

    println!("{:?}", tasks);
}
