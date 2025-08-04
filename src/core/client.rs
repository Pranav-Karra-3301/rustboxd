use reqwest::{Client as ReqwestClient, header::{HeaderMap, HeaderValue, USER_AGENT, REFERER}};
use scraper::Html;
use crate::core::{Error, Result, constants::DOMAIN};

#[derive(Debug, Clone)]
pub struct Client {
    client: ReqwestClient,
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        );
        headers.insert(
            REFERER,
            HeaderValue::from_static(DOMAIN)
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: DOMAIN.to_string(),
        }
    }

    pub async fn get_page(&self, url: &str) -> Result<Html> {
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::PageLoad {
                url: url.to_string(),
                message: e.to_string(),
            })?;

        self.check_response_errors(url, &response)?;

        let html = response
            .text()
            .await
            .map_err(|e| Error::PageLoad {
                url: url.to_string(),
                message: e.to_string(),
            })?;

        Ok(Html::parse_document(&html))
    }

    fn check_response_errors(&self, url: &str, response: &reqwest::Response) -> Result<()> {
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::NOT_FOUND => Err(Error::PageLoad {
                url: url.to_string(),
                message: "Page not found".to_string(),
            }),
            reqwest::StatusCode::FORBIDDEN => Err(Error::PrivateRoute),
            status => Err(Error::PageLoad {
                url: url.to_string(),
                message: format!("HTTP {}", status),
            }),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
