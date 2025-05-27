use leptos::{prelude::*, html};
use super::{DialRenderer, DialRenderer360, AstrologyCalculator, DialState};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent, WheelEvent};

#[component]
pub fn DialComponent() -> impl IntoView {
    // Create reactive signals for shared dial state
    let (dial_state, set_dial_state) = signal(DialState::new());
    let canvas_ref_90: NodeRef<html::Canvas> = NodeRef::new();
    let canvas_ref_360: NodeRef<html::Canvas> = NodeRef::new();
    
    // Initialize with US Sibley chart data
    let calc = AstrologyCalculator::new();
    let positions = calc.get_us_sibley_chart();
    let midpoints = calc.calculate_midpoints(&positions);
    let alignments = calc.check_alignments(&positions, &midpoints, 1.0);
    
    set_dial_state.set(DialState {
        rotation: 0.0,
        planets: positions,
        midpoints,
        alignments,
        orb_tolerance: 1.0,
        is_dragging: false,
        last_mouse_pos: None,
    });
    
    // Render effect for 90° dial
    Effect::new(move |_| {
        let state = dial_state.get();
        log::info!("🎨 90° Effect triggered! Rotation: {:.1}°", state.rotation);
        
        if let Some(canvas_element) = canvas_ref_90.get_untracked() {
            let canvas: HtmlCanvasElement = canvas_element.unchecked_into();
            
            if let Ok(Some(context_obj)) = canvas.get_context("2d") {
                if let Ok(context) = context_obj.dyn_into::<CanvasRenderingContext2d>() {
                    canvas.set_width(600);
                    canvas.set_height(600);
                    
                    let renderer = DialRenderer::new();
                    if let Err(e) = renderer.render(&canvas, &context, &state) {
                        log::error!("90° Render error: {:?}", e);
                    } else {
                        log::debug!("✅ Rendered 90° dial at rotation: {:.1}°", state.rotation);
                    }
                } else {
                    log::error!("Failed to get 2D context for 90° dial");
                }
            } else {
                log::error!("Failed to get canvas context for 90° dial");
            }
        } else {
            log::warn!("90° Canvas element not found");
        }
    });
    
    // Render effect for 360° dial
    Effect::new(move |_| {
        let state = dial_state.get();
        log::info!("🎨 360° Effect triggered! Rotation: {:.1}°", state.rotation);
        
        if let Some(canvas_element) = canvas_ref_360.get_untracked() {
            let canvas: HtmlCanvasElement = canvas_element.unchecked_into();
            
            if let Ok(Some(context_obj)) = canvas.get_context("2d") {
                if let Ok(context) = context_obj.dyn_into::<CanvasRenderingContext2d>() {
                    canvas.set_width(600);
                    canvas.set_height(600);
                    
                    let renderer = DialRenderer360::new();
                    if let Err(e) = renderer.render(&canvas, &context, &state) {
                        log::error!("360° Render error: {:?}", e);
                    } else {
                        log::debug!("✅ Rendered 360° dial at rotation: {:.1}°", state.rotation);
                    }
                } else {
                    log::error!("Failed to get 2D context for 360° dial");
                }
            } else {
                log::error!("Failed to get canvas context for 360° dial");
            }
        } else {
            log::warn!("360° Canvas element not found");
        }
    });
    
    // Helper function to get canvas-relative coordinates
    let get_canvas_coords = |event: &MouseEvent, canvas_element: &HtmlCanvasElement| -> Option<(f64, f64)> {
        let element: &web_sys::Element = canvas_element.as_ref();
        let rect = element.get_bounding_client_rect();
        let canvas_x = event.client_x() as f64 - rect.left();
        let canvas_y = event.client_y() as f64 - rect.top();
        Some((canvas_x, canvas_y))
    };

    // Helper function to calculate angle from center to point
    let calculate_angle_from_center = |x: f64, y: f64, center_x: f64, center_y: f64| -> f64 {
        let dx = x - center_x;
        let dy = y - center_y;
        let angle_rad = dy.atan2(dx);
        let angle_deg = angle_rad * 180.0 / std::f64::consts::PI;
        // Normalize to 0-360 range
        (angle_deg + 360.0) % 360.0
    };

    // Generic mouse event handlers that work for both dials
    let create_mouse_down_handler = move |canvas_ref: NodeRef<html::Canvas>| {
        let set_dial_state = set_dial_state;
        let get_canvas_coords = get_canvas_coords;
        let calculate_angle_from_center = calculate_angle_from_center;
        
        move |event: MouseEvent| {
            log::info!("Mouse down event triggered");
            
            if let Some(canvas_element) = canvas_ref.get_untracked() {
                let canvas: HtmlCanvasElement = canvas_element.unchecked_into();
                
                if let Some((canvas_x, canvas_y)) = get_canvas_coords(&event, &canvas) {
                    let center_x = canvas.width() as f64 / 2.0;
                    let center_y = canvas.height() as f64 / 2.0;
                    let angle = calculate_angle_from_center(canvas_x, canvas_y, center_x, center_y);
                    
                    log::info!("Mouse down at canvas coordinates: ({:.1}, {:.1}), angle: {:.1}°", 
                              canvas_x, canvas_y, angle);
                    
                    set_dial_state.update(|state| {
                        state.is_dragging = true;
                        state.last_mouse_pos = Some(nalgebra::Vector2::new(canvas_x, canvas_y));
                        log::info!("Set dragging to true");
                    });
                }
            }
        }
    };
    
    let create_mouse_move_handler = move |canvas_ref: NodeRef<html::Canvas>, is_360_dial: bool| {
        let set_dial_state = set_dial_state;
        let get_canvas_coords = get_canvas_coords;
        let calculate_angle_from_center = calculate_angle_from_center;
        
        move |event: MouseEvent| {
            if let Some(canvas_element) = canvas_ref.get_untracked() {
                let canvas: HtmlCanvasElement = canvas_element.unchecked_into();
                
                if let Some((canvas_x, canvas_y)) = get_canvas_coords(&event, &canvas) {
                    set_dial_state.update(|state| {
                        if !state.is_dragging {
                            return;
                        }

                        if let Some(last_pos) = state.last_mouse_pos {
                            let center_x = canvas.width() as f64 / 2.0;
                            let center_y = canvas.height() as f64 / 2.0;
                            
                            // Calculate angles from center to both positions
                            let last_angle = calculate_angle_from_center(last_pos.x, last_pos.y, center_x, center_y);
                            let current_angle = calculate_angle_from_center(canvas_x, canvas_y, center_x, center_y);
                            
                            // Calculate angular difference (handle wraparound)
                            let mut angle_delta = current_angle - last_angle;
                            if angle_delta > 180.0 {
                                angle_delta -= 360.0;
                            } else if angle_delta < -180.0 {
                                angle_delta += 360.0;
                            }
                            
                            let sensitivity = if event.shift_key() && event.ctrl_key() {
                                0.01
                            } else if event.shift_key() || event.ctrl_key() {
                                0.1
                            } else {
                                1.0
                            };
                            
                            // For 90° dial, reduce the angle delta by 4x since it's compressed
                            let rotation_delta = if is_360_dial {
                                angle_delta * sensitivity
                            } else {
                                angle_delta * sensitivity * 4.0  // 90° dial needs 4x multiplier
                            };
                            
                            state.rotation = (state.rotation + rotation_delta + 360.0) % 360.0;
                            state.last_mouse_pos = Some(nalgebra::Vector2::new(canvas_x, canvas_y));
                            
                            log::info!("Angular delta: {:.1}°, Rotation: {:.1}°, Dial: {}", 
                                     angle_delta, state.rotation, if is_360_dial { "360°" } else { "90°" });
                            
                            // Recalculate alignments
                            let calc = AstrologyCalculator::new();
                            state.alignments = calc.check_alignments(&state.planets, &state.midpoints, state.orb_tolerance);
                        }
                    });
                }
            }
        }
    };
    
    let create_mouse_up_handler = move || {
        let set_dial_state = set_dial_state;
        move |_event: MouseEvent| {
            log::info!("Mouse up event triggered");
            set_dial_state.update(|state| {
                state.is_dragging = false;
                state.last_mouse_pos = None;
                log::info!("Set dragging to false");
            });
        }
    };
    
    let create_wheel_handler = move |is_360_dial: bool| {
        let set_dial_state = set_dial_state;
        move |event: WheelEvent| {
            event.prevent_default();
            log::info!("Wheel event: delta_y = {}", event.delta_y());
            
            set_dial_state.update(|state| {
                let sensitivity = if event.shift_key() && event.ctrl_key() {
                    0.01
                } else if event.shift_key() || event.ctrl_key() {
                    0.1
                } else {
                    1.0
                };
                
                // For 90° dial, apply 4x multiplier to make wheel movement feel natural
                let base_rotation = -event.delta_y() * sensitivity * 0.1;
                let rotation_delta = if is_360_dial {
                    base_rotation
                } else {
                    base_rotation * 4.0  // 90° dial needs 4x multiplier
                };
                
                state.rotation = (state.rotation + rotation_delta + 360.0) % 360.0;
                
                log::info!("Wheel rotation: {:.1}°, Dial: {}", state.rotation, if is_360_dial { "360°" } else { "90°" });
                
                // Recalculate alignments
                let calc = AstrologyCalculator::new();
                state.alignments = calc.check_alignments(&state.planets, &state.midpoints, state.orb_tolerance);
            });
        }
    };

    // Create specific handlers for each dial
    let handle_mouse_down_90 = create_mouse_down_handler(canvas_ref_90);
    let handle_mouse_move_90 = create_mouse_move_handler(canvas_ref_90, false);
    let handle_mouse_up_90 = create_mouse_up_handler();
    let handle_wheel_90 = create_wheel_handler(false);

    let handle_mouse_down_360 = create_mouse_down_handler(canvas_ref_360);
    let handle_mouse_move_360 = create_mouse_move_handler(canvas_ref_360, true);
    let handle_mouse_up_360 = create_mouse_up_handler();
    let handle_wheel_360 = create_wheel_handler(true);

    view! {
        <div class="dial-container" style="text-align: center; padding: 2rem;">
            <h2>"Synchronized Astrology Dials - USA Sibley Chart"</h2>
            <p>"July 4, 1776 • 5:10 PM LMT • Philadelphia, PA"</p>
            
            <div class="dials-wrapper" style="display: flex; gap: 3rem; justify-content: center; align-items: flex-start; margin: 2rem 0;">
                // 90° Uranian Dial
                <div class="dial-section" style="flex-shrink: 0; text-align: center;">
                    <h3 style="color: #7f5af0; margin-bottom: 1rem;">"90° Uranian Dial"</h3>
                    <canvas
                        node_ref=canvas_ref_90
                        class="astrology-dial-90"
                        width="600"
                        height="600"
                        style="border: 2px solid #7f5af0; border-radius: 50%; cursor: grab; background: #1a1a1a;"
                        on:mousedown=handle_mouse_down_90
                        on:mousemove=handle_mouse_move_90
                        on:mouseup=handle_mouse_up_90
                        on:wheel=handle_wheel_90
                        tabindex="0"
                    />
                    <div style="margin-top: 0.5rem; color: #94a1b2; font-size: 0.9rem;">
                        "Current position: " {move || format!("{:.1}°", (dial_state.get().rotation / 4.0) % 90.0)}
                    </div>
                </div>
                
                // 360° Traditional Dial
                <div class="dial-section" style="flex-shrink: 0; text-align: center;">
                    <h3 style="color: #2cb67d; margin-bottom: 1rem;">"360° Traditional Dial"</h3>
                    <canvas
                        node_ref=canvas_ref_360
                        class="astrology-dial-360"
                        width="600"
                        height="600"
                        style="border: 2px solid #2cb67d; border-radius: 50%; cursor: grab; background: #1a1a1a;"
                        on:mousedown=handle_mouse_down_360
                        on:mousemove=handle_mouse_move_360
                        on:mouseup=handle_mouse_up_360
                        on:wheel=handle_wheel_360
                        tabindex="0"
                    />
                    <div style="margin-top: 0.5rem; color: #94a1b2; font-size: 0.9rem;">
                        "Current rotation: " {move || format!("{:.1}°", dial_state.get().rotation)}
                    </div>
                </div>
            </div>
            
            // Alignment display
            <div class="alignment-display" style="
                max-width: 800px;
                margin: 2rem auto;
                background: rgba(127, 90, 240, 0.1); 
                border: 2px solid #7f5af0; 
                border-radius: 8px; 
                padding: 1.5rem;
                text-align: left;
                color: #fffffe;
                font-family: monospace;
                font-size: 0.9rem;
            ">
                <h3 style="margin-top: 0; color: #7f5af0; text-align: center;">"Live Synchronization Status"</h3>
                
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                    <div>
                        <strong style="color: #7f5af0;">"90° Dial (Uranian)"</strong>
                        <div style="margin-left: 1rem; color: #94a1b2;">
                            "• Position: " {move || format!("{:.1}°", (dial_state.get().rotation / 4.0) % 90.0)}
                            <br/>
                            "• 0° Axis: " {move || format!("{:.1}°", dial_state.get().rotation % 90.0)}
                            <br/>
                            "• 22.5° Axis: " {move || format!("{:.1}°", (dial_state.get().rotation + 90.0) % 90.0)}
                            <br/>
                            "• 45° Axis: " {move || format!("{:.1}°", (dial_state.get().rotation + 180.0) % 90.0)}
                            <br/>
                            "• 67.5° Axis: " {move || format!("{:.1}°", (dial_state.get().rotation + 270.0) % 90.0)}
                        </div>
                    </div>
                    
                    <div>
                        <strong style="color: #2cb67d;">"360° Dial (Traditional)"</strong>
                        <div style="margin-left: 1rem; color: #94a1b2;">
                            "• Rotation: " {move || format!("{:.1}°", dial_state.get().rotation)}
                            <br/>
                            "• Aries Point (0°): " {move || format!("{:.1}°", dial_state.get().rotation)}
                            <br/>
                            "• Cancer Point (90°): " {move || format!("{:.1}°", (dial_state.get().rotation + 90.0) % 360.0)}
                            <br/>
                            "• Libra Point (180°): " {move || format!("{:.1}°", (dial_state.get().rotation + 180.0) % 360.0)}
                            <br/>
                            "• Capricorn Point (270°): " {move || format!("{:.1}°", (dial_state.get().rotation + 270.0) % 360.0)}
                        </div>
                    </div>
                </div>
                
                <div style="margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid #7f5af0; text-align: center;">
                    <strong style="color: #ffd93d;">"Active Alignments: "</strong>
                    <span style="color: #94a1b2;">
                        {move || {
                            let state = dial_state.get();
                            if state.alignments.is_empty() {
                                "No alignments within 1° orb".to_string()
                            } else {
                                format!("{} alignment(s) found", state.alignments.len())
                            }
                        }}
                    </span>
                </div>
            </div>
            
            <div class="dial-info" style="max-width: 900px; margin: 2rem auto 0; color: #94a1b2;">
                <p>"🔄 Both dials are synchronized - drag either one to rotate both simultaneously"</p>
                <p>"🖱️ Use mouse wheel on either dial for smooth rotation"</p>
                <p>"🎯 Hold Shift for fine control, Ctrl for finer, Shift+Ctrl for extra fine"</p>
                <p>"📐 90° dial shows compressed view (0-90°), 360° dial shows full circle"</p>
                <p>"🟡 Yellow lines show midpoints aligned with axis points within 1° orb"</p>
            </div>
        </div>
    }
}
