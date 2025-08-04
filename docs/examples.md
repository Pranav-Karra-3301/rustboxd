# Examples and Use Cases

This document provides comprehensive examples for using Rustboxd in various scenarios.

## Basic Usage

### Getting Started

```rust
use rustboxd::{User, Movie, Search};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a user
    let user = User::new("username").await?;
    println!("Hello, {}!", user.display_name);
    
    // Get a movie
    let movie = Movie::new("the-matrix").await?;
    println!("Movie: {} ({})", movie.title, movie.year.unwrap_or(0));
    
    // Search for content
    let search = Search::new("kubrick", Some("films")).await?;
    println!("Found {} Kubrick films", search.results.films.len());
    
    Ok(())
}
```

## User Analysis

### Profile Statistics

```rust
use rustboxd::User;

async fn analyze_user_profile(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(username).await?;
    
    println!("=== User Profile: {} ===", user.display_name);
    println!("Username: {}", user.username);
    println!("Profile URL: {}", user.url);
    
    if let Some(bio) = &user.bio {
        println!("Bio: {}", bio);
    }
    
    if let Some(location) = &user.location {
        println!("Location: {}", location);
    }
    
    if let Some(stats) = &user.stats {
        println!("\n=== Statistics ===");
        println!("Films watched: {}", stats.films);
        println!("Reviews written: {}", stats.reviews);
        println!("Lists created: {}", stats.lists);
        println!("Followers: {}", stats.followers);
        println!("Following: {}", stats.following);
    }
    
    Ok(())
}
```

### Viewing History Analysis

```rust
use rustboxd::User;
use std::collections::HashMap;

async fn analyze_viewing_patterns(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(username).await?;
    
    // Get current year diary
    let diary_2024 = user.get_diary(Some(2024), None, None, None).await?;
    println!("Films watched in 2024: {}", diary_2024.len());
    
    // Analyze rating distribution
    let mut rating_counts = HashMap::new();
    for rating in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0] {
        let films = user.get_films_by_rating(rating).await?;
        rating_counts.insert(rating, films.len());
    }
    
    println!("\n=== Rating Distribution ===");
    for (rating, count) in rating_counts {
        println!("{:.1} stars: {} films", rating, count);
    }
    
    // Genre preferences
    let genre_info = user.get_genre_info().await?;
    println!("\n=== Genre Statistics ===");
    println!("Genre data entries: {}", genre_info.len());
    
    Ok(())
}
```

### Social Network Analysis

```rust
use rustboxd::User;

async fn analyze_social_network(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(username).await?;
    
    // Get social connections
    let followers = user.get_followers().await?;
    let following = user.get_following().await?;
    
    println!("=== Social Network for {} ===", user.display_name);
    println!("Followers: {}", followers.len());
    println!("Following: {}", following.len());
    
    // Calculate follow ratio
    if followers.len() > 0 {
        let ratio = following.len() as f64 / followers.len() as f64;
        println!("Follow ratio: {:.2}", ratio);
    }
    
    // Get activity
    let activity = user.get_activity().await?;
    println!("Recent activity entries: {}", activity.len());
    
    // Get reviews and likes
    let reviews = user.get_reviews().await?;
    let liked_reviews = user.get_liked_reviews().await?;
    
    println!("\n=== Content Creation ===");
    println!("Reviews written: {}", reviews.len());
    println!("Reviews liked: {}", liked_reviews.len());
    
    Ok(())
}
```

## Movie Analysis

### Detailed Movie Information

```rust
use rustboxd::Movie;

async fn analyze_movie(slug: &str) -> Result<(), Box<dyn std::error::Error>> {
    let movie = Movie::new(slug).await?;
    
    println!("=== Movie Analysis: {} ===", movie.title);
    
    // Basic information
    if let Some(year) = movie.year {
        println!("Release Year: {}", year);
    }
    
    if let Some(runtime) = movie.runtime {
        println!("Runtime: {} minutes ({:.1} hours)", runtime, runtime as f64 / 60.0);
    }
    
    if let Some(rating) = movie.rating {
        println!("Average Rating: {:.1}/5.0", rating);
    }
    
    // Genres
    if !movie.genres.is_empty() {
        println!("Genres: {}", movie.genres.join(", "));
    }
    
    // Cast and crew
    println!("\n=== Cast & Crew ===");
    println!("Cast members: {}", movie.cast.len());
    println!("Crew members: {}", movie.crew.len());
    
    // Show top 5 cast members
    for (i, person) in movie.cast.iter().take(5).enumerate() {
        if let Some(role) = &person.role_name {
            println!("{}. {} as {}", i + 1, person.name, role);
        } else {
            println!("{}. {}", i + 1, person.name);
        }
    }
    
    // Additional data
    let reviews = movie.get_reviews().await?;
    let similar = movie.get_similar().await?;
    
    println!("\n=== Community Data ===");
    println!("Reviews: {}", reviews.len());
    println!("Similar films: {}", similar.len());
    
    Ok(())
}
```

