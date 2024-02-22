// TODO: idea: make a ENV struct that has function to update the .env file
// persistent.rs provides an interface for managing the persistent storage required for tmgr
use std::{
    env::{self},
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, Read, Write},
    path::{Path, PathBuf},
};

use super::db_errors;
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
fn drop_file_extension(os_str: &OsString) -> &str {
    if let Some(dot_index) = os_str.to_str().and_then(|s| s.rfind('.')) {
        &os_str.to_str().unwrap()[..dot_index]
    } else {
        os_str.to_str().unwrap()
    }
}
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

/// Checks if the database directory exists.
///
/// # Arguments
///
/// * `path` - A String containing the path to the database directory.
///
/// # Returns
///
/// A boolean value indicating whether the database directory exists or not.
fn check_db_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// Checks if the specified database exists in the database directory folder.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database file to be checked.
///
/// # Returns
///
/// A boolean value indicating whether the database file exists or not.
fn check_if_db_exists(name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let db_path = path_to_db(name)?;
    Ok(Path::new(&db_path).exists())
}

/// Creates a directory for the database if it does not exist.
///
/// # Returns
///
/// An Result<()> if successful, or an error if the operation fails.
fn mkdir_db() -> Result<(), Box<dyn std::error::Error>> {
    let db_dir = path_to_db_dir()?;
    fs::create_dir(db_dir)?;
    Ok(())
}

/// Creates a new database file with the specified name in the database directory.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database file to be created.
///
/// # Errors
///
/// * Errors if the database already exists.
pub fn mk_db(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // check if database directory exists; if not then make it
    let db_dir = path_to_db_dir()?;
    if !check_db_dir(&db_dir) {
        mkdir_db()?;
    }

    // Create the new file
    if !check_if_db_exists(name)? {
        let db_path = path_to_db(name)?;
        File::create(db_path)?;
        Ok(())
    } else {
        Err(db_errors::DatabaseError::new(
            &format!("Unable to create database {}", name),
            db_errors::DatabaseErrorKind::AlreadyExists,
        )
        .into())
    }
}

/// Removes the specified database from the database directory folder.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database file to be removed.
///
/// # Returns
///
/// An Result containing a boolean value indicating whether the database was successfully removed or not.
///
/// # Errors
///
/// * Errors if database does not exist.
pub fn rm_db(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // remove db file if it exists
    if check_if_db_exists(name)? {
        let db_path = path_to_db(name)?;
        fs::remove_file(db_path)?;
        Ok(())
    } else {
        Err(db_errors::DatabaseError::new(
            &format!("Unable to remove database {}", name),
            db_errors::DatabaseErrorKind::DoesNotExist,
        )
        .into())
    }
}

/// Retrieves a list of all database names in the database directory.
///
/// # Returns
///
/// A Result containing a vector of strings, representing the names of all the databases in the directory, if successful. If an error occurs, an Err variant containing an error type is returned.
///
/// # Errors
/// * Errors if there is an issue reading the database directory
/// * Errors if there is a problem converting the file names to strings.
pub fn list_dbs() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let db_dir = path_to_db_dir()?;
    Ok(fs::read_dir(db_dir)?
        .flatten()
        .map(|f| drop_file_extension(&f.file_name()).to_string())
        .collect())
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

/// Creates a new .env file and initializes it with the default database variable.
///
/// # Returns
///
/// Returns `Ok(true)` if the .env file is created and initialized successfully, and an `Err` if an I/O error occurs.
fn mk_env() -> io::Result<bool> {
    let mut f = File::create(path_to_env())?;
    f.write_all(b"db_var=none\n")?;
    f.write_all(b"db_dir=none\n")?;
    f.flush()?;
    Ok(true)
}

fn env_exists() -> bool {
    Path::new(&path_to_env()).exists()
}
/// Updates the specified environment variable in the .env file with a new value.
///
/// # Arguments
///
/// * `var_name` - A string slice that holds the name of the environment variable to be updated.
/// * `new_val` - A string slice that holds the new value for the environment variable.
///
/// # Returns
///
/// Returns `Ok(true)` if the environment variable is updated successfully, and an `Err` if an I/O error occurs.
fn update_env_var(var_name: &str, new_val: &str) -> io::Result<bool> {
    // checks if .env file exists, if not create it
    if !env_exists() {
        mk_env()?;
    }
    // open existing .env file in write mode and update the "db_var" variable
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path_to_env())?;

    // read the file
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;

    // Find the start position of the "var_name" variable (var_name=)
    if let Some(idx) = buf.find(var_name) {
        // Update the variable to new_value

        // Find the end position of the variable (\n)
        let idx_of_end_of_line = buf[idx..].find('\n').unwrap() + idx;

        // Update the variable with the new_value
        buf.replace_range(
            idx..idx_of_end_of_line,
            &format!("{}={}", var_name, new_val),
        );
    } else {
        // Add the variable to the file
        buf.push_str(&format!("{}={}\n", var_name, new_val));
    }

    // write updated buffer to file
    // seek to the beginning of the file and write the updated buffer
    f.seek(std::io::SeekFrom::Start(0))?;
    f.write_all(buf.as_bytes())?;
    f.set_len(buf.len() as u64)?;
    f.flush()?;
    Ok(true)
}

pub fn env_check_state() -> Result<(), Box<dyn std::error::Error>> {
    // Check if env exists; make if not
    if !env_exists() {
        mk_env()?;
    }
    dotenv::from_path(path_to_env())?;
    // check if db_dir is set
    if dotenv::var("db_dir").is_err() || dotenv::var("db_dir").unwrap() == "none" {
        return Err(db_errors::DatabaseError::new(
            "Environment State Error",
            db_errors::DatabaseErrorKind::DirectoryNotSet,
        )
        .into());
    }
    Ok(())
}

/// Updates the db_var variable in the .env file to the specified value.
///
/// # Arguments
///
/// * `db_name` - A string slice that holds the name of the database to be set.
///
/// # Returns
///
/// Returns `Ok(true)` if the database is updated successfully, `Ok(false)` if the database does not exist, and an `Err` if an I/O error occurs.
pub fn change_db(db_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ensure the database exists. If not, return Ok(false)
    let db_path = path_to_db(db_name)?;
    if Path::new(&db_path).exists() {
        // update the "db_var" env variable in the .env file to the db_name value
        update_env_var("db_var", db_name)?;
        Ok(())
    } else {
        Err(db_errors::DatabaseError::new(
            &format!("Unable to set database {}", db_name),
            db_errors::DatabaseErrorKind::DoesNotExist,
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir;

    #[test]
    fn test_drop_file_extension_with_extension() {
        let file_name = OsString::from("example.txt");
        assert_eq!(drop_file_extension(&file_name), "example");
    }

    #[test]
    fn test_drop_file_extension_without_extension() {
        let file_name = OsString::from("example");
        assert_eq!(drop_file_extension(&file_name), "example");
    }

    #[test]
    fn test_check_db_dir_existing_dir() {
        let temp_dir = tempdir::TempDir::new("test_db_dir").unwrap();
        let dir_path = temp_dir.path().to_str().unwrap().to_string();
        assert!(check_db_dir(&dir_path));
    }

    #[test]
    fn test_check_db_dir_non_existing_dir() {
        assert!(!check_db_dir("non_existing_dir"));
    }
}
