# Rustboxd

A fast, async Rust library for scraping Letterboxd data.

[![Crates.io](https://img.shields.io/crates/v/rustboxd.svg)](https://crates.io/crates/rustboxd)
[![Documentation](https://docs.rs/rustboxd/badge.svg)](https://docs.rs/rustboxd)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Async/Await Support**: Built on tokio for high-performance concurrent operations
- **Type Safety**: Full Rust type safety with serde serialization
- **Comprehensive API**: Access users, movies, films, lists, and search functionality
- **Error Handling**: Detailed error types with context
- **Rate Limiting**: Built-in respect for server resources
- **No Dependencies on Python**: Pure Rust implementation

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustboxd = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Basic Usage

```rust
use rustboxd::{User, Movie, Search};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get user information
    let user = User::new("username").await?;
    println!("Hello, {}!", user.display_name);
    
    // Get movie details
    let movie = Movie::new("the-matrix").await?;
    println!("Movie: {} ({})", movie.title, movie.year.unwrap_or(0));
    
    // Search for content
    let search = Search::new("kubrick", Some("films")).await?;
    println!("Found {} Kubrick films", search.results.films.len());
    
    Ok(())
}
}
```

## User Operations

### Basic User Information

```rust
use rustboxd::User;

let user = User::new("username").await?;

// Basic profile information
println!("Display Name: {}", user.display_name);
println!("Bio: {}", user.bio.unwrap_or_default());
println!("Location: {}", user.location.unwrap_or_default());

// Statistics
if let Some(stats) = &user.stats {
    println!("Films: {}", stats.films);
    println!("Reviews: {}", stats.reviews);
    println!("Lists: {}", stats.lists);
}
```

### User Film Data

```rust
// Get all films watched by user
let films = user.get_films().await?;
println!("Total films: {}", films.len());

// Get films by specific rating
let five_star_films = user.get_films_by_rating(5.0).await?;
println!("5-star films: {}", five_star_films.len());

// Get watchlist
let watchlist = user.get_watchlist().await?;
println!("Watchlist: {}", watchlist.len());

// Get liked films
let liked_films = user.get_films_liked().await?;
println!("Liked films: {}", liked_films.len());
```

### User Activity and Social

```rust
// Get user's diary
let diary = user.get_diary(Some(2024), None, None, None).await?;
println!("2024 diary entries: {}", diary.len());

// Get reviews
let reviews = user.get_reviews().await?;
println!("Reviews written: {}", reviews.len());

// Get social connections
let followers = user.get_followers().await?;
let following = user.get_following().await?;
println!("Followers: {}, Following: {}", followers.len(), following.len());

// Get lists created by user
let lists = user.get_lists().await?;
println!("Lists created: {}", lists.len());
```

## Movie Operations

### Movie Details

```rust
use rustboxd::Movie;

let movie = Movie::new("the-matrix").await?;

// Basic information
println!("Title: {}", movie.title);
println!("Year: {}", movie.year.unwrap_or(0));
println!("Runtime: {} minutes", movie.runtime.unwrap_or(0));
println!("Rating: {:.1}/5", movie.rating.unwrap_or(0.0));

// Genres and themes
println!("Genres: {}", movie.genres.join(", "));
println!("Themes: {}", movie.themes.join(", "));

// Cast and crew
println!("Cast: {} people", movie.cast.len());
println!("Crew: {} people", movie.crew.len());
```

### Movie Community Data

```rust
// Get reviews for the movie
let reviews = movie.get_reviews().await?;
println!("Reviews: {}", reviews.len());

// Get similar films
let similar = movie.get_similar().await?;
println!("Similar films: {}", similar.len());

// Get users who watched this movie
let watchers = movie.get_watchers().await?;
println!("Watchers: {}", watchers.len());
```

## Search Operations

### Film Search

```rust
use rustboxd::Search;

// Search for films
let search = Search::new("christopher nolan", Some("films")).await?;
println!("Found {} films", search.results.films.len());

for film in search.results.films.iter().take(5) {
    println!("- {} ({})", film.title, film.year.unwrap_or(0));
}
```

### Multi-type Search

```rust
// Search all content types
let search = Search::new("kubrick", None).await?;

println!("Search results for 'kubrick':");
println!("- Films: {}", search.results.films.len());
println!("- Reviews: {}", search.results.reviews.len());
println!("- Lists: {}", search.results.lists.len());
println!("- Members: {}", search.results.members.len());
```

## Lists

### List Information

```rust
use rustboxd::List;

let list = List::new("author", "list-slug").await?;

println!("List: {}", list.title);
println!("Author: {}", list.author);
println!("Films: {}", list.film_count);
println!("Likes: {}", list.likes);

// Access films in the list
for film in list.films.iter().take(5) {
    println!("{}. {}", film.position.unwrap_or(0), film.title);
}
```

### List Comments

```rust
// Get comments on the list
let comments = list.get_comments().await?;
println!("Comments: {}", comments.len());

for comment in comments.iter().take(3) {
    println!("{}: {}", comment.author, comment.content);
}
```

## Films Collection

### Collection Analysis

```rust
use rustboxd::Films;

let films = Films::new("https://letterboxd.com/films/popular/").await?;

println!("Total films: {}", films.count);

// Filter by rating
let high_rated = films.filter_by_rating(4.0);
println!("Films rated 4.0+: {}", high_rated.len());

// Filter by year
let recent = films.filter_by_year(2020);
println!("Films from 2020+: {}", recent.len());

// Filter by genre
let horror = films.filter_by_genre("Horror");
println!("Horror films: {}", horror.len());
```

## Error Handling

Rustboxd provides detailed error types:

```rust
use rustboxd::{User, Error};

match User::new("username").await {
    Ok(user) => println!("Success: {}", user.display_name),
    Err(Error::InvalidUsername(username)) => {
        println!("Invalid username format: {}", username);
    }
    Err(Error::UserNotFound(username)) => {
        println!("User not found: {}", username);
    }
    Err(Error::PageLoad { url, message }) => {
        println!("Failed to load page {}: {}", url, message);
    }
    Err(Error::ParseError { field, message }) => {
        println!("Parse error in {}: {}", field, message);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Async Best Practices

### Concurrent Operations

```rust
use tokio;

// Run multiple operations concurrently
let (user, movie, search) = tokio::try_join!(
    User::new("username"),
    Movie::new("the-matrix"),
    Search::new("kubrick", Some("films"))
)?;
```

### Rate Limiting

```rust
use tokio::time::{sleep, Duration};

// Be respectful to the server
for username in usernames {
    let user = User::new(username).await?;
    println!("Loaded: {}", user.display_name);
    
    // Wait between requests
    sleep(Duration::from_millis(500)).await;
}
```

## Advanced Examples

For more comprehensive examples, see the [examples documentation](docs/examples.md) and [performance guide](docs/performance.md).

## Documentation

- [API Documentation](docs/api.md) - Detailed API reference
- [Examples](docs/examples.md) - Comprehensive usage examples
- [Performance Guide](docs/performance.md) - Optimization and best practices

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This library is for educational and personal use. Please respect Letterboxd's robots.txt and terms of service. Consider implementing appropriate rate limiting and caching to minimize server load.
    
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
