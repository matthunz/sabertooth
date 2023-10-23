use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Account {
    pub username: String,
    pub avatar: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StatusData {
    pub id: String,
    pub account: Account,
    pub content: String,
    #[serde(rename = "favourites_count")]
    pub favorites_count: u32,
    #[serde(rename = "favourited", default)]
    pub is_favorited: bool,
    pub reblogs_count: u32,
    #[serde(rename = "reblogged", default)]
    pub is_reblogged: bool,
    pub replies_count: u32,
    #[serde(rename = "replied", default)]
    pub is_replied: bool,
}

pub async fn get_timeline() -> Vec<StatusData> {
    reqwest::get("https://mas.to/api/v1/timelines/public")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
