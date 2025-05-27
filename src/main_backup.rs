// Backup of the current main.rs
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// Use the dial module locally
use crate::dial::DialComponent;

// Import the dial module
mod dial;

// Main application component
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <header>
                <h1>"Rust Astrology - 90° Uranian Dial"</h1>
                <p>"Interactive real-time midpoint detection"</p>
            </header>
            
            <main>
                <DialComponent/>
            </main>
            
            <footer>
                <p>"Built with Rust, WebAssembly, and Leptos"</p>
                <p>"Featuring live midpoint detection for Uranian astrology"</p>
            </footer>
        </div>
    }
}

fn main() {
    // Initialize logger for WebAssembly
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("WebAssembly application starting...");
    
    // Mount the Leptos app to the body
    log::info!("Mounting Leptos application with 90° dial");
    leptos::mount::mount_to_body(|| view! { <App/> });
}
