use rustboxd::{User, Movie, Search, Films, List};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Rustboxd Examples");
    println!("================");

    // Example 1: Get user information
    println!("\n1. Getting user information:");
    match User::new("nmcassa").await {
        Ok(user) => {
            println!("Username: {}", user.username);
            println!("Display Name: {}", user.display_name);
            println!("URL: {}", user.url);
            if let Some(bio) = &user.bio {
                println!("Bio: {}", bio);
            }
        }
        Err(e) => println!("Error fetching user: {}", e),
    }

    // Example 2: Get movie details
    println!("\n2. Getting movie details:");
    match Movie::new("the-matrix").await {
        Ok(movie) => {
            println!("Title: {}", movie.title);
            if let Some(year) = movie.year {
                println!("Year: {}", year);
            }
            if let Some(rating) = movie.rating {
                println!("Rating: {}/5", rating);
            }
            println!("Genres: {:?}", movie.genres);
        }
        Err(e) => println!("Error fetching movie: {}", e),
    }

    // Example 3: Search for films
    println!("\n3. Searching for films:");
    match Search::new("pulp fiction", Some("films")).await {
        Ok(search) => {
            println!("Search query: {}", search.query);
            println!("Found {} films", search.results.films.len());
            
            for (i, film) in search.results.films.iter().take(3).enumerate() {
                println!("  {}. {} ({})", i + 1, film.title, 
                    film.year.map_or("Unknown".to_string(), |y| y.to_string()));
            }
        }
        Err(e) => println!("Error searching: {}", e),
    }

    // Example 4: Get user's films
    println!("\n4. Getting user's films:");
    let user = User::new("nmcassa").await?;
    match user.get_films().await {
        Ok(films) => {
            println!("Retrieved {} film entries", films.len());
        }
        Err(e) => println!("Error fetching user films: {}", e),
    }

    // Example 5: Get user's diary
    println!("\n5. Getting user's diary for current year:");
    match user.get_diary(Some(2024), None, None, None).await {
        Ok(diary) => {
            println!("Retrieved {} diary entries", diary.len());
        }
        Err(e) => println!("Error fetching diary: {}", e),
    }

    println!("\nAll examples completed!");

    Ok(())
}
