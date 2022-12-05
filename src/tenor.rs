use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tenor {
    pub results: Vec<Result>,
    pub next: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: String,
    pub title: String,
    #[serde(rename = "content_description")]
    pub content_description: String,
    #[serde(rename = "content_rating")]
    pub content_rating: String,
    #[serde(rename = "h1_title")]
    pub h1_title: String,
    pub media: Vec<Medum>,
    #[serde(rename = "bg_color")]
    pub bg_color: String,
    pub created: f64,
    pub itemurl: String,
    pub url: String,
    pub tags: Vec<Value>,
    pub flags: Vec<Value>,
    pub shares: i64,
    pub hasaudio: bool,
    pub hascaption: bool,
    #[serde(rename = "source_id")]
    pub source_id: String,
    pub composite: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medum {
    pub nanomp4: Nanomp4,
    pub mp4: Mp4,
    pub tinymp4: Tinymp4,
    pub mediumgif: Mediumgif,
    pub loopedmp4: Loopedmp4,
    pub nanogif: Nanogif,
    pub tinywebm: Tinywebm,
    pub webm: Webm,
    pub tinygif: Tinygif,
    pub gif: Gif,
    pub nanowebm: Nanowebm,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nanomp4 {
    pub size: i64,
    pub dims: Vec<i64>,
    pub preview: String,
    pub url: String,
    pub duration: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp4 {
    pub duration: f64,
    pub dims: Vec<i64>,
    pub preview: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tinymp4 {
    pub size: i64,
    pub duration: f64,
    pub preview: String,
    pub url: String,
    pub dims: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mediumgif {
    pub preview: String,
    pub size: i64,
    pub dims: Vec<i64>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loopedmp4 {
    pub size: i64,
    pub url: String,
    pub preview: String,
    pub duration: f64,
    pub dims: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nanogif {
    pub url: String,
    pub preview: String,
    pub size: i64,
    pub dims: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tinywebm {
    pub dims: Vec<i64>,
    pub preview: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webm {
    pub preview: String,
    pub dims: Vec<i64>,
    pub url: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tinygif {
    pub dims: Vec<i64>,
    pub preview: String,
    pub url: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gif {
    pub dims: Vec<i64>,
    pub preview: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nanowebm {
    pub url: String,
    pub size: i64,
    pub dims: Vec<i64>,
    pub preview: String,
}
