use std::fmt;

use db_cmds::DatabaseError;
use status_cmds::StatusError;
use todo_cmds::TodoError;
use update_cmds::UpdateError;

pub mod db_cmds;
pub mod status_cmds;
pub mod todo_cmds;
pub mod update_cmds;

pub type TmgrResult<T> = std::result::Result<T, TmgrError>;

#[derive(Debug)]
pub enum TmgrError {
    Database(DatabaseError),
    Status(StatusError),
    Todo(TodoError),
    Update(UpdateError),
}

impl From<DatabaseError> for TmgrError {
    fn from(e: DatabaseError) -> Self {
        TmgrError::Database(e)
    }
}

impl From<StatusError> for TmgrError {
    fn from(e: StatusError) -> Self {
        TmgrError::Status(e)
    }
}

impl From<TodoError> for TmgrError {
    fn from(e: TodoError) -> Self {
        TmgrError::Todo(e)
    }
}

impl From<UpdateError> for TmgrError {
    fn from(e: UpdateError) -> Self {
        TmgrError::Update(e)
    }
}

impl fmt::Display for TmgrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TmgrError::Database(e) => e.fmt(f),
            TmgrError::Status(e) => e.fmt(f),
            TmgrError::Todo(e) => e.fmt(f),
            TmgrError::Update(e) => e.fmt(f),
        }
    }
}
