use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::{Client, Result, constants::DOMAIN};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub url: String,
    pub slug: String,
    pub movie_id: Option<u64>,
    pub title: String,
    pub original_title: Option<String>,
    pub runtime: Option<u32>,
    pub rating: Option<f32>,
    pub year: Option<i32>,
    pub tmdb_link: Option<String>,
    pub imdb_link: Option<String>,
    pub poster: Option<String>,
    pub banner: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub trailer: Option<MovieTrailer>,
    pub alternative_titles: Vec<String>,
    pub details: Option<MovieDetails>,
    pub genres: Vec<String>,
    pub cast: Vec<MoviePerson>,
    pub crew: Vec<MoviePerson>,
    pub popular_reviews: Vec<MovieReview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieTrailer {
    pub id: String,
    pub link: String,
    pub embed_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieDetails {
    pub director: Vec<String>,
    pub writer: Vec<String>,
    pub producer: Vec<String>,
    pub cinematographer: Vec<String>,
    pub editor: Vec<String>,
    pub composer: Vec<String>,
    pub production_companies: Vec<String>,
    pub countries: Vec<String>,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoviePerson {
    pub name: String,
    pub role_name: Option<String>,
    pub slug: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieReview {
    pub author: String,
    pub rating: Option<f32>,
    pub content: String,
    pub likes: u32,
    pub date: String,
}

impl Movie {
    pub async fn new(slug: &str) -> Result<Self> {
        let client = Client::new();
        let url = format!("{}/film/{}", DOMAIN, slug);
        
        let dom = client.get_page(&url).await?;
        
        // Parse movie data from HTML
        let movie = Self::parse_movie_data(&dom, slug, &url)?;
        
        Ok(movie)
    }

    fn parse_movie_data(dom: &scraper::Html, slug: &str, url: &str) -> Result<Self> {
        use scraper::Selector;
        
        let title_selector = Selector::parse("h1.headline-1").unwrap();
        let year_selector = Selector::parse(".film-poster").unwrap();
        let rating_selector = Selector::parse(".average-rating").unwrap();
        let _runtime_selector = Selector::parse("p.text-link").unwrap();
        let tagline_selector = Selector::parse(".tagline").unwrap();
        let description_selector = Selector::parse(".truncate p").unwrap();
        let genres_selector = Selector::parse("#tab-genres .text-slug").unwrap();
        
        let title = dom.select(&title_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_else(|| slug.replace('-', " "));

        let year = dom.select(&year_selector)
            .next()
            .and_then(|el| el.value().attr("data-film-year"))
            .and_then(|year_str| year_str.parse().ok());

        let rating = dom.select(&rating_selector)
            .next()
            .and_then(|el| el.inner_html().parse().ok());

        let tagline = dom.select(&tagline_selector)
            .next()
            .map(|el| el.inner_html());

        let description = dom.select(&description_selector)
            .next()
            .map(|el| el.inner_html());

        let genres: Vec<String> = dom.select(&genres_selector)
            .map(|el| el.inner_html())
            .collect();

        Ok(Movie {
            url: url.to_string(),
            slug: slug.to_string(),
            movie_id: None, // TODO: Extract movie ID
            title,
            original_title: None, // TODO: Extract original title
            runtime: None, // TODO: Parse runtime
            rating,
            year,
            tmdb_link: None, // TODO: Extract TMDB link
            imdb_link: None, // TODO: Extract IMDB link
            poster: None, // TODO: Extract poster URL
            banner: None, // TODO: Extract banner URL
            tagline,
            description,
            trailer: None, // TODO: Parse trailer
            alternative_titles: Vec::new(), // TODO: Extract alternative titles
            details: None, // TODO: Parse movie details
            genres,
            cast: Vec::new(), // TODO: Parse cast
            crew: Vec::new(), // TODO: Parse crew
            popular_reviews: Vec::new(), // TODO: Parse reviews
        })
    }

    pub async fn get_watchers(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/film/{}/members/", DOMAIN, self.slug);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse watchers from the page
        Ok(HashMap::new())
    }

    pub async fn get_reviews(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/film/{}/reviews/", DOMAIN, self.slug);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse reviews from the page
        Ok(HashMap::new())
    }

    pub async fn get_similar(&self) -> Result<HashMap<String, serde_json::Value>> {
        let client = Client::new();
        let url = format!("{}/film/{}/similar/", DOMAIN, self.slug);
        let _dom = client.get_page(&url).await?;
        
        // TODO: Parse similar movies from the page
        Ok(HashMap::new())
    }
}
