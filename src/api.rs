use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub username: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize)]
pub struct StatusData {
    pub id: String,
    pub account: Account,
    pub content: String,
    #[serde(rename = "favourites_count")]
    pub favorites_count: u32,
    pub reblogs_count: u32,
    pub replies_count: u32,
}

pub async fn get_timeline() -> Vec<StatusData> {
    reqwest::get("https://mas.to/api/v1/timelines/public")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
