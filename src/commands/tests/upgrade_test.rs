use crate::commands::upgrade::{delete_existing_binary, latest_release_url, move_new_binary};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn latest_release_url_test() {
    let expected = "https://api.github.com/repos/charliekarafotias/tmgr/releases/latest";
    let actual = latest_release_url();
    assert_eq!(
        expected, actual,
        "latest_release_url() should return the correct URL"
    );
}

#[test]
fn delete_existing_binary_test() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let file_path = temp_dir.path().join("fake_file");
    let mut file = File::create(&file_path).expect("Failed to create temporary file");
    file.write_all(b"This is a fake file")
        .expect("Failed to write to temporary file");

    delete_existing_binary(&file_path).expect("delete_existing_binary should return Ok");
    assert!(!file_path.try_exists().expect("file should be deleted"));
}

#[test]
fn move_new_binary_test() {
    let temp_dir_old = tempdir().expect("Failed to create temporary directory");
    let temp_dir_new = tempdir().expect("Failed to create temporary directory");
    let file_path_old = temp_dir_old.path().join("fake_old");
    let file_path_new = temp_dir_new.path().join("fake_new");
    let mut file = File::create(&file_path_old).expect("Failed to create temporary file");
    file.write_all(b"This is a fake file")
        .expect("Failed to write to temporary file");

    move_new_binary(file_path_old.clone(), file_path_new.clone())
        .expect("move_new_binary should return Ok");
    assert!(!file_path_old.try_exists().expect("file should be deleted"));
    assert!(file_path_new.try_exists().expect("should exist"));
}
