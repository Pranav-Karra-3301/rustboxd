use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};
use crate::models::WatchlistMovie;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct UserWatchlist {
    username: String,
}

impl UserWatchlist {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn get_watchlist(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/{}/watchlist/", DOMAIN, self.username);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse user watchlist from the page
        Ok(HashMap::new())
    }
    
    pub async fn get_watchlist_movies(&self) -> Result<HashMap<String, WatchlistMovie>> {
        let client = Client::new();
        let url = format!("{}/{}/watchlist/", DOMAIN, self.username);
        let dom = client.get_page(&url).await?;
        
        let mut movies = HashMap::new();
        
        // Parse watchlist movies from the HTML
        let poster_selector = Selector::parse("li.poster-container").unwrap();
        let film_selector = Selector::parse("div.film-poster").unwrap();
        
        for poster in dom.select(&poster_selector) {
            if let Some(film) = poster.select(&film_selector).next() {
                if let Some(slug_attr) = film.value().attr("data-film-slug") {
                    let slug = slug_attr.to_string();
                    if let Some(title_attr) = film.value().attr("data-film-name") {
                        let name = title_attr.to_string();
                        let movie_url = format!("{}/film/{}/", DOMAIN, slug);
                        
                        let movie = WatchlistMovie {
                            name: name.clone(),
                            slug: slug.clone(),
                            url: movie_url,
                        };
                        
                        movies.insert(slug, movie);
                    }
                }
            }
        }
        
        Ok(movies)
    }
}
