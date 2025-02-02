use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub matchterm: String,
    pub previouspage: i64,
    pub searchterm: String,
    pub query: String,
    pub total: i64,
    pub page: i64,
    pub nextpage: i64,
    pub results: Vec<Item>,
    #[serde(rename = "language_filters")]
    pub language_filters: Vec<LanguageFilter>,
    #[serde(rename = "source_filters")]
    pub source_filters: Vec<SourceFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "result")]
pub struct Item {
    pub repo: String,
    pub language: String,
    pub linescount: i64,
    pub location: String,
    pub name: String,
    pub url: String,
    pub md5hash: String,
    pub lines: serde_json::Value,
    pub id: i64,
    pub filename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageFilter {
    pub count: i64,
    pub language: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFilter {
    pub count: i64,
    pub source: String,
    pub id: i64,
}
