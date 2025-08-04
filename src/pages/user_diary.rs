use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug)]
pub struct UserDiary {
    username: String,
}

impl UserDiary {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_diary(&self, year: Option<i32>, month: Option<u32>, day: Option<u32>, page: Option<u32>) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        
        let mut url = format!("{}/{}/films/diary/", DOMAIN, self.username);
        
        if let Some(year) = year {
            url.push_str(&format!("for/{}/", year));
            
            if let Some(month) = month {
                url.push_str(&format!("{:02}/", month));
                
                if let Some(day) = day {
                    url.push_str(&format!("{:02}/", day));
                }
            }
        }
        
        if let Some(page) = page {
            url.push_str(&format!("page/{}/", page));
        }
        
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse diary entries from the page
        Ok(HashMap::new())
    }

    pub async fn get_year(&self, year: i32) -> Result<HashMap<String, serde_json::Value>> {
        self.get_diary(Some(year), None, None, None).await
    }

    pub async fn get_month(&self, year: i32, month: u32) -> Result<HashMap<String, serde_json::Value>> {
        self.get_diary(Some(year), Some(month), None, None).await
    }

    pub async fn get_day(&self, year: i32, month: u32, day: u32) -> Result<HashMap<String, serde_json::Value>> {
        self.get_diary(Some(year), Some(month), Some(day), None).await
    }

    pub async fn get_wrapped(&self, year: i32) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/films/diary/for/{}/wrapped/", DOMAIN, self.username, year);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse wrapped data from the page
        Ok(HashMap::new())
    }
}
