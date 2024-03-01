// persistent.rs provides an interface for managing the persistent storage required for tmgr
use std::{
    env, io,
    path::{Path, PathBuf},
};

use dotenv;
// -------------------- Functions for managing file space --------------------

/// Given an OsString, this function returns a string slice representing the file name without its extension.
/// If the OsString contains a file extension, the function removes the extension and returns the file name.
/// If the OsString does not contain a file extension, the function returns the original file name.
///
/// # Arguments
///
/// * `os_str` - An OsString containing the file name with or without an extension.
///
/// # Returns
///
/// A string slice representing the file name without its extension.
// pub fn drop_file_extension(os_str: &OsString) -> &str {
//     if let Some(dot_index) = os_str.to_str().and_then(|s| s.rfind('.')) {
//         &os_str.to_str().unwrap()[..dot_index]
//     } else {
//         os_str.to_str().unwrap()
//     }
// }
// -------------------- Functions for managing Databases --------------------

/// Provides the path to a database file in the database directory.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database.
///
/// # Returns
///
/// A string containing the file path of the database file.
///
/// # Errors
///
/// Errors if the conversion to a string fails.
///
/// # Examples
///
/// ```
/// use your_crate_name::path_to_db;
///
/// // Returns the path to the database file representing the database "my_db"
/// let db_path = path_to_db("my_db").unwrap();
/// ```
pub fn path_to_db(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(path_to_db_dir()?);
    let name_with_ext: String = name.to_string() + ".db";
    path = path.join(name_with_ext);
    if let Some(path_str) = path.to_str() {
        return Ok(path_str.to_string());
    }
    Err(Box::new(io::Error::new(
        io::ErrorKind::Other,
        "failed to convert path to string",
    )))
}

/// Returns the path to the directory containing the database files.
///
/// # Returns
///
/// A string containing the path to the directory of the database files.
///
/// # Panics
///
/// * Panics if the current working directory value is invalid.
/// * Panics if the conversion to a string fails.
fn path_to_db_dir() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::from_path(path_to_env())?;
    let db_dir = dotenv::var("db_dir")?;
    let path = Path::new(&db_dir).join("databases");

    if let Some(path_str) = path.to_str() {
        return Ok(path_str.to_string());
    }
    Err(Box::new(io::Error::new(
        io::ErrorKind::Other,
        "failed to convert path to string",
    )))
}

//  -------------------- Functions for managing dotenv --------------------

/// Returns the path to the .env file as a String.
///
/// # Returns
///
/// A string containing the path to the .env file.
///
/// # Panics
///
/// * Panics if the current working directory value is invalid
/// * Panics if the conversion to a string fails.
pub fn path_to_env() -> String {
    let mut path = env::current_exe().expect("ERROR: Unable to resolve executable directory");
    path.pop(); // remove the tmgr part of the path
    path = path.join(".env");
    path.to_str()
        .expect("failed to convert path to string")
        .to_string()
}
