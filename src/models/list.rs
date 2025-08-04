use serde::{Deserialize, Serialize};
use crate::core::{Client, Error, Result, constants::DOMAIN};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub slug: String,
    pub url: String,
    pub film_count: u32,
    pub likes: u32,
    pub comments: u32,
    pub is_ranked: bool,
    pub films: Vec<ListFilm>,
    pub tags: Vec<String>,
    pub created_date: Option<String>,
    pub updated_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilm {
    pub title: String,
    pub year: Option<i32>,
    pub slug: String,
    pub url: String,
    pub poster: Option<String>,
    pub director: Option<String>,
    pub position: Option<u32>,
    pub notes: Option<String>,
}

impl List {
    pub async fn new(author: &str, slug: &str) -> Result<Self> {
        let client = Client::new();
        let url = format!("{}/{}/list/{}", DOMAIN, author, slug);
        
        let dom = client.get_page(&url).await?;
        let list = Self::parse_list_data(&dom, author, slug, &url)?;
        
        Ok(list)
    }

    pub async fn from_url(url: &str) -> Result<Self> {
        let client = Client::new();
        let dom = client.get_page(url).await?;
        
        // Extract author and slug from URL
        let url_parts: Vec<&str> = url.split('/').collect();
        if url_parts.len() < 6 {
            return Err(Error::Parse("Invalid list URL format".to_string()));
        }
        
        let author = url_parts[3];
        let slug = url_parts[5];
        
        let list = Self::parse_list_data(&dom, author, slug, url)?;
        
        Ok(list)
    }

    fn parse_list_data(dom: &scraper::Html, author: &str, slug: &str, url: &str) -> Result<Self> {
        use scraper::Selector;
        
        let title_selector = Selector::parse("h1.list-title").unwrap();
        let description_selector = Selector::parse(".list-description").unwrap();
        let stats_selector = Selector::parse(".list-stats li").unwrap();
        let film_selector = Selector::parse(".poster-list li").unwrap();
        let tags_selector = Selector::parse(".list-tags a").unwrap();
        
        let title = dom.select(&title_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_else(|| "Untitled List".to_string());

        let description = dom.select(&description_selector)
            .next()
            .map(|el| el.inner_html());

        let stats: Vec<_> = dom.select(&stats_selector).collect();
        let film_count = stats.get(0)
            .and_then(|el| el.inner_html().parse().ok())
            .unwrap_or(0);
        
        let likes = stats.get(1)
            .and_then(|el| Self::parse_count_text(&el.inner_html()))
            .unwrap_or(0);
            
        let comments = stats.get(2)
            .and_then(|el| Self::parse_count_text(&el.inner_html()))
            .unwrap_or(0);

        // Parse films
        let mut films = Vec::new();
        for (index, element) in dom.select(&film_selector).enumerate() {
            if let Ok(film) = Self::parse_list_film(&element, index as u32 + 1) {
                films.push(film);
            }
        }

        // Parse tags
        let tags: Vec<String> = dom.select(&tags_selector)
            .map(|el| el.inner_html())
            .collect();

        Ok(List {
            title,
            description,
            author: author.to_string(),
            slug: slug.to_string(),
            url: url.to_string(),
            film_count,
            likes,
            comments,
            is_ranked: false, // TODO: Detect if list is ranked
            films,
            tags,
            created_date: None, // TODO: Extract creation date
            updated_date: None, // TODO: Extract update date
        })
    }

    fn parse_list_film(element: &scraper::ElementRef, position: u32) -> Result<ListFilm> {
        use scraper::Selector;
        
        let poster_selector = Selector::parse(".poster").unwrap();
        let img_selector = Selector::parse("img").unwrap();
        let link_selector = Selector::parse("a").unwrap();
        
        let poster_element = element.select(&poster_selector).next()
            .ok_or_else(|| Error::Parse("Poster element not found".to_string()))?;
        
        let img_element = poster_element.select(&img_selector).next()
            .ok_or_else(|| Error::Parse("Image element not found".to_string()))?;
        
        let link_element = poster_element.select(&link_selector).next()
            .ok_or_else(|| Error::Parse("Link element not found".to_string()))?;
        
        let title = img_element.value().attr("alt")
            .ok_or_else(|| Error::Parse("Film title not found".to_string()))?;
        
        let href = link_element.value().attr("href")
            .ok_or_else(|| Error::Parse("Film href not found".to_string()))?;
        
        let slug = href.trim_start_matches("/film/").trim_end_matches("/").to_string();
        let url = format!("{}{}", DOMAIN, href);
        
        let poster = img_element.value().attr("src").map(|s| s.to_string());

        Ok(ListFilm {
            title: title.to_string(),
            year: None, // TODO: Extract year if available
            slug,
            url,
            poster,
            director: None, // TODO: Extract director if available
            position: Some(position),
            notes: None, // TODO: Extract notes if available
        })
    }

    fn parse_count_text(text: &str) -> Option<u32> {
        // Handle formats like "1.2K", "500", etc.
        let cleaned = text.replace(',', "");
        if cleaned.ends_with('K') {
            let num_str = cleaned.trim_end_matches('K');
            if let Ok(num) = num_str.parse::<f32>() {
                return Some((num * 1000.0) as u32);
            }
        } else if let Ok(num) = cleaned.parse::<u32>() {
            return Some(num);
        }
        None
    }

    pub async fn get_comments(&self) -> Result<Vec<ListComment>> {
        let client = Client::new();
        let url = format!("{}/comments/", self.url);
        let dom = client.get_page(&url).await?;
        
        Self::parse_comments(&dom)
    }

    fn parse_comments(dom: &scraper::Html) -> Result<Vec<ListComment>> {
        use scraper::Selector;
        
        let mut comments = Vec::new();
        let comment_selector = Selector::parse(".comment").unwrap();
        
        for element in dom.select(&comment_selector) {
            if let Ok(comment) = Self::parse_comment(&element) {
                comments.push(comment);
            }
        }
        
        Ok(comments)
    }

    fn parse_comment(element: &scraper::ElementRef) -> Result<ListComment> {
        use scraper::Selector;
        
        let author_selector = Selector::parse(".comment-author").unwrap();
        let content_selector = Selector::parse(".comment-content").unwrap();
        let date_selector = Selector::parse(".comment-date").unwrap();
        
        let author = element.select(&author_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_default();
        
        let content = element.select(&content_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_default();
        
        let date = element.select(&date_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_default();

        Ok(ListComment {
            author,
            content,
            date,
            likes: 0, // TODO: Parse likes if available
        })
    }

    pub fn get_film_by_position(&self, position: u32) -> Option<&ListFilm> {
        self.films.iter().find(|film| film.position == Some(position))
    }

    pub fn get_films_by_year(&self, year: i32) -> Vec<&ListFilm> {
        self.films.iter()
            .filter(|film| film.year == Some(year))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListComment {
    pub author: String,
    pub content: String,
    pub date: String,
    pub likes: u32,
}
