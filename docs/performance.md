# Performance and Best Practices

This guide covers optimization strategies, rate limiting, error handling, and best practices when using Rustboxd.

## Performance Optimization

### Async Operations

Rustboxd is built on async/await patterns. Take advantage of concurrent operations when possible:

```rust
use rustboxd::{User, Movie};
use tokio::time::Instant;

async fn concurrent_operations() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Bad: Sequential operations
    let user1 = User::new("user1").await?;
    let user2 = User::new("user2").await?;
    let movie = Movie::new("the-matrix").await?;
    
    println!("Sequential: {:?}", start.elapsed());
    
    let start = Instant::now();
    
    // Good: Concurrent operations
    let (user1, user2, movie) = tokio::try_join!(
        User::new("user1"),
        User::new("user2"),
        Movie::new("the-matrix")
    )?;
    
    println!("Concurrent: {:?}", start.elapsed());
    
    Ok(())
}
```

### Batch Processing

When processing multiple items, use proper batching and rate limiting:

```rust
use rustboxd::User;
use tokio::time::{sleep, Duration};

async fn process_users_efficiently(usernames: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    const BATCH_SIZE: usize = 5;
    const DELAY_BETWEEN_BATCHES: Duration = Duration::from_secs(2);
    
    for batch in usernames.chunks(BATCH_SIZE) {
        println!("Processing batch of {} users", batch.len());
        
        // Process batch concurrently
        let results = futures::future::join_all(
            batch.iter().map(|&username| async move {
                match User::new(username).await {
                    Ok(user) => {
                        println!("✓ Loaded: {}", user.display_name);
                        Some(user)
                    }
                    Err(e) => {
                        println!("✗ Failed {}: {}", username, e);
                        None
                    }
                }
            })
        ).await;
        
        let successful: Vec<_> = results.into_iter().flatten().collect();
        println!("Batch completed: {}/{} successful", successful.len(), batch.len());
        
        // Rate limiting between batches
        if batch.len() == BATCH_SIZE {
            println!("Waiting before next batch...");
            sleep(DELAY_BETWEEN_BATCHES).await;
        }
    }
    
    Ok(())
}
```

### Memory Management

For large datasets, consider streaming or pagination:

```rust
use rustboxd::User;

async fn memory_efficient_analysis(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(username).await?;
    
    // Instead of loading all films at once:
    // let all_films = user.get_films().await?; // Potentially large
    
    // Process by rating categories (smaller chunks)
    let mut total_films = 0;
    let mut rating_sum = 0.0;
    
    for rating in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0] {
        let films = user.get_films_by_rating(rating).await?;
        total_films += films.len();
        rating_sum += rating * films.len() as f32;
        
        println!("Processed {} films with {:.1} rating", films.len(), rating);
        
        // Small delay for rate limiting
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    
    if total_films > 0 {
        let average_rating = rating_sum / total_films as f32;
        println!("Average rating: {:.2} (from {} films)", average_rating, total_films);
    }
    
    Ok(())
}
```

## Rate Limiting

### Respectful API Usage

Always implement proper rate limiting to be respectful to Letterboxd's servers:

```rust
use tokio::time::{sleep, Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RateLimiter {
    last_request: Arc<Mutex<Instant>>,
    min_interval: Duration,
}

impl RateLimiter {
    pub fn new(requests_per_second: f64) -> Self {
        let min_interval = Duration::from_secs_f64(1.0 / requests_per_second);
        Self {
            last_request: Arc::new(Mutex::new(Instant::now() - min_interval)),
            min_interval,
        }
    }
    
    pub async fn wait(&self) {
        let mut last = self.last_request.lock().await;
        let now = Instant::now();
        let elapsed = now.duration_since(*last);
        
        if elapsed < self.min_interval {
            let sleep_duration = self.min_interval - elapsed;
            drop(last); // Release lock before sleeping
            sleep(sleep_duration).await;
            *self.last_request.lock().await = Instant::now();
        } else {
            *last = now;
        }
    }
}

// Usage example
async fn rate_limited_operations(usernames: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let limiter = RateLimiter::new(2.0); // 2 requests per second
    
    for username in usernames {
        limiter.wait().await;
        
        match User::new(username).await {
            Ok(user) => println!("✓ {}: {}", username, user.display_name),
            Err(e) => println!("✗ {}: {}", username, e),
        }
    }
    
    Ok(())
}
```

### Adaptive Rate Limiting

Implement adaptive rate limiting that responds to server responses:

