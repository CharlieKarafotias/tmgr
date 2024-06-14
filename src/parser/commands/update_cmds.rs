mod request_models;
use request_models::GithubReleaseResponse;
use reqwest::header::USER_AGENT;
use semver::Version;

pub fn update() {
    println!("Checking repository for updates...");

    let (needs_update, binary_download_url) = check_for_updates();
    if needs_update {
        if let Some(binary_download_url) = binary_download_url {
            println!("Update found, downloading...");
            download_binary(binary_download_url);
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
async fn download_binary(_binary_download_url: String) {
    todo!("download binary");
    // let client = reqwest::Client::new();
    // let res = client
    //     .get(binary_download_url)
    //     .header(USER_AGENT, "tmgr-rust")
    //     .send()
    //     .await;
    // TODO: need to find bin with tmgr in it, then save binary
    // let bytes = res.bytes().unwrap();
    // std::fs::write("tmgr", bytes).unwrap();
}