### Movie Collection Analysis

```rust
use rustboxd::Films;

async fn analyze_film_collection(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let films = Films::new(url).await?;
    
    println!("=== Film Collection Analysis ===");
    println!("Total films: {}", films.count);
    
    // Analyze by decade
    let mut decade_counts = std::collections::HashMap::new();
    for film in films.movies.values() {
        if let Some(year) = film.year {
            let decade = (year / 10) * 10;
            *decade_counts.entry(decade).or_insert(0) += 1;
        }
    }
    
    println!("\n=== Films by Decade ===");
    let mut decades: Vec<_> = decade_counts.iter().collect();
    decades.sort_by_key(|(decade, _)| *decade);
    
    for (decade, count) in decades {
        println!("{}s: {} films", decade, count);
    }
    
    // Find highest rated films
    let high_rated: Vec<_> = films.filter_by_rating(4.5);
    println!("\n=== Highly Rated Films (4.5+) ===");
    for film in high_rated.iter().take(10) {
        println!("{} ({}) - {:.1}/5", 
            film.title, 
            film.year.unwrap_or(0),
            film.rating.unwrap_or(0.0)
        );
    }
    
    Ok(())
}
```

## Search and Discovery

### Advanced Search

```rust
use rustboxd::Search;

async fn advanced_search_example() -> Result<(), Box<dyn std::error::Error>> {
    // Search for films by director
    let nolan_films = Search::new("christopher nolan", Some("films")).await?;
    println!("Christopher Nolan films found: {}", nolan_films.results.films.len());
    
    // Search for reviews containing specific terms
    let reviews = Search::new("masterpiece cinematography", Some("reviews")).await?;
    println!("Reviews about cinematography: {}", reviews.results.reviews.len());
    
    // Search for lists
    let lists = Search::new("best of 2024", Some("lists")).await?;
    println!("'Best of 2024' lists: {}", lists.results.lists.len());
    
    // Search for users
    let critics = Search::new("film critic", Some("members")).await?;
    println!("Film critics found: {}", critics.results.members.len());
    
    // General search (all content types)
    let general = Search::new("blade runner", None).await?;
    println!("General 'Blade Runner' results:");
    println!("  Films: {}", general.results.films.len());
    println!("  Reviews: {}", general.results.reviews.len());
    println!("  Lists: {}", general.results.lists.len());
    
    Ok(())
}
```

### Discovery Workflow

```rust
use rustboxd::{Search, Movie, User};

async fn discover_films_workflow(genre: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Film Discovery: {} ===", genre);
    
    // 1. Search for films in the genre
    let search = Search::new(genre, Some("films")).await?;
    println!("Found {} {} films", search.results.films.len(), genre);
    
    // 2. Analyze top results
    for (i, film_result) in search.results.films.iter().take(5).enumerate() {
        println!("\n{}. Analyzing: {}", i + 1, film_result.title);
        
        // Get detailed movie information
        if let Ok(movie) = Movie::new(&film_result.slug).await {
            if let Some(rating) = movie.rating {
                println!("   Rating: {:.1}/5", rating);
            }
            
            if !movie.genres.is_empty() {
                println!("   Genres: {}", movie.genres.join(", "));
            }
            
            // Get similar films
            let similar = movie.get_similar().await?;
            println!("   Similar films available: {}", similar.len());
        }
        
        // Rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // 3. Find related lists
    let lists = Search::new(&format!("best {} films", genre), Some("lists")).await?;
    println!("\nFound {} related lists", lists.results.lists.len());
    
    Ok(())
}
```

## List Management

### List Analysis

