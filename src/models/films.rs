use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::{Client, Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Films {
    pub url: String,
    pub ajax_url: String,
    pub movies: HashMap<String, FilmEntry>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilmEntry {
    pub title: String,
    pub year: Option<i32>,
    pub slug: String,
    pub url: String,
    pub poster: Option<String>,
    pub rating: Option<f32>,
    pub director: Option<String>,
    pub watched: bool,
    pub liked: bool,
    pub in_watchlist: bool,
}

impl Films {
    pub async fn new(url: &str) -> Result<Self> {
        let client = Client::new();
        let ajax_url = Self::get_ajax_url(url);
        
        let movies = Self::scrape_movies(&client, &ajax_url, url).await?;
        let count = movies.len();

        Ok(Films {
            url: url.to_string(),
            ajax_url,
            movies,
            count,
        })
    }

    fn get_ajax_url(url: &str) -> String {
        // Convert regular URL to AJAX URL
        if url.contains("/films/") {
            url.replace("/films/", "/ajax/films/")
        } else if url.contains("/film/") {
            url.replace("/film/", "/ajax/film/")
        } else {
            format!("{}/ajax", url)
        }
    }

    async fn scrape_movies(client: &Client, ajax_url: &str, original_url: &str) -> Result<HashMap<String, FilmEntry>> {
        let mut movies = HashMap::new();
        let mut page = 1;
        
        const VERTICAL_MAX: usize = 100; // 20 * 5 pages
        const HORIZONTAL_MAX: usize = 72; // 12 * 6 pages

        loop {
            let page_url = format!("{}/page/{}", ajax_url, page);
            let dom = client.get_page(&page_url).await?;
            
            let new_movies = if original_url.contains("/films/") {
                Self::extract_horizontal_movies(&dom)?
            } else if original_url.contains("/film/") {
                Self::extract_vertical_movies(&dom)?
            } else {
                HashMap::new()
            };

            let new_count = new_movies.len();
            movies.extend(new_movies);

            // Check if we should continue pagination
            let max_per_page = if original_url.contains("/films/") {
                HORIZONTAL_MAX
            } else {
                VERTICAL_MAX
            };

            if new_count < max_per_page || movies.len() >= 1000 {
                break;
            }

            page += 1;
        }

        Ok(movies)
    }

    fn extract_horizontal_movies(dom: &scraper::Html) -> Result<HashMap<String, FilmEntry>> {
        use scraper::Selector;
        
        let mut movies = HashMap::new();
        let film_selector = Selector::parse(".poster-container").unwrap();
        
        for element in dom.select(&film_selector) {
            if let Ok(film) = Self::parse_horizontal_film(&element) {
                movies.insert(film.slug.clone(), film);
            }
        }

        Ok(movies)
    }

    fn extract_vertical_movies(dom: &scraper::Html) -> Result<HashMap<String, FilmEntry>> {
        use scraper::Selector;
        
        let mut movies = HashMap::new();
        let film_selector = Selector::parse(".film-detail").unwrap();
        
        for element in dom.select(&film_selector) {
            if let Ok(film) = Self::parse_vertical_film(&element) {
                movies.insert(film.slug.clone(), film);
            }
        }

        Ok(movies)
    }

    fn parse_horizontal_film(element: &scraper::ElementRef) -> Result<FilmEntry> {
        use scraper::Selector;
        
        let img_selector = Selector::parse("img").unwrap();
        let link_selector = Selector::parse("a").unwrap();
        
        let img_element = element.select(&img_selector).next()
            .ok_or_else(|| Error::Parse("Film image not found".to_string()))?;
        
        let link_element = element.select(&link_selector).next()
            .ok_or_else(|| Error::Parse("Film link not found".to_string()))?;
        
        let title = img_element.value().attr("alt")
            .ok_or_else(|| Error::Parse("Film title not found".to_string()))?;
        
        let href = link_element.value().attr("href")
            .ok_or_else(|| Error::Parse("Film href not found".to_string()))?;
        
        let slug = href.trim_start_matches("/film/").trim_end_matches("/").to_string();
        let url = format!("https://letterboxd.com{}", href);
        
        let poster = img_element.value().attr("src").map(|s| s.to_string());

        Ok(FilmEntry {
            title: title.to_string(),
            year: None, // TODO: Extract year if available
            slug,
            url,
            poster,
            rating: None, // TODO: Extract rating if available
            director: None, // TODO: Extract director if available
            watched: false, // TODO: Check watched status
            liked: false, // TODO: Check liked status
            in_watchlist: false, // TODO: Check watchlist status
        })
    }

    fn parse_vertical_film(element: &scraper::ElementRef) -> Result<FilmEntry> {
        use scraper::Selector;
        
        let title_selector = Selector::parse(".film-title a").unwrap();
        let year_selector = Selector::parse(".film-year").unwrap();
        let poster_selector = Selector::parse(".film-poster img").unwrap();
        
        let title_element = element.select(&title_selector).next()
            .ok_or_else(|| Error::Parse("Film title not found".to_string()))?;
        
        let title = title_element.inner_html();
        let href = title_element.value().attr("href")
            .ok_or_else(|| Error::Parse("Film URL not found".to_string()))?;
        
        let slug = href.trim_start_matches("/film/").trim_end_matches("/").to_string();
        let url = format!("https://letterboxd.com{}", href);
        
        let year = element.select(&year_selector)
            .next()
            .and_then(|el| el.inner_html().parse().ok());
        
        let poster = element.select(&poster_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .map(|s| s.to_string());

        Ok(FilmEntry {
            title,
            year,
            slug,
            url,
            poster,
            rating: None, // TODO: Extract rating if available
            director: None, // TODO: Extract director if available
            watched: false, // TODO: Check watched status
            liked: false, // TODO: Check liked status
            in_watchlist: false, // TODO: Check watchlist status
        })
    }

    pub fn filter_by_year(&self, year: i32) -> Vec<&FilmEntry> {
        self.movies.values()
            .filter(|film| film.year == Some(year))
            .collect()
    }

    pub fn filter_by_rating(&self, min_rating: f32) -> Vec<&FilmEntry> {
        self.movies.values()
            .filter(|film| film.rating.map_or(false, |r| r >= min_rating))
            .collect()
    }

    pub fn get_watched(&self) -> Vec<&FilmEntry> {
        self.movies.values()
            .filter(|film| film.watched)
            .collect()
    }

    pub fn get_liked(&self) -> Vec<&FilmEntry> {
        self.movies.values()
            .filter(|film| film.liked)
            .collect()
    }

    pub fn get_in_watchlist(&self) -> Vec<&FilmEntry> {
        self.movies.values()
            .filter(|film| film.in_watchlist)
            .collect()
    }
}
