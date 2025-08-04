use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserNetwork {
    username: String,
}

impl UserNetwork {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_followers(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/followers/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse followers from the page
        Ok(HashMap::new())
    }

    pub async fn get_following(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/following/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse following from the page
        Ok(HashMap::new())
    }
}
