use chrono::{Datelike, Utc};

// Date/Time Constants
pub fn current_year() -> i32 {
    Utc::now().year()
}

pub fn current_month() -> u32 {
    Utc::now().month()
}

pub fn current_day() -> u32 {
    Utc::now().day()
}

pub const MONTH_ABBREVIATIONS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
];

// Domain/URL Constants
pub const URL_PROTOCOLS: [&str; 2] = ["http://", "https://"];

pub const DOMAIN_FULL: &str = "letterboxd.com";
pub const DOMAIN_SHORT: &str = "boxd.it";

// Base URLs
pub const DOMAIN: &str = "https://letterboxd.com";
pub const SITE: &str = "https://letterboxd.com/";
pub const SITE_SHORT: &str = "https://boxd.it/";

pub const DOMAIN_MATCHES: [&str; 2] = ["letterboxd.com/", "boxd.it/"];

// Movie-Related Constants
pub const VALID_RATINGS: [f32; 10] = [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];

pub const GENRES: [&str; 19] = [
    "action", "adventure", "animation", "comedy", "crime",
    "documentary", "drama", "family", "fantasy", "history",
    "horror", "music", "mystery", "romance", "science-fiction",
    "tv-movie", "thriller", "war", "western"
];

// Search Filters
pub const SEARCH_FILTERS: [&str; 11] = [
    "films", "reviews", "lists", "original-lists",
    "stories", "cast-crew", "members", "tags",
    "articles", "episodes", "full-text"
];
