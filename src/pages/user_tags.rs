use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserTags {
    username: String,
}

impl UserTags {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_tags(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/tags/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse user tags from the page
        Ok(HashMap::new())
    }
}
