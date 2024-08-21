use crate::commands::db::DB;
use crate::commands::model::Task;

#[tokio::main]
pub(crate) async fn run(all: bool) {
    // TODO: implement all part vs in progress ones
    let db = DB::new().await;
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
