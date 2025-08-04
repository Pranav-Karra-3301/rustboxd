use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserLists {
    username: String,
}

impl UserLists {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_lists(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/lists/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse user lists from the page
        Ok(HashMap::new())
    }
}
