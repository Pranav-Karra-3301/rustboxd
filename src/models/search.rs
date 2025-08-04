use serde::{Deserialize, Serialize};
use crate::core::{Client, Error, Result, constants::{DOMAIN, SEARCH_FILTERS}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search {
    pub query: String,
    pub search_filter: Option<String>,
    pub url: String,
    pub results: SearchResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub films: Vec<SearchFilm>,
    pub reviews: Vec<SearchReview>,
    pub lists: Vec<SearchList>,
    pub members: Vec<SearchMember>,
    pub cast_crew: Vec<SearchPerson>,
    pub tags: Vec<SearchTag>,
    pub stories: Vec<SearchStory>,
    pub articles: Vec<SearchArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilm {
    pub title: String,
    pub year: Option<i32>,
    pub slug: String,
    pub url: String,
    pub poster: Option<String>,
    pub rating: Option<f32>,
    pub director: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchReview {
    pub author: String,
    pub film_title: String,
    pub film_slug: String,
    pub content: String,
    pub rating: Option<f32>,
    pub likes: u32,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchList {
    pub title: String,
    pub author: String,
    pub slug: String,
    pub url: String,
    pub film_count: u32,
    pub likes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMember {
    pub username: String,
    pub display_name: String,
    pub url: String,
    pub avatar: Option<String>,
    pub films_watched: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPerson {
    pub name: String,
    pub slug: String,
    pub url: String,
    pub photo: Option<String>,
    pub known_for: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTag {
    pub name: String,
    pub url: String,
    pub film_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStory {
    pub title: String,
    pub author: String,
    pub url: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArticle {
    pub title: String,
    pub author: String,
    pub url: String,
    pub date: String,
    pub summary: String,
}

impl Search {
    pub async fn new(query: &str, search_filter: Option<&str>) -> Result<Self> {
        if let Some(filter) = search_filter {
            if !SEARCH_FILTERS.contains(&filter) {
                return Err(Error::Parse(format!("Invalid search filter: {}", filter)));
            }
        }

        let client = Client::new();
        let encoded_query = urlencoding::encode(query);
        
        let url = if let Some(filter) = search_filter {
            format!("{}/s/search/{}/{}", DOMAIN, filter, encoded_query)
        } else {
            format!("{}/s/search/{}", DOMAIN, encoded_query)
        };

        let dom = client.get_page(&url).await?;
        let results = Self::parse_search_results(&dom, search_filter)?;

        Ok(Search {
            query: query.to_string(),
            search_filter: search_filter.map(|s| s.to_string()),
            url,
            results,
        })
    }

    fn parse_search_results(dom: &scraper::Html, search_filter: Option<&str>) -> Result<SearchResults> {
        use scraper::Selector;
        
        let mut results = SearchResults {
            films: Vec::new(),
            reviews: Vec::new(),
            lists: Vec::new(),
            members: Vec::new(),
            cast_crew: Vec::new(),
            tags: Vec::new(),
            stories: Vec::new(),
            articles: Vec::new(),
        };

        match search_filter {
            Some("films") | None => {
                let film_selector = Selector::parse(".film-detail").unwrap();
                for element in dom.select(&film_selector) {
                    if let Ok(film) = Self::parse_film_result(&element) {
                        results.films.push(film);
                    }
                }
            }
            Some("reviews") => {
                let review_selector = Selector::parse(".review").unwrap();
                for element in dom.select(&review_selector) {
                    if let Ok(review) = Self::parse_review_result(&element) {
                        results.reviews.push(review);
                    }
                }
            }
            Some("lists") => {
                let list_selector = Selector::parse(".list-item").unwrap();
                for element in dom.select(&list_selector) {
                    if let Ok(list) = Self::parse_list_result(&element) {
                        results.lists.push(list);
                    }
                }
            }
            Some("members") => {
                let member_selector = Selector::parse(".person-summary").unwrap();
                for element in dom.select(&member_selector) {
                    if let Ok(member) = Self::parse_member_result(&element) {
                        results.members.push(member);
                    }
                }
            }
            _ => {} // TODO: Implement other search filters
        }

        Ok(results)
    }

    fn parse_film_result(element: &scraper::ElementRef) -> Result<SearchFilm> {
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
        let url = format!("{}{}", DOMAIN, href);
        
        let year = element.select(&year_selector)
            .next()
            .and_then(|el| el.inner_html().parse().ok());
        
        let poster = element.select(&poster_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .map(|s| s.to_string());

        Ok(SearchFilm {
            title,
            year,
            slug,
            url,
            poster,
            rating: None, // TODO: Parse rating if available
            director: None, // TODO: Parse director if available
        })
    }

    fn parse_review_result(_element: &scraper::ElementRef) -> Result<SearchReview> {
        // TODO: Implement review parsing
        Err(Error::Parse("Review parsing not implemented".to_string()))
    }

    fn parse_list_result(_element: &scraper::ElementRef) -> Result<SearchList> {
        // TODO: Implement list parsing
        Err(Error::Parse("List parsing not implemented".to_string()))
    }

    fn parse_member_result(_element: &scraper::ElementRef) -> Result<SearchMember> {
        // TODO: Implement member parsing
        Err(Error::Parse("Member parsing not implemented".to_string()))
    }

    pub async fn get_more_results(&mut self, max_pages: u32) -> Result<()> {
        for page in 2..=max_pages {
            let client = Client::new();
            let page_url = format!("{}/page/{}", self.url, page);
            
            let dom = client.get_page(&page_url).await?;
            let page_results = Self::parse_search_results(&dom, self.search_filter.as_deref())?;
            
            // Merge results
            self.results.films.extend(page_results.films);
            self.results.reviews.extend(page_results.reviews);
            self.results.lists.extend(page_results.lists);
            self.results.members.extend(page_results.members);
            self.results.cast_crew.extend(page_results.cast_crew);
            self.results.tags.extend(page_results.tags);
            self.results.stories.extend(page_results.stories);
            self.results.articles.extend(page_results.articles);
        }
        
        Ok(())
    }
}
