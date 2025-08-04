# Rustboxd

A comprehensive Letterboxd web scraper and API client library written in Rust, providing type-safe access to Letterboxd data.

## Features

- **User Profiles & Statistics** - Get detailed user information, watching statistics, and preferences
- **Movie Details & Metadata** - Access comprehensive film information including cast, crew, ratings, and reviews  
- **Search Functionality** - Search films, users, reviews, lists, and more with filters
- **Diary Entries & Watchlists** - Access user viewing history and watchlist data
- **Film Collections & Lists** - Browse and analyze user-created film lists
- **User Activity & Reviews** - Track user activity, reviews, and social interactions
- **Async/Await Support** - Built with modern async Rust for high performance
- **Type-Safe API** - Leverage Rust's type system for reliable data handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustboxd = "0.1.0"
```

## Quick Start

```rust
use rustboxd::{User, Movie, Search};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get user information
    let user = User::new("nmcassa").await?;
    println!("User: {}", user.display_name);
    println!("Films watched: {}", user.stats.map(|s| s.films).unwrap_or(0));
    
    // Get movie details
    let movie = Movie::new("the-matrix").await?;
    println!("Movie: {} ({})", movie.title, movie.year.unwrap_or(0));
    println!("Genres: {:?}", movie.genres);
    
    // Search for films
    let search = Search::new("pulp fiction", Some("films")).await?;
    println!("Found {} results", search.results.films.len());
    
    // Get user's diary entries
    let diary = user.get_diary(Some(2024), None, None, None).await?;
    println!("Diary entries: {}", diary.len());
    
    Ok(())
}
```

## Advanced Usage

### User Analysis

```rust
use rustboxd::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new("username").await?;
    
    // Get detailed user data
    let films = user.get_films().await?;
    let reviews = user.get_reviews().await?;
    let watchlist = user.get_watchlist().await?;
    let following = user.get_following().await?;
    
    println!("User has {} films, {} reviews", films.len(), reviews.len());
    
    // Get films by rating
    let five_star_films = user.get_films_by_rating(5.0).await?;
    println!("5-star films: {}", five_star_films.len());
    
    // Get genre preferences
    let genre_info = user.get_genre_info().await?;
    println!("Genre data: {:?}", genre_info);
    
    Ok(())
}
```

### Movie Information

```rust
use rustboxd::Movie;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let movie = Movie::new("parasite-2019").await?;
    
    println!("Title: {}", movie.title);
    println!("Director: {:?}", movie.details.as_ref().map(|d| &d.director));
    println!("Cast: {}", movie.cast.len());
    println!("Rating: {:?}", movie.rating);
    
    // Get additional movie data
    let reviews = movie.get_reviews().await?;
    let similar = movie.get_similar().await?;
    
    Ok(())
}
```

### Search and Discovery

```rust
use rustboxd::Search;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Search films
    let film_search = Search::new("christopher nolan", Some("films")).await?;
    
    // Search reviews
    let review_search = Search::new("masterpiece", Some("reviews")).await?;
    
    // Search users
    let user_search = Search::new("film critic", Some("members")).await?;
    
    // Search lists
    let list_search = Search::new("best of 2024", Some("lists")).await?;
    
    Ok(())
}
```

## Project Structure

```
rustboxd/
├── src/
│   ├── core/           # Core functionality (HTTP client, errors, constants)
│   ├── models/         # Data models (User, Movie, Search, Films, List)
│   ├── pages/          # Page-specific scrapers and parsers
│   ├── utils/          # Utility functions for parsing and validation
│   └── lib.rs         # Library entry point
├── examples/           # Usage examples
├── tests/             # Integration and unit tests
└── docs/              # Documentation
```

## Features by Module

### Core
- HTTP client with proper headers and error handling
- Comprehensive error types with detailed messages
- Constants for Letterboxd URLs, selectors, and validation

### Models
- **User**: Complete user profile data and statistics
- **Movie**: Full film metadata including cast, crew, and reviews
- **Search**: Multi-type search with filtering capabilities
- **Films**: Collection handling with pagination support
- **List**: User list data with film entries and metadata

### Pages
- Individual page scrapers for different Letterboxd sections
- Specialized parsers for user activity, diary, films, reviews, etc.
- Async implementation for concurrent data fetching

### Utils
- HTML parsing utilities with error handling
- Data transformation and normalization functions
- Input validation and sanitization
- URL building and manipulation helpers

## Error Handling

Rustboxd provides comprehensive error handling:

```rust
use rustboxd::{User, Error};

match User::new("invalid-user").await {
    Ok(user) => println!("User found: {}", user.display_name),
    Err(Error::PageLoad { url, message }) => {
        println!("Failed to load {}: {}", url, message);
    }
    Err(Error::InvalidUsername(username)) => {
        println!("Invalid username format: {}", username);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Performance

- Async/await for non-blocking I/O operations
- HTTP connection reuse for multiple requests
- Efficient HTML parsing with the `scraper` crate
- Memory-efficient data structures with `serde`

## Validation

Built-in validation for:
- Usernames (alphanumeric and underscore only)
- Film slugs (lowercase letters, numbers, hyphens)
- Ratings (0.5-5.0 in 0.5 increments)
- Search filters (predefined valid options)
- URLs and date formats

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Development

1. Clone the repository:
   ```bash
   git clone https://github.com/Pranav-Karra-3301/rustboxd.git
   cd rustboxd
   ```

2. Install Rust and dependencies:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run examples:
   ```bash
   cargo run --example basic_usage
   cargo run --example advanced_usage
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an unofficial library for educational and research purposes. Please respect Letterboxd's terms of service and implement appropriate rate limiting in your applications.

## Related Projects

- [letterboxdpy](https://github.com/nmcassa/letterboxdpy) - Python library that inspired this project
- [letterboxd-api](https://api-docs.letterboxd.com/) - Official Letterboxd API (requires authentication)
letterboxd webscraper in rust
