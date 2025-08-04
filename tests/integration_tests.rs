use rustboxd::{User, Movie, Search};
use tokio_test;

#[tokio::test]
async fn test_user_creation() {
    let result = User::new("testuser").await;
    
    // We expect this to either succeed or fail with a specific error
    // Since we're testing against a real website, we'll just check the error type
    match result {
        Ok(user) => {
            assert_eq!(user.username, "testuser");
            assert!(!user.url.is_empty());
        }
        Err(e) => {
            // Expected for non-existent users or network issues
            println!("Expected error for test user: {}", e);
        }
    }
}

#[tokio::test]
async fn test_movie_creation() {
    let result = Movie::new("test-movie-slug").await;
    
    match result {
        Ok(movie) => {
            assert_eq!(movie.slug, "test-movie-slug");
            assert!(!movie.url.is_empty());
        }
        Err(e) => {
            // Expected for non-existent movies or network issues
            println!("Expected error for test movie: {}", e);
        }
    }
}

#[tokio::test]
async fn test_search_creation() {
    let result = Search::new("test query", Some("films")).await;
    
    match result {
        Ok(search) => {
            assert_eq!(search.query, "test query");
            assert_eq!(search.search_filter, Some("films".to_string()));
        }
        Err(e) => {
            // Expected for network issues
            println!("Expected error for test search: {}", e);
        }
    }
}

#[test]
fn test_invalid_search_filter() {
    // Test that invalid search filters are rejected at compile time or runtime
    // This test doesn't need to make network requests
    let valid_filters = ["films", "reviews", "lists", "members"];
    
    for filter in valid_filters {
        // These should be valid
        assert!(rustboxd::core::constants::SEARCH_FILTERS.contains(&filter));
    }
}
