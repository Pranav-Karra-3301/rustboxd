//! Rustboxd - A Letterboxd web scraper and API client library
//!
//! This library provides a Rust interface for scraping data from Letterboxd,
//! including user profiles, movie details, search functionality, and more.

pub mod core;
pub mod models;
pub mod pages;
pub mod utils;

// Re-export main types
pub use models::{User, Movie, Search, Films, List};
pub use core::{Error, Result, Client};

pub use chrono;
pub use serde_json;
