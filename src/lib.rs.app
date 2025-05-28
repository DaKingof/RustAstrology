// Main app component
#[component]
pub fn App() -> impl IntoView {
    let (current_harmonic, set_current_harmonic) = create_signal(HarmonicType::Fourth);
    // Shared rotation signal for both dials
    let (shared_rotation, set_shared_rotation) = create_signal(0.0);

    view! {
        <div style="background-color: #1a1a1a; padding: 20px;">
            <h1 style="color: #fff; text-align: center; margin-bottom: 20px;">
                "Natal Astrology Chart Dials"
            </h1>
            <h2 style="color: #fff; text-align: center; margin-bottom: 30px;">
                "US Sibley Chart"
            </h2>
            
            <div style="display: flex; justify-content: center; gap: 40px; flex-wrap: wrap;">
                <LeftDial current_harmonic=current_harmonic shared_rotation=shared_rotation set_shared_rotation=set_shared_rotation/>
                <RightDial current_harmonic=current_harmonic shared_rotation=shared_rotation set_shared_rotation=set_shared_rotation/>
            </div>
            
            <div style="display: flex; justify-content: center; gap: 10px; margin-top: 30px; flex-wrap: wrap;">
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Second)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "2nd"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Third)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "3rd"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Fourth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "4th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Fifth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "5th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Sixth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "6th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Seventh)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "7th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Eighth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "8th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Ninth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "9th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Tenth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "10th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Eleventh)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "11th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Twelfth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "12th"
                </button>
                <button 
                    on:click=move |_| set_current_harmonic.set(HarmonicType::Sixteenth)
                    style="padding: 8px 16px; background: #4a4a4a; color: white; border: 1px solid #666; cursor: pointer; margin: 5px;"
                >
                    "16th"
                </button>
            </div>
        </div>
    }
}

// Entry point for WASM
#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}