```rust
use rustboxd::List;

async fn analyze_film_list(author: &str, slug: &str) -> Result<(), Box<dyn std::error::Error>> {
    let list = List::new(author, slug).await?;
    
    println!("=== List Analysis: {} ===", list.title);
    println!("Created by: {}", list.author);
    println!("Films: {}", list.film_count);
    println!("Likes: {}", list.likes);
    println!("Comments: {}", list.comments);
    
    if let Some(description) = &list.description {
        println!("Description: {}", description);
    }
    
    // Analyze films in the list
    println!("\n=== Films Analysis ===");
    
    // Group by decade
    let mut decade_distribution = std::collections::HashMap::new();
    for film in &list.films {
        if let Some(year) = film.year {
            let decade = (year / 10) * 10;
            *decade_distribution.entry(decade).or_insert(0) += 1;
        }
    }
    
    println!("Distribution by decade:");
    for (decade, count) in decade_distribution {
        println!("  {}s: {} films", decade, count);
    }
    
    // Show top 10 films
    println!("\n=== Top 10 Films ===");
    for film in list.films.iter().take(10) {
        if let Some(pos) = film.position {
            println!("{}. {} ({})", pos, film.title, film.year.unwrap_or(0));
        }
    }
    
    // Get comments
    let comments = list.get_comments().await?;
    println!("\n=== Comments ===");
    println!("Total comments: {}", comments.len());
    
    for comment in comments.iter().take(3) {
        println!("- {}: {}", comment.author, comment.content);
    }
    
    Ok(())
}
```

### List Comparison

```rust
use rustboxd::List;
use std::collections::HashSet;

async fn compare_lists(
    list1_author: &str, list1_slug: &str,
    list2_author: &str, list2_slug: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let list1 = List::new(list1_author, list1_slug).await?;
    let list2 = List::new(list2_author, list2_slug).await?;
    
    println!("=== List Comparison ===");
    println!("List 1: {} ({} films)", list1.title, list1.film_count);
    println!("List 2: {} ({} films)", list2.title, list2.film_count);
    
    // Extract film slugs
    let films1: HashSet<String> = list1.films.iter().map(|f| f.slug.clone()).collect();
    let films2: HashSet<String> = list2.films.iter().map(|f| f.slug.clone()).collect();
    
    // Find overlaps
    let common: HashSet<_> = films1.intersection(&films2).collect();
    let unique_to_1: HashSet<_> = films1.difference(&films2).collect();
    let unique_to_2: HashSet<_> = films2.difference(&films1).collect();
    
    println!("\n=== Comparison Results ===");
    println!("Films in common: {}", common.len());
    println!("Unique to '{}': {}", list1.title, unique_to_1.len());
    println!("Unique to '{}': {}", list2.title, unique_to_2.len());
    
    // Calculate similarity percentage
    let total_unique = films1.union(&films2).count();
    let similarity = (common.len() as f64 / total_unique as f64) * 100.0;
    println!("Similarity: {:.1}%", similarity);
    
    // Show common films
    if !common.is_empty() {
        println!("\n=== Common Films ===");
        for (i, slug) in common.iter().take(10).enumerate() {
            if let Some(film) = list1.films.iter().find(|f| &f.slug == *slug) {
                println!("{}. {}", i + 1, film.title);
            }
        }
    }
    
    Ok(())
}
```

## Data Export and Analysis

### User Data Export

```rust
use rustboxd::User;
use serde_json;
use std::fs;

async fn export_user_data(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(username).await?;
    
    println!("Exporting data for user: {}", user.display_name);
    
    // Create export directory
    let export_dir = format!("exports/{}", username);
    fs::create_dir_all(&export_dir)?;
    
    // Export basic profile
    let profile_json = serde_json::to_string_pretty(&user)?;
    fs::write(format!("{}/profile.json", export_dir), profile_json)?;
    
    // Export films
    let films = user.get_films().await?;
    let films_json = serde_json::to_string_pretty(&films)?;
    fs::write(format!("{}/films.json", export_dir), films_json)?;
    
    // Export diary
    let diary = user.get_diary(None, None, None, None).await?;
    let diary_json = serde_json::to_string_pretty(&diary)?;
    fs::write(format!("{}/diary.json", export_dir), diary_json)?;
    
    // Export reviews
    let reviews = user.get_reviews().await?;
    let reviews_json = serde_json::to_string_pretty(&reviews)?;
    fs::write(format!("{}/reviews.json", export_dir), reviews_json)?;
    
    // Export lists
    let lists = user.get_lists().await?;
    let lists_json = serde_json::to_string_pretty(&lists)?;
    fs::write(format!("{}/lists.json", export_dir), lists_json)?;
    
    // Export social data
    let followers = user.get_followers().await?;
    let following = user.get_following().await?;
    
    let social_data = serde_json::json!({
        "followers": followers,
        "following": following
    });
    fs::write(format!("{}/social.json", export_dir), 
              serde_json::to_string_pretty(&social_data)?)?;
    
    println!("Export completed to: {}", export_dir);
    
    Ok(())
}
```

