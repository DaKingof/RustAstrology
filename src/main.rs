use leptos::prelude::*;

// Import the dial component
mod dial;
use dial::DialComponent;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <header style="text-align: center; padding: 2rem; background: #1e1e23; border-bottom: 2px solid #7f5af0;">
                <h1 style="background: linear-gradient(135deg, #7f5af0 0%, #2cb67d 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; margin: 0;">"🌟 Rust Astrology"</h1>
                <p style="color: #94a1b2; margin: 0.5rem 0 0 0;">"Interactive 90° Uranian Astrology Dial"</p>
            </header>
            
            <main style="padding: 2rem;">
                <DialComponent/>
            </main>
        </div>
    }
}

// Helper function to request animation frame
fn request_animation_frame(closure: impl FnOnce() + 'static) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let closure = Closure::once_into_js(closure);
    
    web_sys::window()
        .unwrap()
        .request_animation_frame(closure.unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    
    log::info!("🚀 Starting Debug Mode - checking canvas functionality...");
    
    leptos::mount::mount_to_body(|| view! { <App/> });
    
    log::info!("✅ Debug app mounted - watch for canvas debug messages...");
}
