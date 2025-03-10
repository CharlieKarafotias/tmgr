use super::super::{
    cli::model::TmgrVersion,
    db::DB,
    model::{CommandResult, TmgrError, TmgrErrorKind},
};
use std::fmt;

pub(crate) async fn run(
    db: &DB,
    previous_major_version: TmgrVersion,
) -> Result<CommandResult<bool>, MigrateError> {
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
            db.client
                .query(v2_fix_low)
                .await
                .map_err(|_| MigrateError {
                    kind: MigrateErrorKind::DatabaseError,
                    message: "Failed to convert low priority v2 tasks to v3".to_string(),
                })?;
            db.client
                .query(v2_fix_medium)
                .await
                .map_err(|_| MigrateError {
                    kind: MigrateErrorKind::DatabaseError,
                    message: "Failed to convert medium priority v2 tasks to v3".to_string(),
                })?;
            db.client
                .query(v2_fix_high)
                .await
                .map_err(|_| MigrateError {
                    kind: MigrateErrorKind::DatabaseError,
                    message: "Failed to convert high priority v2 tasks to v3".to_string(),
                })?;
        }
        TmgrVersion::V3 => (), // latest version - do nothing
        TmgrVersion::Invalid => Err(MigrateError {
            kind: MigrateErrorKind::UnableToGetTmgrVersion,
            message: "Unable to determine tmgr current version".to_string(),
        })?,
    }

    let current_major_version =
        env!("CARGO_PKG_VERSION")
            .split(".")
            .next()
            .ok_or(MigrateError {
                kind: MigrateErrorKind::UnableToGetTmgrVersion,
                message: "Unable to determine tmgr current version".to_string(),
            })?;

    Ok(CommandResult::new(
        format!(
            "Successfully migrated tasks from {} to v{} schema",
            String::from(&previous_major_version),
            current_major_version,
        ),
        true,
    ))
}

// --- Migrate Errors ---
#[derive(Debug)]
pub enum MigrateErrorKind {
    DatabaseError,
    UnableToGetTmgrVersion,
}

#[derive(Debug)]
pub struct MigrateError {
    kind: MigrateErrorKind,
    message: String,
}

impl fmt::Display for MigrateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (migrate error: {})", self.message, self.kind)
    }
}

impl fmt::Display for MigrateErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MigrateErrorKind::DatabaseError => write!(f, "Database error"),
            MigrateErrorKind::UnableToGetTmgrVersion => write!(f, "Unable to get tmgr version"),
        }
    }
}

impl From<MigrateError> for TmgrError {
    fn from(err: MigrateError) -> Self {
        TmgrError::new(TmgrErrorKind::MigrateCommand, err.to_string())
    }
}
