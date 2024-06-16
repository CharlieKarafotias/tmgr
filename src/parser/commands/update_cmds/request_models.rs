use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubReleaseResponse {
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
}
