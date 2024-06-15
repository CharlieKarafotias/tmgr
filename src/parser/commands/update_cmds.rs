mod request_models;
use std::{
    fmt,
    fs::{self, File, Permissions},
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

struct UpdateInfo {
    needs_update: bool,
    binary_download_url: String,
}

/// Updates the current executable to the latest version
pub fn update(state: &State) {
    println!("Checking repository for updates...");
    match check_for_updates() {
        Ok(UpdateInfo {
            needs_update,
            binary_download_url,
        }) => {
            if needs_update {
                match download_binary_to_downloads_folder(binary_download_url) {
                    Ok(_) => {
                        todo!("Implement update logic");
                        //     if let Some(path_to_existing_executable) = find_existing_executable(state) {
                        //         delete_existing_binary(path_to_existing_executable.as_str());
                        //         // update downloaded binary name from tmgr_new to tmgr
                        //         let mut new_binary_path = PathBuf::from(path_to_existing_executable);
                        //         new_binary_path.set_file_name("tmgr");
                        //         move_new_binary(new_binary_download_path, new_binary_path);
                        //     }
                        //     println!("Update complete");
                        // } else {
                        //     println!("ERROR: No download url for the application found on GitHub");
                        // }
                    }
                    Err(e) => println!("ERROR: {}", e),
                }
            } else {
                println!("Already on latest version");
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
}

#[tokio::main]
async fn check_for_updates() -> Result<UpdateInfo, UpdateError> {
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
        .await
        .map_err(|e| UpdateError {
            message: e.to_string(),
            kind: UpdateErrorKind::RepoCheckFail,
        })?;

    // convert response to struct
    let latest_release = res
        .json::<GithubReleaseResponse>()
        .await
        .map_err(|e| UpdateError {
            message: e.to_string(),
            kind: UpdateErrorKind::GitHibResponseToRustStructConversionFail,
        })?;

    // parse latest version
    let latest_version_github =
        Version::parse(&latest_release.tag_name.replace('v', "")).map_err(|e| UpdateError {
            message: e.to_string(),
            kind: UpdateErrorKind::NoLatestVersion,
        })?;

    // parse current version
    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|e| UpdateError {
        message: e.to_string(),
        kind: UpdateErrorKind::NoCurrentVersion,
    })?;

    if !latest_release.assets.is_empty() {
        Ok(UpdateInfo {
            needs_update: latest_version_github > current_version,
            binary_download_url: latest_release.assets[0].browser_download_url.clone(),
        })
    } else {
        Err(UpdateError {
            message: "no assets found".to_string(),
            kind: UpdateErrorKind::NoDownloadLinkFromGitHub,
        })
    }
}

#[tokio::main]
async fn download_binary_to_downloads_folder(
    binary_download_url: String,
) -> Result<PathBuf, UpdateError> {
    // get download directory of the current system
    let system_file_structure = UserDirs::new().ok_or(UpdateError {
        message: "ERROR: Unable to determine system's file structure".to_string(),
        kind: UpdateErrorKind::UnableToDetermineFileStructure,
    })?;
    let download_dir_path = system_file_structure.download_dir().ok_or(UpdateError {
        message: "ERROR: Unable to determine download directory".to_string(),
        kind: UpdateErrorKind::UnableToDetermineFileStructure,
    })?;

    // download binary from github
    let client = reqwest::Client::new();
    let res = client
        .get(binary_download_url)
        .header(USER_AGENT, "tmgr-rust")
        .send()
        .await
        .map_err(|e| UpdateError {
            message: e.to_string(),
            kind: UpdateErrorKind::BinaryDownloadFail,
        })?;

    // convert response to bytes
    let bytes = res.bytes().await.map_err(|e| UpdateError {
        message: e.to_string(),
        kind: UpdateErrorKind::CorruptedBinaryDownload,
    })?;

    // write bytes to new file called tmgr_new in Downloads folder
    let full_path = PathBuf::from(download_dir_path).join("tmgr_new");
    let mut f = File::create(&full_path).map_err(|e| UpdateError {
        message: e.to_string(),
        kind: UpdateErrorKind::CreateFileFail,
    })?;
    f.write_all(&bytes).map_err(|e| UpdateError {
        message: e.to_string(),
        kind: UpdateErrorKind::CreateFileFail,
    })?;

    // make new file executable
    f.set_permissions(Permissions::from_mode(0o751))
        .map_err(|e| UpdateError {
            message: e.to_string(),
            kind: UpdateErrorKind::CreateFileFail,
        })?;
    Ok(full_path)
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

fn delete_existing_binary(existing_binary_path: &str) {
    fs::remove_file(existing_binary_path).expect("ERROR: Failed to delete existing binary");
}

fn move_new_binary(existing_binary_path: PathBuf, new_binary_path: PathBuf) {
    let _ = fs::rename(existing_binary_path, new_binary_path);
}

#[derive(Debug)]
struct UpdateError {
    kind: UpdateErrorKind,
    message: String,
}

#[derive(Debug)]
enum UpdateErrorKind {
    RepoCheckFail,
    NoDownloadLinkFromGitHub,
    NoCurrentVersion,
    NoLatestVersion,
    GitHibResponseToRustStructConversionFail,
    UnableToDetermineFileStructure,
    BinaryDownloadFail,
    CorruptedBinaryDownload,
    CreateFileFail,
}

impl fmt::Display for UpdateErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateErrorKind::RepoCheckFail => write!(
                f,
                "unable to retrieve fetch tmgr GitHub repo, try again later"
            ),
            UpdateErrorKind::NoDownloadLinkFromGitHub => {
                write!(f, "no download url for the tmgr executable found on GitHub")
            }
            UpdateErrorKind::NoCurrentVersion => {
                write!(f, "unable to determine current version of tmgr")
            }
            UpdateErrorKind::NoLatestVersion => {
                write!(f, "unable to determine latest version of tmgr from GitHub")
            }
            UpdateErrorKind::GitHibResponseToRustStructConversionFail => {
                write!(f, "unable to convert GitHub response to Rust struct")
            }
            UpdateErrorKind::UnableToDetermineFileStructure => {
                write!(f, "Unable to determine system's file structure")
            }
            UpdateErrorKind::BinaryDownloadFail => write!(f, "unable to retrieve fetch tmgr the latest executable from GitHub repo, try again later"),
            UpdateErrorKind::CorruptedBinaryDownload => write!(f, "unable to convert downloaded executable to bytes"),
            UpdateErrorKind::CreateFileFail => write!(f, "unable to create file in downloads folder"),
        }
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (update error: {})", self.message, self.kind)
    }
}
