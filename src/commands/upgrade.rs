use directories::UserDirs;
use reqwest::header::USER_AGENT;
use semver::Version;
use serde::Deserialize;
use std::{
    env::current_exe,
    error::Error,
    fmt, fs,
    fs::{File, Permissions},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

pub(crate) async fn run() -> Result<String, Box<dyn Error>> {
    println!("Checking repository for updates...");
    let update_info = check_for_updates().await.map_err(|e| e.to_string())?;

    if update_info.needs_update {
        let new_binary_download_path =
            download_binary_to_downloads_folder(update_info.binary_download_url)
                .await
                .map_err(|e| e.to_string())?;
        let path_to_existing_executable = current_exe()?;
        delete_existing_binary(&path_to_existing_executable).map_err(|e| e.to_string())?;
        // move new binary from download folder to bin of current executable
        move_new_binary(new_binary_download_path, path_to_existing_executable)
            .map_err(|e| e.to_string())?;
        Ok(format!(
            "Update complete: v{} -> v{}",
            update_info.current_version, update_info.latest_version
        ))
    } else {
        Ok("Already on latest version".to_string())
    }
}

async fn check_for_updates() -> Result<UpdateInfo, UpdateError> {
    // get latest release from GitHub
    let client = reqwest::Client::new();
    let res = client
        .get(latest_release_url())
        .header(USER_AGENT, "tmgr-rust")
        .send()
        .await
        .map_err(|e| UpdateError {
            message: e.to_string(),
            kind: UpdateErrorKind::RepoCheckFail,
        })?;
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
            current_version: current_version.to_string(),
            needs_update: latest_version_github > current_version,
            binary_download_url: latest_release.assets[0].browser_download_url.clone(),
            latest_version: latest_version_github.to_string(),
        })
    } else {
        Err(UpdateError {
            message: "no assets found".to_string(),
            kind: UpdateErrorKind::NoDownloadLinkFromGitHub,
        })
    }
}

pub(super) fn latest_release_url() -> String {
    let repo_link = env!("CARGO_PKG_REPOSITORY");
    let url: Vec<&str> = repo_link.split('/').rev().collect();
    let repo = url[0];
    let github_account = url[1];
    format!("https://api.github.com/repos/{github_account}/{repo}/releases/latest",)
}

async fn download_binary_to_downloads_folder(
    binary_download_url: String,
) -> Result<PathBuf, UpdateError> {
    // get download directory of the current system
    let system_file_structure = UserDirs::new().ok_or(UpdateError {
        message: "Unable to determine system's file structure".to_string(),
        kind: UpdateErrorKind::UnableToDetermineFileStructure,
    })?;
    let download_dir_path = system_file_structure.download_dir().ok_or(UpdateError {
        message: "Unable to determine download directory".to_string(),
        kind: UpdateErrorKind::UnableToDetermineFileStructure,
    })?;

    // download binary from GitHUb
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

pub(super) fn delete_existing_binary(existing_binary_path: &PathBuf) -> Result<(), UpdateError> {
    fs::remove_file(existing_binary_path).map_err(|e| UpdateError {
        message: e.to_string(),
        kind: UpdateErrorKind::UnableToDeleteExistingBinary,
    })
}

pub(super) fn move_new_binary(
    existing_binary_path: PathBuf,
    new_binary_path: PathBuf,
) -> Result<(), UpdateError> {
    fs::rename(existing_binary_path, new_binary_path).map_err(|e| UpdateError {
        message: e.to_string(),
        kind: UpdateErrorKind::UnableToMoveBinary,
    })
}

struct UpdateInfo {
    binary_download_url: String,
    latest_version: String,
    needs_update: bool,
    current_version: String,
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
    UnableToDeleteExistingBinary,
    UnableToMoveBinary,
}

// --- Update Errors ---
#[derive(Debug)]
pub struct UpdateError {
    kind: UpdateErrorKind,
    message: String,
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
            UpdateErrorKind::BinaryDownloadFail => write!(
                f,
                "unable to retrieve fetch tmgr the latest executable from GitHub repo, try again later"
            ),
            UpdateErrorKind::CorruptedBinaryDownload => {
                write!(f, "unable to convert downloaded executable to bytes")
            }
            UpdateErrorKind::CreateFileFail => {
                write!(f, "unable to create file in downloads folder")
            }
            UpdateErrorKind::UnableToDeleteExistingBinary => {
                write!(f, "unable to delete existing executable")
            }
            UpdateErrorKind::UnableToMoveBinary => write!(
                f,
                "unable to move downloaded executable to bin of current executable"
            ),
        }
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (update error: {})", self.message, self.kind)
    }
}

// -- Request Models --
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GithubReleaseResponse {
    #[serde(rename = "tag_name")]
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
    #[serde(rename = "browser_download_url")]
    browser_download_url: String,
}