### Rating Analysis

```rust
use rustboxd::User;
use std::collections::HashMap;

async fn analyze_rating_patterns(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(username).await?;
    
    println!("=== Rating Analysis for {} ===", user.display_name);
    
    // Get all rating categories
    let mut rating_distribution = HashMap::new();
    let mut total_films = 0;
    
    for rating in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0] {
        let films = user.get_films_by_rating(rating).await?;
        rating_distribution.insert(rating, films.len());
        total_films += films.len();
        
        // Small delay to be respectful
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
    
    // Get unrated films
    let unrated = user.get_films_not_rated().await?;
    
    println!("Total rated films: {}", total_films);
    println!("Unrated films: {}", unrated.len());
    
    println!("\n=== Rating Distribution ===");
    for rating in [5.0, 4.5, 4.0, 3.5, 3.0, 2.5, 2.0, 1.5, 1.0, 0.5] {
        let count = rating_distribution[&rating];
        let percentage = if total_films > 0 {
            (count as f64 / total_films as f64) * 100.0
        } else {
            0.0
        };
        
        println!("{:.1} ★: {:4} films ({:5.1}%)", rating, count, percentage);
    }
    
    // Calculate average rating
    let mut total_score = 0.0;
    for (rating, count) in rating_distribution {
        total_score += rating * count as f64;
    }
    
    let average_rating = if total_films > 0 {
        total_score / total_films as f64
    } else {
        0.0
    };
    
    println!("\nAverage rating: {:.2}/5.0", average_rating);
    
    // Calculate rating generosity
    let high_ratings = rating_distribution[&4.0] + rating_distribution[&4.5] + rating_distribution[&5.0];
    let low_ratings = rating_distribution[&0.5] + rating_distribution[&1.0] + rating_distribution[&1.5];
    
    println!("High ratings (4.0+): {} ({:.1}%)", 
             high_ratings, 
             (high_ratings as f64 / total_films as f64) * 100.0);
    println!("Low ratings (≤1.5): {} ({:.1}%)", 
             low_ratings, 
             (low_ratings as f64 / total_films as f64) * 100.0);
    
    Ok(())
}
```

## Batch Operations

### Multiple User Comparison

```rust
use rustboxd::User;
use std::collections::HashMap;

async fn compare_multiple_users(usernames: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multi-User Comparison ===");
    
    let mut users = Vec::new();
    
    // Fetch all users
    for username in usernames {
        match User::new(username).await {
            Ok(user) => {
                println!("✓ Loaded: {}", user.display_name);
                users.push(user);
            }
            Err(e) => {
                println!("✗ Failed to load {}: {}", username, e);
            }
        }
        
        // Rate limiting
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    println!("\n=== Comparison Results ===");
    
    // Compare statistics
    for user in &users {
        if let Some(stats) = &user.stats {
            println!("{}: {} films, {} reviews, {} followers", 
                     user.display_name, stats.films, stats.reviews, stats.followers);
        }
    }
    
    // Find mutual follows
    println!("\n=== Social Connections ===");
    for (i, user1) in users.iter().enumerate() {
        for user2 in users.iter().skip(i + 1) {
            // Check if they follow each other
            let user1_following = user1.get_following().await?;
            let user2_following = user2.get_following().await?;
            
            // This is simplified - in practice you'd need to parse the following data
            println!("Checking connection between {} and {}", 
                     user1.display_name, user2.display_name);
        }
    }
    
    Ok(())
}
```

### Batch Film Analysis

