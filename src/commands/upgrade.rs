use serde::Deserialize;

pub(crate) async fn run() {
    todo!("Upgrade command not yet implemented")
}

// --- Start Request Models ---

#[derive(Default, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GithubReleaseResponse {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Default, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
    #[serde(rename = "browser_download_url")]
    browser_download_url: String,
}

// --- End Request Models --
