use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};
use crate::models::{DiaryMovieEntry, Movie};
use scraper::{Html, Selector};

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
    
    pub async fn get_diary_entries(&self) -> Result<Vec<DiaryMovieEntry>> {
        let client = Client::new();
        let url = format!("{}/{}/films/diary/", DOMAIN, self.username);
        let dom = client.get_page(&url).await?;
        
        let mut entries = Vec::new();
        
        // Parse diary entries from the HTML
        let entry_selector = Selector::parse("tr.diary-entry-row").unwrap();
        let film_selector = Selector::parse("h3.headline-3 a").unwrap();
        let date_selector = Selector::parse("td.td-calendar-date a").unwrap();
        
        for entry in dom.select(&entry_selector) {
            if let Some(film_link) = entry.select(&film_selector).next() {
                let name = film_link.inner_html();
                let href = film_link.value().attr("href").unwrap_or("");
                let slug = href.trim_start_matches("/film/").trim_end_matches('/').to_string();
                
                // Extract date if available
                let (month, day) = if let Some(date_elem) = entry.select(&date_selector).next() {
                    if let Some(datetime) = date_elem.value().attr("data-date") {
                        // Parse date format: "2024-03-15"
                        let parts: Vec<&str> = datetime.split('-').collect();
                        if parts.len() >= 3 {
                            let month = parts[1].parse::<u32>().unwrap_or(1);
                            let day = parts[2].parse::<u32>().unwrap_or(1);
                            (month, day)
                        } else {
                            (1, 1)
                        }
                    } else {
                        (1, 1)
                    }
                } else {
                    (1, 1)
                };
                
                // Try to get movie details
                let movie_entry = DiaryMovieEntry {
                    name: name.clone(),
                    slug: slug.clone(),
                    title: name,
                    year: None,
                    director: None,
                    genres: Vec::new(),
                    runtime: None,
                    rating: None,
                    description: None,
                    month,
                    day,
                };
                
                entries.push(movie_entry);
            }
        }
        
        // Enrich with movie details (limit to first 10 for performance)
        for entry in entries.iter_mut().take(10) {
            if let Ok(movie) = Movie::new(&entry.slug).await {
                entry.title = movie.title.clone();
                entry.year = movie.year;
                entry.director = movie.crew.get("director")
                    .and_then(|dirs| dirs.first())
                    .and_then(|d| d.get("name"))
                    .map(|n| n.to_string());
                entry.genres = movie.genres.iter()
                    .filter(|g| g.get("type").and_then(|t| t.as_str()) == Some("genre"))
                    .filter_map(|g| g.get("name").and_then(|n| n.as_str()))
                    .map(String::from)
                    .collect();
                entry.runtime = movie.runtime.and_then(|r| r.parse().ok());
                entry.rating = movie.rating.and_then(|r| r.parse().ok());
                entry.description = movie.description.clone();
            }
        }
        
        Ok(entries)
    }
}
