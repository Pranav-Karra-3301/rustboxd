use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Failed to parse HTML")]
    HtmlParse,
    
    #[error("Page load error for URL {url}: {message}")]
    PageLoad { url: String, message: String },
    
    #[error("Invalid response from server")]
    InvalidResponse,
    
    #[error("Private route access denied")]
    PrivateRoute,
    
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    
    #[error("Movie not found: {0}")]
    MovieNotFound(String),
    
    #[error("Parsing error: {0}")]
    Parse(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
