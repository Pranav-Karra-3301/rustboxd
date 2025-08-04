use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserFilms {
    username: String,
}

impl UserFilms {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_films(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/films/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse user films from the page
        Ok(HashMap::new())
    }

    pub async fn get_films_rated(&self, rating: f32) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let rating_str = if rating.fract() == 0.0 {
            format!("{}", rating as i32)
        } else {
            format!("{}", rating)
        };
        let url = format!("{}/{}/films/rated/{}/", DOMAIN, self.username, rating_str);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse rated films from the page
        Ok(HashMap::new())
    }

    pub async fn get_films_not_rated(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/films/not-rated/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse unrated films from the page
        Ok(HashMap::new())
    }

    pub async fn get_genre_info(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/films/genres/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse genre statistics from the page
        Ok(HashMap::new())
    }
}