```rust
use rustboxd::Movie;

async fn analyze_film_series(film_slugs: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Film Series Analysis ===");
    
    let mut movies = Vec::new();
    
    for slug in film_slugs {
        match Movie::new(slug).await {
            Ok(movie) => {
                println!("✓ Loaded: {}", movie.title);
                movies.push(movie);
            }
            Err(e) => {
                println!("✗ Failed to load {}: {}", slug, e);
            }
        }
        
        // Rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Analyze the series
    println!("\n=== Series Overview ===");
    
    let mut total_runtime = 0;
    let mut ratings = Vec::new();
    let mut years = Vec::new();
    
    for (i, movie) in movies.iter().enumerate() {
        println!("{}. {} ({})", i + 1, movie.title, movie.year.unwrap_or(0));
        
        if let Some(runtime) = movie.runtime {
            total_runtime += runtime;
            println!("   Runtime: {} minutes", runtime);
        }
        
        if let Some(rating) = movie.rating {
            ratings.push(rating);
            println!("   Rating: {:.1}/5", rating);
        }
        
        if let Some(year) = movie.year {
            years.push(year);
        }
        
        println!("   Genres: {}", movie.genres.join(", "));
    }
    
    // Calculate statistics
    println!("\n=== Series Statistics ===");
    println!("Total films: {}", movies.len());
    println!("Total runtime: {} minutes ({:.1} hours)", total_runtime, total_runtime as f64 / 60.0);
    
    if !ratings.is_empty() {
        let avg_rating = ratings.iter().sum::<f32>() / ratings.len() as f32;
        println!("Average rating: {:.2}/5", avg_rating);
        
        let highest = ratings.iter().fold(0.0, |a, &b| a.max(b));
        let lowest = ratings.iter().fold(5.0, |a, &b| a.min(b));
        println!("Rating range: {:.1} - {:.1}", lowest, highest);
    }
    
    if !years.is_empty() {
        years.sort();
        let span = years.last().unwrap() - years.first().unwrap();
        println!("Release span: {} - {} ({} years)", 
                 years.first().unwrap(), years.last().unwrap(), span);
    }
    
    Ok(())
}
```

## Error Handling Patterns

### Robust Data Fetching

```rust
use rustboxd::{User, Error};
use tokio::time::{sleep, Duration};

async fn robust_user_fetch(username: &str, max_retries: u32) -> Option<User> {
    for attempt in 1..=max_retries {
        match User::new(username).await {
            Ok(user) => {
                println!("✓ Successfully loaded user: {}", user.display_name);
                return Some(user);
            }
            Err(Error::PageLoad { url, message }) => {
                println!("⚠ Attempt {}: Page load failed - {}", attempt, message);
                if attempt < max_retries {
                    sleep(Duration::from_secs(attempt as u64)).await;
                }
            }
            Err(Error::InvalidUsername(username)) => {
                println!("✗ Invalid username format: {}", username);
                return None; // Don't retry for invalid usernames
            }
            Err(e) => {
                println!("⚠ Attempt {}: Unexpected error - {}", attempt, e);
                if attempt < max_retries {
                    sleep(Duration::from_secs(attempt as u64)).await;
                }
            }
        }
    }
    
    println!("✗ Failed to load user after {} attempts", max_retries);
    None
}
```

### Graceful Degradation

```rust
use rustboxd::User;

async fn user_summary_with_fallbacks(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(username).await?;
    
    println!("=== User Summary: {} ===", user.display_name);
    
    // Try to get various data with fallbacks
    
    // Basic profile (always available)
    println!("Profile URL: {}", user.url);
    
    // Films (with fallback)
    match user.get_films().await {
        Ok(films) => println!("Films watched: {}", films.len()),
        Err(e) => println!("Films data unavailable: {}", e),
    }
    
    // Diary (with fallback)
    match user.get_diary(Some(2024), None, None, None).await {
        Ok(diary) => println!("2024 diary entries: {}", diary.len()),
        Err(e) => println!("Diary data unavailable: {}", e),
    }
    
    // Reviews (with fallback)
    match user.get_reviews().await {
        Ok(reviews) => println!("Reviews written: {}", reviews.len()),
        Err(e) => println!("Reviews data unavailable: {}", e),
    }
    
    // Social data (with timeout)
    let social_timeout = Duration::from_secs(10);
    
    match tokio::time::timeout(social_timeout, user.get_followers()).await {
        Ok(Ok(followers)) => println!("Followers: {}", followers.len()),
        Ok(Err(e)) => println!("Followers data unavailable: {}", e),
        Err(_) => println!("Followers request timed out"),
    }
    
    Ok(())
}
```

These examples demonstrate the flexibility and power of Rustboxd for various use cases, from simple data retrieval to complex analysis workflows. The library's async nature makes it suitable for both single requests and batch operations while maintaining good performance and reliability.
