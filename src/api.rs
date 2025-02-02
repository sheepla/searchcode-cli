use crate::types::*;
use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ApiClient(pub reqwest::Client);

#[derive(Debug, Builder, Default)]
pub struct SearchParam {
    query: String,

    #[builder(default)]
    language_code: Option<u32>,

    #[builder(default)]
    page: Option<u32>,

    #[builder(default)]
    max_pages: Option<u32>,

    #[builder(default)]
    min_lines: Option<String>,

    #[builder(default)]
    max_lines: Option<String>,

    #[builder(default)]
    provider_code: Option<u32>,
}

impl ApiClient {
    pub async fn search(&self, param: &SearchParam) -> eyre::Result<SearchResult> {
        let query_param = {
            let mut query_param = HashMap::<&str, String>::new();

            query_param.insert("q", param.query.to_owned());

            if let Some(language_code) = param.language_code {
                query_param.insert("p", language_code.to_string());
            }

            if let Some(page) = param.page {
                query_param.insert("p", page.to_string());
            }

            if let Some(max_pages) = param.max_pages {
                query_param.insert("per_page", max_pages.to_string());
            }

            if let Some(min_lines) = &param.min_lines {
                query_param.insert("loc", min_lines.to_owned());
            }

            if let Some(max_lines) = &param.max_lines {
                query_param.insert("loc2", max_lines.to_owned());
            }

            if let Some(provider_code) = &param.provider_code {
                query_param.insert("src", provider_code.to_string());
            }

            query_param
        };

        let response = self
            .0
            .get("https://searchcode.com/api/codesearch_I")
            .query(&query_param)
            .send()
            .await?
            .error_for_status()?;

        let search_result = response.json::<SearchResult>().await?;

        Ok(search_result)
    }
}