```rust
use rustboxd::{User, Error};
use tokio::time::{sleep, Duration};

async fn adaptive_rate_limited_fetch(username: &str) -> Result<User, Error> {
    let mut delay = Duration::from_millis(500);
    let max_delay = Duration::from_secs(30);
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 5;
    
    loop {
        attempts += 1;
        
        match User::new(username).await {
            Ok(user) => {
                // Success - can reduce delay for next request
                return Ok(user);
            }
            Err(Error::PageLoad { .. }) if attempts < MAX_ATTEMPTS => {
                println!("Request failed, waiting {:?} before retry {}", delay, attempts);
                sleep(delay).await;
                
                // Exponential backoff
                delay = (delay * 2).min(max_delay);
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Error Handling

### Comprehensive Error Handling

Handle different error types appropriately:

```rust
use rustboxd::{User, Error};

async fn robust_user_operation(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    match User::new(username).await {
        Ok(user) => {
            println!("Successfully loaded user: {}", user.display_name);
            
            // Try to get additional data with proper error handling
            match user.get_films().await {
                Ok(films) => println!("Films: {}", films.len()),
                Err(Error::PageLoad { url, message }) => {
                    println!("Warning: Could not load films page ({}): {}", url, message);
                    // Continue with other operations
                }
                Err(e) => {
                    println!("Error loading films: {}", e);
                    return Err(e.into());
                }
            }
        }
        Err(Error::InvalidUsername(username)) => {
            println!("Error: '{}' is not a valid username format", username);
            return Err("Invalid username".into());
        }
        Err(Error::UserNotFound(username)) => {
            println!("Error: User '{}' does not exist", username);
            return Err("User not found".into());
        }
        Err(Error::PageLoad { url, message }) => {
            println!("Error: Could not load user page ({}): {}", url, message);
            return Err("Page load failed".into());
        }
        Err(Error::ParseError { field, message }) => {
            println!("Error: Failed to parse {} - {}", field, message);
            return Err("Parse error".into());
        }
        Err(e) => {
            println!("Unexpected error: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}
```

### Retry Strategies

Implement smart retry logic:

```rust
use rustboxd::{User, Error};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct RetryConfig {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

async fn retry_with_backoff<F, Fut, T>(
    operation: F,
    config: RetryConfig,
) -> Result<T, Error>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, Error>>,
{
    let mut delay = config.initial_delay;
    let mut last_error = None;
    
    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(Error::InvalidUsername(_)) | Err(Error::UserNotFound(_)) => {
                // Don't retry these errors
                return Err(last_error.unwrap_or_else(|| Error::InvalidUsername("unknown".to_string())));
            }
            Err(e) => {
                last_error = Some(e);
                
                if attempt < config.max_attempts {
                    println!("Attempt {} failed, retrying in {:?}", attempt, delay);
                    sleep(delay).await;
                    delay = Duration::from_secs_f64(
                        (delay.as_secs_f64() * config.backoff_multiplier).min(config.max_delay.as_secs_f64())
                    );
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}

// Usage
async fn reliable_user_fetch(username: &str) -> Result<User, Error> {
    retry_with_backoff(
        || User::new(username),
        RetryConfig::default(),
    ).await
}
```

## Best Practices

### Resource Management

```rust
use rustboxd::User;
use std::sync::Arc;
use tokio::sync::Semaphore;

// Limit concurrent requests
async fn controlled_concurrent_operations(usernames: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_CONCURRENT: usize = 3;
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    
    let tasks: Vec<_> = usernames.into_iter().map(|username| {
        let sem = semaphore.clone();
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            
            match User::new(username).await {
                Ok(user) => {
                    println!("✓ {}: {}", username, user.display_name);
                    Ok(user)
                }
                Err(e) => {
                    println!("✗ {}: {}", username, e);
                    Err(e)
                }
            }
        })
    }).collect();
    
    // Wait for all tasks to complete
    let results = futures::future::join_all(tasks).await;
    
    let successful = results.into_iter()
        .filter_map(|r| r.ok().and_then(|r| r.ok()))
        .count();
    
    println!("Completed: {} successful operations", successful);
    
    Ok(())
}
```

### Caching Strategies

```rust
use rustboxd::User;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

#[derive(Clone)]
struct CachedUser {
    user: User,
    cached_at: Instant,
}

pub struct UserCache {
    cache: Arc<RwLock<HashMap<String, CachedUser>>>,
    ttl: Duration,
}

impl UserCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }
    
    pub async fn get_user(&self, username: &str) -> Result<User, Box<dyn std::error::Error>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(username) {
                if cached.cached_at.elapsed() < self.ttl {
                    println!("Cache hit for user: {}", username);
                    return Ok(cached.user.clone());
                }
            }
        }
        
        // Cache miss or expired - fetch fresh data
        println!("Cache miss for user: {}", username);
        let user = User::new(username).await?;
        
        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(username.to_string(), CachedUser {
                user: user.clone(),
                cached_at: Instant::now(),
            });
        }
        
        Ok(user)
    }
    
    pub async fn clear_expired(&self) {
        let mut cache = self.cache.write().await;
        cache.retain(|_, cached| cached.cached_at.elapsed() < self.ttl);
    }
}

