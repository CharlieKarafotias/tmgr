use std::{error, fmt};

#[derive(Debug)]
pub enum DatabaseErrorKind {
    DatabaseAlreadyExists,
    DatabaseDoesNotExist,
}

#[derive(Debug)]
pub struct DatabaseError {
    kind: DatabaseErrorKind,
    message: String,
}

impl fmt::Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseErrorKind::DatabaseAlreadyExists => write!(f, "database already exists"),
            DatabaseErrorKind::DatabaseDoesNotExist => write!(f, "database does not exists"),
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
