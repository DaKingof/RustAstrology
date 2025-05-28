// Main application setup and state management
use leptos::*;

use crate::models::harmonic::HarmonicType;
use crate::components::left_dial::LeftDial;
use crate::components::right_dial::RightDial;

/// Main App component that coordinates the entire application
#[component]
pub fn App() -> impl IntoView {
    // Shared state between dials
    let (current_harmonic, set_current_harmonic) = create_signal(HarmonicType::Fourth);
    let (shared_rotation, set_shared_rotation) = create_signal(0.0);
    
    view! {
        <div class="app-container" style="background-color: #1a1a1a; color: white; font-family: Arial, sans-serif; padding: 20px; min-height: 100vh;">
            <h1 style="text-align: center; margin-bottom: 30px;">
                "Rust Astrology Harmonic Explorer"
            </h1>
            
            <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 40px;">
                <div style="display: flex; flex-direction: column; align-items: center;">
                    <h3>"360° Standard Dial"</h3>
                    <LeftDial 
                        current_harmonic=current_harmonic
                        shared_rotation=shared_rotation
                        set_shared_rotation=set_shared_rotation
                    />
                </div>
                
                <div style="display: flex; flex-direction: column; align-items: center;">
                    <h3>{move || format!("{}th Harmonic Dial", current_harmonic.get().value())}</h3>
                    <RightDial 
                        current_harmonic=current_harmonic
                        shared_rotation=shared_rotation
                        set_shared_rotation=set_shared_rotation
                    />
                </div>
            </div>
            
            <div style="display: flex; justify-content: center; gap: 10px; margin-top: 30px; flex-wrap: wrap;">
                {HarmonicType::all_types().into_iter().map(|harmonic| {
                    let set_harmonic = set_current_harmonic.clone();
                    let harmonic_value = harmonic;
                    
                    view! {
                        <button 
                            on:click=move |_| set_harmonic.set(harmonic_value)
                            style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                        >
                            {harmonic_value.display_name()}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            <div style="margin-top: 30px; text-align: center; font-size: 0.9em; color: #888;">
                <p>"Drag either dial to rotate both in sync. Select harmonic values using the buttons above."</p>
                <p>"Built with Rust, WebAssembly, and Leptos."</p>
            </div>
        </div>
    }
}
