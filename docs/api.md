# API Documentation

## Core Module

### Client

The `Client` struct provides the HTTP functionality for making requests to Letterboxd.

```rust
use rustboxd::core::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let html = client.get_page("https://letterboxd.com/user/username/").await?;
    // Process HTML...
    Ok(())
}
```

#### Methods

- `new() -> Self` - Creates a new client with default headers
- `get_page(url: &str) -> Result<Html>` - Fetches and parses HTML from a URL

### Error Types

Comprehensive error handling for all operations:

```rust
use rustboxd::Error;

match some_operation().await {
    Ok(result) => println!("Success: {:?}", result),
    Err(Error::PageLoad { url, message }) => {
        eprintln!("Failed to load {}: {}", url, message);
    }
    Err(Error::InvalidUsername(username)) => {
        eprintln!("Invalid username: {}", username);
    }
    Err(Error::MovieNotFound(slug)) => {
        eprintln!("Movie not found: {}", slug);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

#### Error Variants

- `Http(reqwest::Error)` - HTTP request errors
- `HtmlParse` - HTML parsing failures
- `PageLoad { url: String, message: String }` - Page loading errors
- `InvalidResponse` - Invalid server responses
- `PrivateRoute` - Access denied to private content
- `InvalidUsername(String)` - Invalid username format
- `MovieNotFound(String)` - Movie slug not found
- `Parse(String)` - General parsing errors
- `Serialization(serde_json::Error)` - JSON serialization errors
- `UrlParse(url::ParseError)` - URL parsing errors

### Constants

Pre-defined constants for URLs, validation, and configuration:

```rust
use rustboxd::core::constants::*;

// Date functions
let year = current_year();
let month = current_month();
let day = current_day();

