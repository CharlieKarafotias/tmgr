#[allow(unused_imports)]
use crate::commands::{complete, db};
#[tokio::test]
async fn given_no_existing_tasks_when_completing_a_task_then_no_task_should_be_completed() {
    let db = db::DB::new_test().await;
    let res = complete::run(&db, "test".to_string()).await;
    assert!(res.is_ok());
    let res_str = res.unwrap();
    // TODO: check that response of no task updated is returned - when implemented
    assert_eq!(
        res_str,
        "Successfully updated task to completed".to_string()
    );
}
