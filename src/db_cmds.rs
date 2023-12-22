// add, delete, list, set

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::persistent::{check_if_db_exists, mk_db, mkdir_db};

// TODO: update the task commands under db.rs to be able to read the correct db files
pub fn db_add(name: String) {
    let path = mkdir_db().expect("Failed to check/make db folder");
    // check if db exists
    if !check_if_db_exists(&path) {
        // create db if not
        mk_db(&path, &name);
        // TODO: set as active db in dotenv
        // notify user of success
        println!("Successfully created database with name: {}", name);
    } else {
        // notify user of error
        println!("The database by the name {} already exists", name);
    }
}

// TODO: finish these
pub fn db_delete(name: String) {
    let mut file_name = name.clone();
    file_name.push_str(".db");

    let mut path = env::current_dir().expect("Failed to get current directory");
    path = path.join("databases");
    path = path.join(file_name);
    // remove file at path if it is a file
    if check_if_db_exists(path.to_str().unwrap()) {
        fs::remove_file(path).expect("Failed to delete database");
        println!("Successfully deleted database");
    } else {
        println!("No database exists with the name: {}", name);
    }
}

pub fn db_list() {
    todo!()
}

pub fn db_set(name: String) {
    todo!()
}
