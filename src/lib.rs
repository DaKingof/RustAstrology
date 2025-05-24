use leptos::*;
use wasm_bindgen::prelude::*;

// This is the entry point for the web application
#[wasm_bindgen]
pub fn run_app() {
    // Only run this code in the browser
    #[cfg(target_arch = "wasm32")] {
        // Set up better panic and error handling in the browser
        console_error_panic_hook::set_once();
        
        // Initialize logging
        console_log::init_with_level(log::Level::Debug)
            .expect("error initializing logger");
        
        web_sys::console::log_1(&"Rust Astrology app starting...".into());
        
        // Mount the app to the body
        mount_to_body(|| view! { <App/> });
        
        web_sys::console::log_1(&"Rust Astrology app mounted".into());
    }
}

// Function to draw a circle on a canvas
fn draw_circle(canvas_id: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
    
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document.get_element_by_id(canvas_id).expect("canvas not found");
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    
    // Set canvas size to match its display size
    let style = window.get_computed_style(&canvas).unwrap().unwrap();
    let width = style.get_property_value("width").unwrap();
    let height = style.get_property_value("height").unwrap();
    canvas.set_width(width.trim_end_matches("px").parse().unwrap_or(400));
    canvas.set_height(height.trim_end_matches("px").parse().unwrap_or(400));
    
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    
    // Draw a big circle
    let center_x = (canvas.width() as f64) / 2.0;
    let center_y = (canvas.height() as f64) / 2.0;
    let radius = center_x.min(center_y) * 0.9; // 90% of half the smallest dimension
    
    // Draw the outer circle
    ctx.begin_path();
    ctx.arc(center_x, center_y, radius, 0.0, std::f64::consts::PI * 2.0).unwrap();
    ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str("#4a90e2"));
    ctx.set_line_width(4.0);
    ctx.stroke();
    
    // Add some text
    ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#333"));
    ctx.set_font("24px Arial");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    let _ = ctx.fill_text("Astrology Chart", center_x, center_y);
}

// Main application component
#[component]
pub fn App() -> impl IntoView {
    // Create a reactive signal for the zodiac sign
    let (zodiac_sign, _set_zodiac_sign) = create_signal("Aries".to_string());
    
    // Use an effect to draw on the canvas after the component mounts
    create_effect(move |_| {
        // This will run after the component is mounted to the DOM
        draw_circle("chart-canvas");
    });
    
    view! {
        <div class="app">
            <header>
                <h1>"Rust Astrology"</h1>
                <p>"Your modern astrological companion"</p>
            </header>
            
            <main>
                <div class="chart-container">
                    <canvas id="chart-canvas" width="600" height="600"></canvas>
                </div>
                
                <div class="input-section">
                    <label for="birth-date">"Birth Date:"</label>
                    <input 
                        type="date" 
                        id="birth-date"
                        name="birth-date"
                        placeholder="Select your birth date"
                    />
                    
                    <button on:click=on_submit>
                        "Generate Chart"
                    </button>
                </div>
            </main>
            
            <footer>
                <p>"© 2025 Rust Astrology - Built with Rust, WebAssembly, and Leptos"</p>
            </footer>
        </div>
    }
}
