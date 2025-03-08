use super::super::{cli::model::TmgrVersion, db::DB};
use std::error::Error;

pub(crate) async fn run(
    db: &DB,
    previous_major_version: TmgrVersion,
) -> Result<String, Box<dyn Error>> {
    // Logic to get from V2 to V3 (Change: priority field must become TaskPriority value)
    // priority low -> Low
    let v2_fix_low = "UPDATE task SET priority = 'Low' WHERE priority = 'low'";
    // priority medium -> Medium
    let v2_fix_medium = "UPDATE task SET priority = 'Medium' WHERE priority = 'medium'";
    // priority high -> High
    let v2_fix_high = "UPDATE task SET priority = 'High' WHERE priority = 'high'";

    // NOTE: for future, I want to make this work as follows:
    // say previous major version is v2 and this is updated to v4
    // Should go from v2 -> v3 -> v4 in migration logic
    match previous_major_version {
        TmgrVersion::V2 => {
            db.client.query(v2_fix_low).await?;
            db.client.query(v2_fix_medium).await?;
            db.client.query(v2_fix_high).await?;
        }
    }

    Ok(format!(
        "Successfully migrated tasks from {} to v{} schema",
        String::from(&previous_major_version),
        env!("CARGO_PKG_VERSION")
            .split(".")
            .next()
            .expect("Failed to get major version"),
    ))
}
