use rustboxd::{User, Search};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Advanced Rustboxd Usage Examples");
    println!("================================");

    // Advanced Search with pagination
    println!("\n1. Advanced search with multiple pages:");
    let mut search = Search::new("christopher nolan", Some("films")).await?;
    println!("Initial results: {} films", search.results.films.len());
    
    // Get more results from additional pages
    search.get_more_results(3).await?;
    println!("After pagination: {} films", search.results.films.len());

    // Display top results
    for (i, film) in search.results.films.iter().take(5).enumerate() {
        println!("  {}. {} ({})", i + 1, film.title, 
            film.year.map_or("Unknown".to_string(), |y| y.to_string()));
    }

    // User analysis
    println!("\n2. User activity analysis:");
    let user = User::new("nmcassa").await?;
    
    // Get user's liked films
    match user.get_liked_films().await {
        Ok(liked) => println!("User has liked {} films", liked.len()),
        Err(e) => println!("Error getting liked films: {}", e),
    }

    // Get user's reviews
    match user.get_reviews().await {
        Ok(reviews) => println!("User has written {} reviews", reviews.len()),
        Err(e) => println!("Error getting reviews: {}", e),
    }

    // Get user's watchlist
    match user.get_watchlist().await {
        Ok(watchlist) => println!("User has {} films in watchlist", watchlist.len()),
        Err(e) => println!("Error getting watchlist: {}", e),
    }

    // Get user's followers and following
    match user.get_followers().await {
        Ok(followers) => println!("User has {} followers", followers.len()),
        Err(e) => println!("Error getting followers: {}", e),
    }

    match user.get_following().await {
        Ok(following) => println!("User is following {} people", following.len()),
        Err(e) => println!("Error getting following: {}", e),
    }

    // Genre analysis
    println!("\n3. User genre preferences:");
    match user.get_genre_info().await {
        Ok(genre_info) => {
            println!("Retrieved genre statistics: {} entries", genre_info.len());
        }
        Err(e) => println!("Error getting genre info: {}", e),
    }

    // Different search types
    println!("\n4. Different search types:");
    
    // Search for reviews
    let review_search = Search::new("the godfather", Some("reviews")).await?;
    println!("Found {} reviews for 'the godfather'", review_search.results.reviews.len());

    // Search for lists
    let list_search = Search::new("best films", Some("lists")).await?;
    println!("Found {} lists for 'best films'", list_search.results.lists.len());

    // Search for members
    let member_search = Search::new("filmmaker", Some("members")).await?;
    println!("Found {} members for 'filmmaker'", member_search.results.members.len());

    println!("\nAdvanced examples completed!");

    Ok(())
}