// Usage
async fn cached_operations() -> Result<(), Box<dyn std::error::Error>> {
    let cache = UserCache::new(Duration::from_secs(300)); // 5 minute TTL
    
    // These will hit the network
    let user1 = cache.get_user("user1").await?;
    let user2 = cache.get_user("user2").await?;
    
    // These will hit the cache
    let user1_again = cache.get_user("user1").await?;
    let user2_again = cache.get_user("user2").await?;
    
    Ok(())
}
```

### Logging and Monitoring

```rust
use rustboxd::User;
use tracing::{info, warn, error, instrument};

// Configure logging in your main function
pub fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter("rustboxd_example=debug,rustboxd=info")
        .init();
}

#[instrument(skip(usernames))]
async fn monitored_batch_operation(usernames: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting batch operation for {} users", usernames.len());
    
    let mut successful = 0;
    let mut failed = 0;
    
    for username in usernames {
        match fetch_user_with_monitoring(username).await {
            Ok(_) => successful += 1,
            Err(_) => failed += 1,
        }
    }
    
    info!("Batch operation completed: {} successful, {} failed", successful, failed);
    
    Ok(())
}

#[instrument]
async fn fetch_user_with_monitoring(username: &str) -> Result<User, Box<dyn std::error::Error>> {
    info!("Fetching user: {}", username);
    
    let start = std::time::Instant::now();
    
    match User::new(username).await {
        Ok(user) => {
            let duration = start.elapsed();
            info!("Successfully loaded user {} in {:?}", user.display_name, duration);
            Ok(user)
        }
        Err(e) => {
            let duration = start.elapsed();
            error!("Failed to load user {} after {:?}: {}", username, duration, e);
            Err(e.into())
        }
    }
}
```

### Configuration Management

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct RustboxdConfig {
    pub rate_limit: RateLimitConfig,
    pub retry: RetryConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RateLimitConfig {
    pub requests_per_second: f64,
    pub burst_size: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheConfig {
    pub ttl_seconds: u64,
    pub max_entries: usize,
}

impl Default for RustboxdConfig {
    fn default() -> Self {
        Self {
            rate_limit: RateLimitConfig {
                requests_per_second: 2.0,
                burst_size: 5,
            },
            retry: RetryConfig {
                max_attempts: 3,
                initial_delay_ms: 500,
                max_delay_ms: 10000,
                backoff_multiplier: 2.0,
            },
            cache: CacheConfig {
                ttl_seconds: 300,
                max_entries: 1000,
            },
        }
    }
}

impl RustboxdConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

// Usage
async fn configured_application() -> Result<(), Box<dyn std::error::Error>> {
    let config = RustboxdConfig::load_from_file("rustboxd.toml")
        .unwrap_or_else(|_| {
            let config = RustboxdConfig::default();
            config.save_to_file("rustboxd.toml").ok();
            config
        });
    
    println!("Using configuration: {:#?}", config);
    
    // Use configuration for operations...
    
    Ok(())
}
```

## Performance Monitoring

### Basic Metrics

```rust
use std::time::Instant;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Default)]
pub struct Metrics {
    requests_total: AtomicU64,
    requests_successful: AtomicU64,
    requests_failed: AtomicU64,
    total_duration_ms: AtomicU64,
}

impl Metrics {
    pub fn record_request(&self, success: bool, duration: std::time::Duration) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.total_duration_ms.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        
        if success {
            self.requests_successful.fetch_add(1, Ordering::Relaxed);
        } else {
            self.requests_failed.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    pub fn print_summary(&self) {
        let total = self.requests_total.load(Ordering::Relaxed);
        let successful = self.requests_successful.load(Ordering::Relaxed);
        let failed = self.requests_failed.load(Ordering::Relaxed);
        let total_duration = self.total_duration_ms.load(Ordering::Relaxed);
        
        println!("=== Performance Summary ===");
        println!("Total requests: {}", total);
        println!("Successful: {} ({:.1}%)", successful, (successful as f64 / total as f64) * 100.0);
        println!("Failed: {} ({:.1}%)", failed, (failed as f64 / total as f64) * 100.0);
        
        if total > 0 {
            println!("Average duration: {:.1}ms", total_duration as f64 / total as f64);
        }
    }
}

async fn monitored_operations(metrics: Arc<Metrics>) -> Result<(), Box<dyn std::error::Error>> {
    let usernames = vec!["user1", "user2", "user3"];
    
    for username in usernames {
        let start = Instant::now();
        
        let success = match User::new(username).await {
            Ok(user) => {
                println!("✓ {}", user.display_name);
                true
            }
            Err(e) => {
                println!("✗ {}: {}", username, e);
                false
            }
        };
        
        metrics.record_request(success, start.elapsed());
    }
    
    metrics.print_summary();
    
    Ok(())
}
```

By following these performance guidelines and best practices, you'll build efficient, reliable applications with Rustboxd that respect Letterboxd's servers and provide excellent user experience.
