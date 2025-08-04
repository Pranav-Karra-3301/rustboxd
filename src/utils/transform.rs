use crate::core::constants::DOMAIN;

/// Convert regular URL to AJAX URL for pagination
pub fn get_ajax_url(url: &str) -> String {
    if url.contains("/films/") {
        url.replace("/films/", "/ajax/films/")
    } else if url.contains("/film/") {
        url.replace("/film/", "/ajax/film/")
    } else if url.contains("/lists/") {
        url.replace("/lists/", "/ajax/lists/")
    } else if url.contains("/reviews/") {
        url.replace("/reviews/", "/ajax/reviews/")
    } else {
        format!("{}/ajax", url.trim_end_matches('/'))
    }
}

/// Build Letterboxd URL from components
pub fn build_letterboxd_url(path: &str) -> String {
    let path = path.trim_start_matches('/');
    format!("{}/{}", DOMAIN, path)
}

/// Build user profile URL
pub fn build_user_url(username: &str) -> String {
    format!("{}/{}/", DOMAIN, username)
}

/// Build film URL from slug
pub fn build_film_url(slug: &str) -> String {
    format!("{}/film/{}/", DOMAIN, slug)
}

/// Build list URL from author and slug
pub fn build_list_url(author: &str, slug: &str) -> String {
    format!("{}/{}/list/{}/", DOMAIN, author, slug)
}

/// Build search URL
pub fn build_search_url(query: &str, filter: Option<&str>) -> String {
    let encoded_query = urlencoding::encode(query);
    
    if let Some(filter) = filter {
        format!("{}/s/search/{}/{}/", DOMAIN, filter, encoded_query)
    } else {
        format!("{}/s/search/{}/", DOMAIN, encoded_query)
    }
}

/// Build diary URL with optional date components
pub fn build_diary_url(username: &str, year: Option<i32>, month: Option<u32>, day: Option<u32>) -> String {
    let mut url = format!("{}/{}/films/diary/", DOMAIN, username);
    
    if let Some(year) = year {
        url.push_str(&format!("for/{}/", year));
        
        if let Some(month) = month {
            url.push_str(&format!("{:02}/", month));
            
            if let Some(day) = day {
                url.push_str(&format!("{:02}/", day));
            }
        }
    }
    
    url
}

/// Build films URL with optional filters
pub fn build_films_url(username: &str, filter: Option<&str>) -> String {
    let mut url = format!("{}/{}/films/", DOMAIN, username);
    
    if let Some(filter) = filter {
        url.push_str(&format!("{}/", filter));
    }
    
    url
}

/// Add page parameter to URL
pub fn add_page_to_url(base_url: &str, page: u32) -> String {
    let base = base_url.trim_end_matches('/');
    format!("{}/page/{}/", base, page)
}

/// Normalize Letterboxd URL (ensure it has proper format)
pub fn normalize_letterboxd_url(url: &str) -> String {
    let url = url.trim();
    
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }
    
    if url.starts_with("/") {
        return format!("{}{}", DOMAIN, url);
    }
    
    format!("{}/{}", DOMAIN, url)
}

/// Extract pagination info from URL
pub fn extract_page_from_url(url: &str) -> Option<u32> {
    if url.contains("/page/") {
        let parts: Vec<&str> = url.split("/page/").collect();
        if parts.len() > 1 {
            let page_part = parts[1].split('/').next().unwrap_or("");
            return page_part.parse().ok();
        }
    }
    None
}

/// Remove page info from URL
pub fn remove_page_from_url(url: &str) -> String {
    if url.contains("/page/") {
        let parts: Vec<&str> = url.split("/page/").collect();
        if !parts.is_empty() {
            return parts[0].to_string();
        }
    }
    url.to_string()
}

/// Build URL for specific film sections
pub fn build_film_section_url(slug: &str, section: &str) -> String {
    format!("{}/film/{}/{}/", DOMAIN, slug, section)
}

/// Build URL for user sections
pub fn build_user_section_url(username: &str, section: &str) -> String {
    format!("{}/{}/{}/", DOMAIN, username, section)
}
