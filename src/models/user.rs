use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;
use crate::core::{Client, Error, Result, constants::DOMAIN};
use crate::pages::{UserActivity, UserDiary, UserFilms, UserLikes, UserLists, UserNetwork, UserProfile, UserReviews, UserTags, UserWatchlist};
use crate::models::{WatchlistMovie, DiaryMovieEntry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
    pub id: Option<u64>,
    pub is_hq: bool,
    pub display_name: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub watchlist_length: Option<u32>,
    pub stats: Option<UserStats>,
    pub favorites: Option<HashMap<String, FavoriteMovie>>,
    pub avatar: Option<String>,
    pub recent: UserRecent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteMovie {
    pub name: String,
    pub slug: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub films: u32,
    pub this_year: u32,
    pub following: u32,
    pub followers: u32,
    pub lists: u32,
    pub reviews: u32,
    pub diary_entries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecent {
    pub watchlist: Vec<String>,
    pub diary: DiaryData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiaryData {
    pub months: HashMap<String, HashMap<String, Vec<DiaryEntry>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiaryEntry {
    pub name: String,
    pub slug: String,
    pub rating: Option<f32>,
    pub review: Option<String>,
    pub liked: bool,
    pub rewatch: bool,
}

#[derive(Debug)]
pub struct UserPages {
    pub activity: UserActivity,
    pub diary: UserDiary,
    pub films: UserFilms,
    pub likes: UserLikes,
    pub lists: UserLists,
    pub network: UserNetwork,
    pub profile: UserProfile,
    pub reviews: UserReviews,
    pub tags: UserTags,
    pub watchlist: UserWatchlist,
}

impl User {
    pub async fn new(username: &str) -> Result<Self> {
        let username_regex = Regex::new(r"^[A-Za-z0-9_]*$").unwrap();
        if !username_regex.is_match(username) {
            return Err(Error::InvalidUsername(username.to_string()));
        }

        let username = username.to_lowercase();
        let client = Client::new();
        let url = format!("{}/user/{}", DOMAIN, username);
        
        let dom = client.get_page(&url).await?;
        
        // Extract user data from the HTML
        let user = Self::parse_user_data(&dom, &username, &url)?;
        
        Ok(user)
    }

    fn parse_user_data(dom: &scraper::Html, username: &str, url: &str) -> Result<Self> {
        use scraper::Selector;
        
        let display_name_selector = Selector::parse("h1.title-1").unwrap();
        let bio_selector = Selector::parse(".profile-summary .bio").unwrap();
        let location_selector = Selector::parse(".profile-summary .location").unwrap();
        let website_selector = Selector::parse(".profile-summary .website").unwrap();
        let stats_selector = Selector::parse(".profile-stats li").unwrap();
        
        let display_name = dom.select(&display_name_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_else(|| username.to_string());

        let bio = dom.select(&bio_selector)
            .next()
            .map(|el| el.inner_html());

        let location = dom.select(&location_selector)
            .next()
            .map(|el| el.inner_html());

        let website = dom.select(&website_selector)
            .next()
            .map(|el| el.inner_html());

        // Parse stats
        let stats_elements: Vec<_> = dom.select(&stats_selector).collect();
        let stats = if !stats_elements.is_empty() {
            Some(UserStats {
                films: 0,       // TODO: Parse from stats
                this_year: 0,
                following: 0,
                followers: 0,
                lists: 0,
                reviews: 0,
                diary_entries: 0,
            })
        } else {
            None
        };

        Ok(User {
            username: username.to_string(),
            url: url.to_string(),
            id: None, // TODO: Extract user ID
            is_hq: false, // TODO: Detect HQ status
            display_name,
            bio,
            location,
            website,
            watchlist_length: None,
            stats,
            favorites: None,
            avatar: None,
            recent: UserRecent {
                watchlist: Vec::new(),
                diary: DiaryData {
                    months: HashMap::new(),
                },
            },
        })
    }

    pub fn pages(&self) -> UserPages {
        UserPages {
            activity: UserActivity::new(&self.username),
            diary: UserDiary::new(&self.username),
            films: UserFilms::new(&self.username),
            likes: UserLikes::new(&self.username),
            lists: UserLists::new(&self.username),
            network: UserNetwork::new(&self.username),
            profile: UserProfile::new(&self.username),
            reviews: UserReviews::new(&self.username),
            tags: UserTags::new(&self.username),
            watchlist: UserWatchlist::new(&self.username),
        }
    }

    pub async fn get_activity(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().activity.get_activity().await
    }

    pub async fn get_diary(&self, year: Option<i32>, month: Option<u32>, day: Option<u32>, page: Option<u32>) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().diary.get_diary(year, month, day, page).await
    }

    pub async fn get_films(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().films.get_films().await
    }

    pub async fn get_films_by_rating(&self, rating: f32) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().films.get_films_rated(rating).await
    }

    pub async fn get_films_not_rated(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().films.get_films_not_rated().await
    }

    pub async fn get_genre_info(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().films.get_genre_info().await
    }

    pub async fn get_liked_films(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().likes.get_liked_films().await
    }

    pub async fn get_liked_reviews(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().likes.get_liked_reviews().await
    }

    pub async fn get_lists(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().lists.get_lists().await
    }

    pub async fn get_followers(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().network.get_followers().await
    }

    pub async fn get_following(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().network.get_following().await
    }

    pub async fn get_reviews(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().reviews.get_reviews().await
    }

    pub async fn get_tags(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().tags.get_tags().await
    }

    pub async fn get_watchlist(&self) -> Result<HashMap<String, serde_json::Value>> {
        self.pages().watchlist.get_watchlist().await
    }
    
    pub async fn get_watchlist_movies(&self) -> Result<HashMap<String, WatchlistMovie>> {
        let watchlist_page = self.pages().watchlist;
        watchlist_page.get_watchlist_movies().await
    }
    
    pub async fn get_diary_entries(&self) -> Result<Vec<DiaryMovieEntry>> {
        let diary_page = self.pages().diary;
        diary_page.get_diary_entries().await
    }
}
