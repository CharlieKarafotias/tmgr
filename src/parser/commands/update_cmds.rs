mod request_models;
use request_models::GithubReleaseResponse;
use reqwest::header::USER_AGENT;

pub fn update() {
    println!("Checking repository for updates...");

    let needs_update = check_for_updates();
    println!("Needs update: {}", needs_update);

    println!("Update complete!");
}

#[tokio::main]
async fn check_for_updates() -> bool {
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
            let latest_version_github = latest_release.tag_name.replace('v', "");
            let current_version = env!("CARGO_PKG_VERSION").to_string();
            println!("Current version: {}", current_version);
            println!("Latest version: {}", latest_version_github);
            // TODO: semver comparison function
            true
        }
        Err(_) => {
            println!("Failed to check for updates");
            false
        }
    }
}
