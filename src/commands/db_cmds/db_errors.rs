use std::{error, fmt};

#[derive(Debug)]
pub enum DatabaseErrorKind {
    AlreadyExists,
    DoesNotExist,
    VariableNotSet,
    DirectoryNotSet,
    PathCreationFailed,
}

#[derive(Debug)]
pub struct DatabaseError {
    kind: DatabaseErrorKind,
    message: String,
}

impl fmt::Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseErrorKind::AlreadyExists => write!(
                f,
                "database already exists hint: run `tmgr database set <name>`"
            ),
            DatabaseErrorKind::DoesNotExist => write!(
                f,
                "database does not exists hint: run `tmgr database add <name>`"
            ),
            DatabaseErrorKind::VariableNotSet => write!(
                f,
                "database variable not set, hint: run `tmgr database set <name>`"
            ),
            DatabaseErrorKind::DirectoryNotSet => write!(
                f,
                "database directory not set, hint: run `tmgr database set-directory <dir>`"
            ),
            DatabaseErrorKind::PathCreationFailed => write!(
                f,
                "path to current database file could not be created, hint: run `tmgr database set-directory <dir>` or `tmgr database set <name>`"
            ),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (db error: {})", self.message, self.kind)
    }
}

impl DatabaseError {
    pub fn new(message: &str, kind: DatabaseErrorKind) -> DatabaseError {
        DatabaseError {
            kind,
            message: message.to_string(),
        }
    }
}

impl error::Error for DatabaseError {}
