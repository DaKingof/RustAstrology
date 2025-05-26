use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use wasm_bindgen_futures::spawn_local;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn run_app() {
    // Initialize logger for WebAssembly
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("WebAssembly application starting...");
    
    // Here we would initialize the Leptos app
    spawn_local(async {
        log::info!("Async WebAssembly application context initialized");
        // Initialize your Leptos app here
    });
}

#[cfg(not(feature = "wasm"))]
fn main() {
    // Desktop entry point (will not be compiled in WebAssembly builds)
    println!("Starting Rust Astrology desktop application");
    
    // This is only compiled when the "desktop" feature is enabled
    #[cfg(feature = "desktop")]
    {
        // Initialize desktop logging
        env_logger::init();
        log::info!("Desktop application starting...");
        
        // Normally we would initialize Tauri here, but that's handled
        // in the src-tauri/src/main.rs file
        println!("Tauri initialization is handled in the src-tauri crate");
    }
}

// WebAssembly builds need a main function that does nothing
// This is because the entry point is run_app() which is called from JavaScript
#[cfg(feature = "wasm")]
fn main() {
    // Do nothing - WebAssembly is initialized through run_app()
}