use rustboxd::utils::{is_valid_username, is_valid_rating, sanitize_for_url, extract_and_convert_shorthand};

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
fn test_url_sanitization() {
    assert_eq!(sanitize_for_url("The Matrix"), "the-matrix");
    assert_eq!(sanitize_for_url("Spider-Man: No Way Home"), "spider-man-no-way-home");
    assert_eq!(sanitize_for_url("2001: A Space Odyssey"), "2001-a-space-odyssey");
}

#[test]
fn test_shorthand_conversion() {
    assert_eq!(extract_and_convert_shorthand("1.2K"), 1200);
    assert_eq!(extract_and_convert_shorthand("500"), 500);
    assert_eq!(extract_and_convert_shorthand("2.5M"), 2500000);
    assert_eq!(extract_and_convert_shorthand("invalid"), 0);
}
