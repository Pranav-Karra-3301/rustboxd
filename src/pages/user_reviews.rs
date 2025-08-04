use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserReviews {
    username: String,
}

impl UserReviews {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_reviews(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/films/reviews/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse user reviews from the page
        Ok(HashMap::new())
    }
}