// URLs
println!("Base domain: {}", DOMAIN);
println!("Valid genres: {:?}", GENRES);
println!("Valid ratings: {:?}", VALID_RATINGS);
```

## Models

### User

The `User` struct represents a Letterboxd user with all their profile information and statistics.

```rust
use rustboxd::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new("username").await?;
    
    println!("Display name: {}", user.display_name);
    println!("Bio: {:?}", user.bio);
    
    if let Some(stats) = &user.stats {
        println!("Films: {}", stats.films);
        println!("Reviews: {}", stats.reviews);
    }
    
    Ok(())
}
```

#### Fields

- `username: String` - Lowercase username
- `url: String` - Full profile URL
- `id: Option<u64>` - Letterboxd user ID
- `is_hq: bool` - Whether user has HQ membership
- `display_name: String` - Display name as shown on profile
- `bio: Option<String>` - User biography
- `location: Option<String>` - User location
- `website: Option<String>` - User website
- `watchlist_length: Option<u32>` - Number of films in watchlist
- `stats: Option<UserStats>` - User statistics
- `favorites: Option<Vec<String>>` - Favorite films
- `avatar: Option<String>` - Avatar image URL
- `recent: UserRecent` - Recent activity

#### Methods

**Activity & Social**
- `get_activity() -> Result<HashMap<String, Value>>` - Get user activity feed
- `get_followers() -> Result<HashMap<String, Value>>` - Get user's followers
- `get_following() -> Result<HashMap<String, Value>>` - Get users being followed

**Films & Viewing**
- `get_films() -> Result<HashMap<String, Value>>` - Get all watched films
- `get_films_by_rating(rating: f32) -> Result<HashMap<String, Value>>` - Get films with specific rating
- `get_films_not_rated() -> Result<HashMap<String, Value>>` - Get unrated films
- `get_diary(year, month, day, page) -> Result<HashMap<String, Value>>` - Get diary entries
- `get_watchlist() -> Result<HashMap<String, Value>>` - Get watchlist

**Lists & Reviews**
- `get_lists() -> Result<HashMap<String, Value>>` - Get user's lists
- `get_reviews() -> Result<HashMap<String, Value>>` - Get user's reviews
- `get_liked_films() -> Result<HashMap<String, Value>>` - Get liked films
- `get_liked_reviews() -> Result<HashMap<String, Value>>` - Get liked reviews

**Statistics & Analysis**
- `get_genre_info() -> Result<HashMap<String, Value>>` - Get genre viewing statistics
- `get_tags() -> Result<HashMap<String, Value>>` - Get user's tags

### Movie

The `Movie` struct contains comprehensive information about a film.

```rust
use rustboxd::Movie;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let movie = Movie::new("the-matrix").await?;
    
    println!("Title: {}", movie.title);
    println!("Year: {:?}", movie.year);
    println!("Rating: {:?}", movie.rating);
    println!("Genres: {:?}", movie.genres);
    
    // Get additional data
    let reviews = movie.get_reviews().await?;
    let similar = movie.get_similar().await?;
    
    Ok(())
}
```

#### Fields

- `url: String` - Full movie URL
- `slug: String` - Movie slug identifier
- `movie_id: Option<u64>` - Letterboxd movie ID
- `title: String` - Movie title
- `original_title: Option<String>` - Original language title
- `runtime: Option<u32>` - Runtime in minutes
- `rating: Option<f32>` - Average rating (0.5-5.0)
- `year: Option<i32>` - Release year
- `tmdb_link: Option<String>` - TMDB link
- `imdb_link: Option<String>` - IMDB link
- `poster: Option<String>` - Poster image URL
- `banner: Option<String>` - Banner image URL
- `tagline: Option<String>` - Movie tagline
- `description: Option<String>` - Plot description
- `trailer: Option<MovieTrailer>` - Trailer information
- `alternative_titles: Vec<String>` - Alternative titles
- `details: Option<MovieDetails>` - Production details
- `genres: Vec<String>` - Movie genres
- `cast: Vec<MoviePerson>` - Cast members
- `crew: Vec<MoviePerson>` - Crew members
- `popular_reviews: Vec<MovieReview>` - Popular reviews

#### Methods

- `get_watchers() -> Result<HashMap<String, Value>>` - Get users who watched
- `get_reviews() -> Result<HashMap<String, Value>>` - Get movie reviews
- `get_similar() -> Result<HashMap<String, Value>>` - Get similar movies

### Search

The `Search` struct provides search functionality across different content types.

```rust
use rustboxd::Search;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Search films
    let film_search = Search::new("blade runner", Some("films")).await?;
    println!("Found {} films", film_search.results.films.len());
    
    // Search reviews
    let review_search = Search::new("masterpiece", Some("reviews")).await?;
    
    // Search without filter (all types)
    let general_search = Search::new("kubrick", None).await?;
    
    Ok(())
}
```

#### Fields

- `query: String` - Search query
- `search_filter: Option<String>` - Content type filter
- `url: String` - Search URL
- `results: SearchResults` - Search results

#### Available Filters

- `"films"` - Search movies
- `"reviews"` - Search reviews
- `"lists"` - Search lists
- `"original-lists"` - Search original lists only
- `"stories"` - Search stories
- `"cast-crew"` - Search cast and crew
- `"members"` - Search users
- `"tags"` - Search tags
- `"articles"` - Search articles
- `"episodes"` - Search episodes
- `"full-text"` - Full text search

### Films

The `Films` struct handles collections of movies with pagination support.

```rust
use rustboxd::Films;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let films = Films::new("https://letterboxd.com/films/year/2024/").await?;
    
    println!("Found {} films", films.count);
    
    // Filter by year
    let films_2024 = films.filter_by_year(2024);
    
    // Filter by rating
    let high_rated = films.filter_by_rating(4.0);
    
    Ok(())
}
```

#### Methods

- `filter_by_year(year: i32) -> Vec<&FilmEntry>` - Filter by release year
- `filter_by_rating(min_rating: f32) -> Vec<&FilmEntry>` - Filter by minimum rating
- `get_watched() -> Vec<&FilmEntry>` - Get watched films
- `get_liked() -> Vec<&FilmEntry>` - Get liked films
- `get_in_watchlist() -> Vec<&FilmEntry>` - Get watchlisted films

### List

The `List` struct represents user-created film lists.

```rust
use rustboxd::List;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let list = List::new("username", "list-slug").await?;
    
    println!("Title: {}", list.title);
    println!("Films: {}", list.film_count);
    
    // Get list comments
    let comments = list.get_comments().await?;
    
    // Find specific film
    if let Some(film) = list.get_film_by_position(1) {
        println!("First film: {}", film.title);
    }
    
    Ok(())
}
```

#### Fields

- `title: String` - List title
- `description: Option<String>` - List description
- `author: String` - List creator username
- `slug: String` - List slug
- `url: String` - Full list URL
- `film_count: u32` - Number of films
- `likes: u32` - Number of likes
- `comments: u32` - Number of comments
- `is_ranked: bool` - Whether list is ranked
- `films: Vec<ListFilm>` - Films in the list
- `tags: Vec<String>` - List tags

#### Methods

- `from_url(url: &str) -> Result<Self>` - Create from full URL
- `get_comments() -> Result<Vec<ListComment>>` - Get list comments
- `get_film_by_position(position: u32) -> Option<&ListFilm>` - Get film at position
- `get_films_by_year(year: i32) -> Vec<&ListFilm>` - Filter by year

## Utilities

### Parser Functions

```rust
use rustboxd::utils::*;

