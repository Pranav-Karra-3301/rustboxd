use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserLikes {
    username: String,
}

impl UserLikes {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_liked_films(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/likes/films/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse liked films from the page
        Ok(HashMap::new())
    }

    pub async fn get_liked_reviews(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/likes/reviews/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse liked reviews from the page
        Ok(HashMap::new())
    }
}
