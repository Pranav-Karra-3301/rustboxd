use crate::core::{Error, Result};
use crate::core::constants::MONTH_ABBREVIATIONS;

/// Attempt to convert a value to the specified target type
pub fn try_parse<T: std::str::FromStr>(value: &str) -> Option<T> {
    value.parse().ok()
}

/// Extract and convert shorthand notation (e.g., '6.3K') to an integer
pub fn extract_and_convert_shorthand(text: &str) -> u32 {
    let cleaned = text.trim().replace(',', "");
    
    if cleaned.contains('K') {
        if let Ok(num) = cleaned.replace('K', "").parse::<f32>() {
            return (num * 1000.0) as u32;
        }
    } else if cleaned.contains('M') {
        if let Ok(num) = cleaned.replace('M', "").parse::<f32>() {
            return (num * 1_000_000.0) as u32;
        }
    } else if let Ok(num) = cleaned.parse::<u32>() {
        return num;
    }
    
    0
}

/// Extract numeric characters from a string and return them as an integer
pub fn extract_numeric_text(text: &str) -> Option<u32> {
    let numeric_only: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
    numeric_only.parse().ok()
}

/// Parse an ISO 8601 formatted date string
pub fn parse_iso_date(iso_date_str: &str) -> Result<(i32, u32, u32)> {
    let parts: Vec<&str> = iso_date_str.split('T').next().unwrap_or("").split('-').collect();
    
    if parts.len() != 3 {
        return Err(Error::Parse(format!("Invalid ISO date format: {}", iso_date_str)));
    }
    
    let year = parts[0].parse::<i32>()
        .map_err(|_| Error::Parse(format!("Invalid year in date: {}", iso_date_str)))?;
    let month = parts[1].parse::<u32>()
        .map_err(|_| Error::Parse(format!("Invalid month in date: {}", iso_date_str)))?;
    let day = parts[2].parse::<u32>()
        .map_err(|_| Error::Parse(format!("Invalid day in date: {}", iso_date_str)))?;
    
    Ok((year, month, day))
}

/// Parse a written date string (e.g., '01 Jan 2025')
pub fn parse_written_date(written_date_str: &str) -> Result<(i32, u32, u32)> {
    let parts: Vec<&str> = written_date_str.split_whitespace().collect();
    
    if parts.len() != 3 {
        return Err(Error::Parse(format!("Invalid written date format: {}", written_date_str)));
    }
    
    let day = parts[0].parse::<u32>()
        .map_err(|_| Error::Parse(format!("Invalid day in date: {}", written_date_str)))?;
    
    let month = month_to_index(parts[1])
        .ok_or_else(|| Error::Parse(format!("Invalid month in date: {}", written_date_str)))?;
    
    let year = parts[2].parse::<i32>()
        .map_err(|_| Error::Parse(format!("Invalid year in date: {}", written_date_str)))?;
    
    Ok((year, month, day))
}

/// Convert month abbreviation to month number (1-12)
pub fn month_to_index(month_abbr: &str) -> Option<u32> {
    MONTH_ABBREVIATIONS
        .iter()
        .position(|&abbr| abbr.eq_ignore_ascii_case(month_abbr))
        .map(|pos| pos as u32 + 1)
}

/// Extract content from meta tag by property or name attribute
pub fn get_meta_content(dom: &scraper::Html, property: Option<&str>, name: Option<&str>) -> Option<String> {
    use scraper::Selector;
    
    let selector_str = if let Some(prop) = property {
        format!("meta[property='{}']", prop)
    } else if let Some(n) = name {
        format!("meta[name='{}']", n)
    } else {
        return None;
    };
    
    let selector = Selector::parse(&selector_str).ok()?;
    dom.select(&selector)
        .next()?
        .value()
        .attr("content")
        .map(|s| s.to_string())
}

/// Extract attribute value from body tag
pub fn get_body_content(dom: &scraper::Html, attribute: &str) -> Option<String> {
    use scraper::Selector;
    
    let selector = Selector::parse("body").ok()?;
    dom.select(&selector)
        .next()?
        .value()
        .attr(attribute)
        .map(|s| s.to_string())
}

/// Parse rating from text (handles formats like "4.2/5", "4.2", "★★★★☆")
pub fn parse_rating(text: &str) -> Option<f32> {
    let cleaned = text.trim().replace("★", "").replace("☆", "");
    
    if cleaned.contains('/') {
        let parts: Vec<&str> = cleaned.split('/').collect();
        if parts.len() == 2 {
            if let (Ok(rating), Ok(max)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                return Some((rating / max) * 5.0); // Normalize to 5-star scale
            }
        }
    } else if let Ok(rating) = cleaned.parse::<f32>() {
        if rating <= 5.0 {
            return Some(rating);
        }
    }
    
    None
}

/// Parse runtime from text (handles formats like "142 mins", "2h 22m", "2:22")
pub fn parse_runtime(text: &str) -> Option<u32> {
    let cleaned = text.trim().to_lowercase();
    
    if cleaned.contains("min") {
        return extract_numeric_text(&cleaned);
    }
    
    if cleaned.contains('h') && cleaned.contains('m') {
        // Format: "2h 22m"
        let parts: Vec<&str> = cleaned.split('h').collect();
        if parts.len() == 2 {
            let hours = extract_numeric_text(parts[0]).unwrap_or(0);
            let mins = extract_numeric_text(parts[1]).unwrap_or(0);
            return Some(hours * 60 + mins);
        }
    }
    
    if cleaned.contains(':') {
        // Format: "2:22"
        let parts: Vec<&str> = cleaned.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(hours), Ok(mins)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                return Some(hours * 60 + mins);
            }
        }
    }
    
    None
}

/// Clean and normalize text content
pub fn clean_text(text: &str) -> String {
    text.trim()
        .replace('\n', " ")
        .replace('\t', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Extract film slug from Letterboxd URL
pub fn extract_film_slug(url: &str) -> Option<String> {
    if url.contains("/film/") {
        let parts: Vec<&str> = url.split("/film/").collect();
        if parts.len() > 1 {
            let slug = parts[1].split('/').next().unwrap_or("");
            if !slug.is_empty() {
                return Some(slug.to_string());
            }
        }
    }
    None
}

/// Extract user slug from Letterboxd URL
pub fn extract_user_slug(url: &str) -> Option<String> {
    let url = url.trim_end_matches('/');
    let parts: Vec<&str> = url.split('/').collect();
    
    // Handle URLs like "https://letterboxd.com/username"
    if parts.len() >= 4 && parts[2] == "letterboxd.com" {
        return Some(parts[3].to_string());
    }
    
    None
}
