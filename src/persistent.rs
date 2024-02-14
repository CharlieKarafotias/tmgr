// persistent.rs provides an interface for managing the persistent storage required for tmgr
use std::{
    env,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, Read, Write},
    path::{Path, PathBuf},
};
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

/// Converts the given path to a database file path and returns it as a string.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database file.
///
/// # Returns
///
/// A string containing the database file path.
///
/// # Panics
///
/// * Panics if the current working directory value is invalid.
/// * Panics if the conversion to a string fails.
pub fn path_to_db(name: &str) -> String {
    let mut path = PathBuf::from(path_to_db_dir());
    let name_with_ext: String = name.to_string() + ".db";
    path = path.join(name_with_ext);
    path.to_str()
        .expect("failed to convert path to string")
        .to_string()
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
fn path_to_db_dir() -> String {
    let mut path = env::current_dir().expect("Failed to get current directory");
    path = path.join("databases");
    path.to_str()
        .expect("failed to convert path to string")
        .to_string()
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
fn check_if_db_exists(name: &str) -> bool {
    Path::new(&path_to_db(name)).exists()
}

/// Creates a directory for the database if it does not exist, and returns the path to the database directory.
///
/// # Returns
///
/// An io::Result containing the path to the database directory if successful, or an error if the operation fails.
fn mkdir_db() -> io::Result<bool> {
    fs::create_dir(path_to_db_dir())?;
    Ok(true)
}

/// Creates a new database file with the specified name in the database directory.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database file to be created.
///
/// # Panics
///
/// * Panics if database directory does not exist and fails to be created
/// * Panics if file creation fails
pub fn mk_db(name: &str) -> io::Result<bool> {
    // check if database directory exists; if not then make it
    if !check_db_dir(&path_to_db_dir()) {
        mkdir_db()?;
    }

    // Create the new file
    if !check_if_db_exists(name) {
        File::create(path_to_db(name))?;
        Ok(true)
    } else {
        Ok(false)
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
/// An io::Result containing a boolean value indicating whether the database was successfully removed or not.
///
/// # Panics
///
/// * Panics if file deletion fails
pub fn rm_db(name: &str) -> io::Result<bool> {
    // remove db file if it exists
    if check_if_db_exists(name) {
        fs::remove_file(path_to_db(name))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn list_dbs() -> Vec<String> {
    fs::read_dir(path_to_db_dir())
        .unwrap()
        .flatten()
        .map(|f| drop_file_extension(&f.file_name()).to_string())
        .collect()
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
    let mut path = env::current_dir().expect("Failed to get current directory");
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
    f.flush()?;
    Ok(true)
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
    }

    // write updated buffer to file
    // seek to the beginning of the file and write the updated buffer
    f.seek(std::io::SeekFrom::Start(0))?;
    f.write_all(buf.as_bytes())?;
    f.set_len(buf.len() as u64)?;
    f.flush()?;
    Ok(true)
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
pub fn change_db(db_name: &str) -> io::Result<bool> {
    // ensure the database exists. If not, return Ok(false)
    if Path::new(&path_to_db(db_name)).exists() {
        let env_path = path_to_env();

        // update the "db_var" env variable in the .env file to the db_name value
        // ensure the .env file exists. If not, create it
        if !Path::new(&env_path).exists() {
            mk_env()?;
            update_env_var("db_var", db_name)?;
        } else {
            // update the "db_var" env variable in the .env file to the db_name value
            update_env_var("db_var", db_name)?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}
