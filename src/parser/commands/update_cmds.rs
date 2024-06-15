mod request_models;
use std::{
    fs::{File, Permissions},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

use directories::UserDirs;
use request_models::GithubReleaseResponse;
use reqwest::header::USER_AGENT;
use rust_search::{FilterExt, SearchBuilder};
use semver::Version;

use crate::parser::state_mgr::State;

pub fn update(state: &State) {
    println!("Checking repository for updates...");

    let (needs_update, binary_download_url) = check_for_updates();
    if needs_update {
        if let Some(binary_download_url) = binary_download_url {
            println!("Update found, downloading...");
            download_binary_to_downloads_folder(binary_download_url);
            let path_to_exsting_executable = find_existing_executable(state);
            println!("Update complete");
        } else {
            println!("ERROR: No download url for the application found on GitHub");
        }
    } else {
        println!("Already on latest version");
    }
}

#[tokio::main]
async fn check_for_updates() -> (bool, Option<String>) {
    let repo_link = env!("CARGO_PKG_REPOSITORY");
    let url: Vec<&str> = repo_link.split('/').rev().collect();
    let repo = url[0];
    let github_account = url[1];
    let github_latest_release_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        github_account, repo
    );

    // get latest release from github
    let client = reqwest::Client::new();
    let res = client
        .get(github_latest_release_url)
        .header(USER_AGENT, "tmgr-rust")
        .send()
        .await;
    match res {
        Ok(res) => {
            let latest_release = res.json::<GithubReleaseResponse>().await.unwrap();
            let latest_version_github = Version::parse(&latest_release.tag_name.replace('v', ""))
                .expect("ERROR: unable to determine latest version");
            let current_version = Version::parse(env!("CARGO_PKG_VERSION"))
                .expect("ERROR: unable to determine current version");
            (
                latest_version_github > current_version,
                Some(latest_release.assets[0].browser_download_url.clone()),
            )
        }
        Err(_) => {
            println!("ERROR: Failed to get latest release from GitHub");
            (false, None)
        }
    }
}

#[tokio::main]
async fn download_binary_to_downloads_folder(binary_download_url: String) {
    let download_dir = UserDirs::new();
    match download_dir {
        Some(user_dirs) => {
            let download_dir_path = user_dirs
                .download_dir()
                .expect("ERROR: Unable to determine download directory");
            let client = reqwest::Client::new();
            let res = client
                .get(binary_download_url)
                .header(USER_AGENT, "tmgr-rust")
                .send()
                .await;
            let bytes = res
                .expect("ERROR: Failed to download binary")
                .bytes()
                .await
                .expect("ERROR: Failed to download binary");
            let full_path = PathBuf::from(download_dir_path).join("tmgr_new");
            let mut f = File::create(full_path).expect("ERROR: Failed to create file");
            f.write_all(&bytes).expect("ERROR: Failed to write to file");
            f.set_permissions(Permissions::from_mode(0o751)).unwrap();
        }
        None => println!("ERROR: Unable to determine system's file structure"),
    }
}

// TODO: convert all options to results
fn find_existing_executable(state: &State) -> Option<String> {
    // Path to existing state manager (stored at same place as executable)
    let mut existing_executable_path = PathBuf::from(state.get_path());
    existing_executable_path.pop();
    // Search for tmgr in the same directory
    let search: Vec<String> = SearchBuilder::default()
        .search_input("tmgr")
        .location(existing_executable_path)
        .custom_filter(|dir| dir.metadata().unwrap().is_file())
        .strict()
        .build()
        .collect();
    if !search.is_empty() {
        Some(search[0].clone())
    } else {
        println!("ERROR: Unable to find existing executable");
        None
    }
}

fn delete_existing_binary(existing_binary_path: PathBuf) {
    todo!("delete existing binary");
}

fn move_new_binary_and_delete_old(existing_binary_path: PathBuf, new_binary_path: PathBuf) {
    todo!("move new binary and delete old");
}
