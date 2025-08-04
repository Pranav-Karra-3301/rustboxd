use regex::Regex;
use crate::core::constants::{VALID_RATINGS, GENRES, SEARCH_FILTERS};

/// Validate username format (alphanumeric and underscore only)
pub fn is_valid_username(username: &str) -> bool {
    if username.is_empty() || username.len() > 50 {
        return false;
    }
    
    let username_regex = Regex::new(r"^[A-Za-z0-9_]*$").unwrap();
    username_regex.is_match(username)
}

/// Validate film slug format
pub fn is_valid_film_slug(slug: &str) -> bool {
    if slug.is_empty() || slug.len() > 200 {
        return false;
    }
    
    // Film slugs typically contain lowercase letters, numbers, and hyphens
    let slug_regex = Regex::new(r"^[a-z0-9\-]*$").unwrap();
    slug_regex.is_match(slug)
}

/// Validate rating value (must be 0.5-5.0 in 0.5 increments)
pub fn is_valid_rating(rating: f32) -> bool {
    VALID_RATINGS.contains(&rating)
}

/// Validate genre name
pub fn is_valid_genre(genre: &str) -> bool {
    GENRES.contains(&genre.to_lowercase().as_str())
}

/// Validate search filter
pub fn is_valid_search_filter(filter: &str) -> bool {
    SEARCH_FILTERS.contains(&filter)
}

/// Validate year (reasonable range for films)
pub fn is_valid_year(year: i32) -> bool {
    year >= 1888 && year <= 2030 // Cinema started around 1888
}

/// Validate month (1-12)
pub fn is_valid_month(month: u32) -> bool {
    month >= 1 && month <= 12
}

/// Validate day (1-31, basic validation)
pub fn is_valid_day(day: u32) -> bool {
    day >= 1 && day <= 31
}

/// Validate Letterboxd URL format
pub fn is_valid_letterboxd_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^https?://(www\.)?letterboxd\.com/.+$").unwrap();
    url_regex.is_match(url)
}

/// Validate email format (basic validation)
pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// Validate URL format (generic)
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

/// Sanitize string for use in URLs
pub fn sanitize_for_url(input: &str) -> String {
    let normalized = input
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>();

    // Remove consecutive dashes and trim
    let mut result = String::new();
    let mut last_was_dash = false;
    
    for ch in normalized.chars() {
        if ch == '-' {
            if !last_was_dash {
                result.push(ch);
                last_was_dash = true;
            }
        } else {
            result.push(ch);
            last_was_dash = false;
        }
    }
    
    result.trim_matches('-').to_string()
}

/// Clean and validate text input
pub fn clean_and_validate_text(text: &str, max_length: usize) -> Option<String> {
    let cleaned = text.trim();
    
    if cleaned.is_empty() || cleaned.len() > max_length {
        return None;
    }
    
    // Remove excessive whitespace
    let normalized = cleaned
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    
    Some(normalized)
}

/// Validate list slug format
pub fn is_valid_list_slug(slug: &str) -> bool {
    if slug.is_empty() || slug.len() > 100 {
        return false;
    }
    
    let slug_regex = Regex::new(r"^[a-z0-9\-]*$").unwrap();
    slug_regex.is_match(slug)
}

/// Validate and normalize rating input
pub fn normalize_rating(rating: f32) -> Option<f32> {
    if rating < 0.0 || rating > 5.0 {
        return None;
    }
    
    // Round to nearest 0.5
    let rounded = (rating * 2.0).round() / 2.0;
    
    if is_valid_rating(rounded) {
        Some(rounded)
    } else {
        None
    }
}

/// Check if string contains only safe characters (no HTML/script)
pub fn is_safe_text(text: &str) -> bool {
    let dangerous_patterns = [
        "<script", "</script>", "javascript:", "onload=", "onerror=",
        "<iframe", "</iframe>", "<embed", "</embed>", "<object", "</object>"
    ];
    
    let lower_text = text.to_lowercase();
    !dangerous_patterns.iter().any(|&pattern| lower_text.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_validation() {
        assert!(is_valid_username("testuser"));
        assert!(is_valid_username("test_user"));
        assert!(is_valid_username("test123"));
        assert!(!is_valid_username("test-user")); // hyphen not allowed
        assert!(!is_valid_username("test user")); // space not allowed
        assert!(!is_valid_username(""));
    }

    #[test]
    fn test_rating_validation() {
        assert!(is_valid_rating(4.5));
        assert!(is_valid_rating(5.0));
        assert!(is_valid_rating(0.5));
        assert!(!is_valid_rating(4.3)); // not in 0.5 increments
        assert!(!is_valid_rating(5.5)); // too high
    }

    #[test]
    fn test_year_validation() {
        assert!(is_valid_year(2023));
        assert!(is_valid_year(1925));
        assert!(!is_valid_year(1800)); // too early
        assert!(!is_valid_year(2050)); // too late
    }

    #[test]
    fn test_sanitize_for_url() {
        assert_eq!(sanitize_for_url("The Matrix"), "the-matrix");
        assert_eq!(sanitize_for_url("Spider-Man: No Way Home"), "spider-man-no-way-home");
        assert_eq!(sanitize_for_url("2001: A Space Odyssey"), "2001-a-space-odyssey");
    }
}
