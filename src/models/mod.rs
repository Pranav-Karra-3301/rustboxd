pub mod user;
pub mod movie;
pub mod search;
pub mod films;
pub mod list;

pub use user::{User, FavoriteMovie, UserRecent, DiaryData, DiaryEntry};
pub use movie::Movie;
pub use search::Search;
pub use films::Films;
pub use list::List;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WatchlistMovie {
    pub name: String,
    pub slug: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiaryMovieEntry {
    pub name: String,
    pub slug: String,
    pub title: String,
    pub year: Option<u16>,
    pub director: Option<String>,
    pub genres: Vec<String>,
    pub runtime: Option<u16>,
    pub rating: Option<f32>,
    pub description: Option<String>,
    pub month: u32,
    pub day: u32,
}
