//! Rust Astrology - A web-based astrological chart visualization tool
//! 
//! This crate provides interactive dials for exploring astrological harmonics and midpoints.

#![warn(missing_docs)]

use wasm_bindgen::prelude::*;

// Re-export the main app component
pub use app::App;

// Public modules
pub mod models;
pub mod components;
pub mod utils;
pub mod app;

/// WebAssembly entry point
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize logging for better debugging
    console_error_panic_hook::set_once();
    
    // Mount the app to the body
    leptos::mount_to_body(App);
}
