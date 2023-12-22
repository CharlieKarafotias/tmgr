// persistent.rs provides an interface for managing the persistent storage required for tmgr
use std::{
    env,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
// -------------------- Functions for managing Databases --------------------
// Checks if db folder exists
fn check_db_dir(path: &Path) -> bool {
    path.is_dir()
}

// Checks if db file exists
pub fn check_if_db_exists(path: &str) -> bool {
    Path::new(path).is_file()
}

// Create db folder if it doesn't exist
pub fn mkdir_db() -> io::Result<String> {
    let mut path = env::current_dir()?;
    path = path.join("databases");
    if !check_db_dir(&path) {
        fs::create_dir(&path)?;
    }
    Ok(path
        .to_str()
        .expect("failed to convert path to string")
        .to_string())
}

pub fn mk_db(path: &str, name: &str) {
    // creating file name with the format {name}.db
    let mut file_name = name.to_string();
    file_name.push_str(".db");

    // create file path with route of '{curr_dir}/{file_name}'
    let mut new_file_path = PathBuf::new();
    new_file_path = new_file_path.join(path);
    new_file_path = new_file_path.join(file_name);

    // Create the new file
    File::create(new_file_path).expect("Error creating new database file");
}
//  -------------------- Functions for managing dotenv --------------------
// Updates dotenv's db_var to new database
fn change_db(db_name: String) {
    todo!()
}
