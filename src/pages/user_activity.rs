use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserActivity {
    username: String,
}

impl UserActivity {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_activity(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse user activity from the page
        Ok(HashMap::new())
    }

    pub async fn get_activity_following(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/following/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse following activity from the page
        Ok(HashMap::new())
    }
}