// Parse shorthand numbers
let count = extract_and_convert_shorthand("1.2K"); // Returns 1200

// Parse ratings
let rating = parse_rating("4.5/5"); // Returns Some(4.5)

// Parse runtime
let minutes = parse_runtime("2h 22m"); // Returns Some(142)

// Clean text
let clean = clean_text("  Multiple   spaces  "); // Returns "Multiple spaces"

// Extract slugs
let slug = extract_film_slug("https://letterboxd.com/film/the-matrix/");
```

### Validation Functions

```rust
use rustboxd::utils::*;

// Validate inputs
assert!(is_valid_username("user123"));
assert!(is_valid_rating(4.5));
assert!(is_valid_year(2024));

// Sanitize for URLs
let slug = sanitize_for_url("The Matrix: Reloaded"); // Returns "the-matrix-reloaded"

// Check safety
assert!(is_safe_text("Normal text"));
assert!(!is_safe_text("<script>alert('xss')</script>"));
```

### Transform Functions

```rust
use rustboxd::utils::*;

// Build URLs
let url = build_user_url("username");
let film_url = build_film_url("the-matrix");
let search_url = build_search_url("query", Some("films"));

// Handle pagination
let ajax_url = get_ajax_url("https://letterboxd.com/films/");
let page_url = add_page_to_url(&ajax_url, 2);
```

## Best Practices

### Error Handling

Always handle errors appropriately:

```rust
use rustboxd::{User, Error};

async fn get_user_safely(username: &str) -> Option<User> {
    match User::new(username).await {
        Ok(user) => Some(user),
        Err(Error::InvalidUsername(_)) => {
            eprintln!("Invalid username format");
            None
        }
        Err(Error::PageLoad { .. }) => {
            eprintln!("User not found or network error");
            None
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            None
        }
    }
}
```

### Rate Limiting

Implement rate limiting to respect Letterboxd's servers:

```rust
use tokio::time::{sleep, Duration};

async fn fetch_multiple_users(usernames: &[&str]) -> Vec<Option<User>> {
    let mut results = Vec::new();
    
    for username in usernames {
        match User::new(username).await {
            Ok(user) => results.push(Some(user)),
            Err(_) => results.push(None),
        }
        
        // Rate limit: wait 1 second between requests
        sleep(Duration::from_secs(1)).await;
    }
    
    results
}
```

### Memory Efficiency

For large datasets, consider streaming or pagination:

```rust
async fn process_large_list(list_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let list = List::from_url(list_url).await?;
    
    // Process films in chunks to avoid memory issues
    for chunk in list.films.chunks(100) {
        for film in chunk {
            // Process each film
            println!("Processing: {}", film.title);
        }
        
        // Optional: yield to other tasks
        tokio::task::yield_now().await;
    }
    
    Ok(())
}
```

### Caching

Consider caching frequently accessed data:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

struct UserCache {
    cache: Arc<Mutex<HashMap<String, User>>>,
}

impl UserCache {
    fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    async fn get_user(&self, username: &str) -> Result<User, Box<dyn std::error::Error>> {
        let mut cache = self.cache.lock().await;
        
        if let Some(user) = cache.get(username) {
            return Ok(user.clone());
        }
        
        let user = User::new(username).await?;
        cache.insert(username.to_string(), user.clone());
        Ok(user)
    }
}
```
