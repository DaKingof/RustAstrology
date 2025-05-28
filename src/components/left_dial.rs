use leptos::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use std::f64::consts::PI;

use crate::components::dial::Dial;
use crate::components::dial::renderer;
use crate::models::harmonic::HarmonicType;
use crate::models::planet::{self, Planet};
use crate::utils::math::degrees_to_coords;
use crate::utils::constants::*;

#[component]
pub fn LeftDial(
    current_harmonic: ReadSignal<HarmonicType>,
    shared_rotation: ReadSignal<f64>,
    set_shared_rotation: WriteSignal<f64>,
) -> impl IntoView {
    let planets = planet::get_sibley_chart();
    let midpoints = planet::get_planet_pairs_for_midpoints(&planets);
    let canvas_id = "left-dial-canvas";
    let dial = Rc::new(Dial::new(canvas_id));

    {
        let dial = dial.clone();
        create_effect(move |_| {
            let rotation = shared_rotation.get();
            
            if let Some(context) = dial.get_context() {
                // Clear canvas
                renderer::clear_canvas(&context);
                
                // Draw main circle
                renderer::draw_circle(&context);
                
                // Draw tick marks every 5 degrees
                renderer::draw_tick_marks(&context, rotation, 5.0, 72);
                
                // Draw axis lines (0°, 90°, 180°, 270°)
                context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&SECONDARY_STROKE_COLOR));
                context.set_line_width(1.5);
                context.set_font("12px Arial");
                context.set_fill_style(&wasm_bindgen::JsValue::from_str(&BASE_STROKE_COLOR));
                
                for i in 0..4 {
                    let angle = (i as f64 * 90.0 + rotation) % 360.0;
                    let (x1, y1) = degrees_to_coords(angle, 0.0);
                    let (x2, y2) = degrees_to_coords(angle, RADIUS);
                    
                    context.begin_path();
                    context.move_to(x1, y1);
                    context.line_to(x2, y2);
                    context.stroke();
                    
                    // Label the axis
                    let (lx, ly) = degrees_to_coords(angle, RADIUS + 45.0);
                    let label = match i {
                        0 => "0°",
                        1 => "90°",
                        2 => "180°",
                        3 => "270°",
                        _ => "",
                    };
                    let _ = context.fill_text(label, lx, ly);
                    
                    // Draw arrow at end of axis
                    let arrow_size = 5.0;
                    let arrow_angle = (angle + 180.0) % 360.0;
                    let (x1, y1) = degrees_to_coords(arrow_angle, RADIUS);
                    let (x2, y2) = degrees_to_coords(arrow_angle + 30.0, RADIUS - arrow_size);
                    let (x3, y3) = degrees_to_coords(arrow_angle - 30.0, RADIUS - arrow_size);
                    
                    context.begin_path();
                    context.move_to(x1, y1);
                    context.line_to(x2, y2);
                    context.line_to(x3, y3);
                    context.close_path();
                    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&BASE_STROKE_COLOR));
                    context.fill();
                }
                
                // Draw midpoint lines
                context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&MIDPOINT_STROKE_COLOR));
                context.set_line_width(1.0);
                
                // Calculate visual axis angles for midpoint activation
                let arm_angles_visual = (0..4)
                    .map(|i| (i as f64 * 90.0 + rotation) % 360.0)
                    .collect::<Vec<_>>();
                
                // Active midpoints (those close to the axis)
                let active_midpoints = midpoints.iter()
                    .filter(|(midpoint, _, _)| {
                        arm_angles_visual.iter().any(|&axis| {
                            let diff = (midpoint - axis).abs() % 360.0;
                            let normalized_diff = if diff > 180.0 { 360.0 - diff } else { diff };
                            normalized_diff <= 3.0 // 3° activation threshold
                        })
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                
                // Draw connection lines for active midpoints
                for &(_, p1_idx, p2_idx) in &active_midpoints {
                    let p1 = &planets[p1_idx];
                    let p2 = &planets[p2_idx];
                    let (x1, y1) = degrees_to_coords(p1.longitude, RADIUS + 25.0);
                    let (x2, y2) = degrees_to_coords(p2.longitude, RADIUS + 25.0);
                    context.begin_path();
                    context.move_to(x1, y1);
                    context.line_to(x2, y2);
                    context.stroke();
                }
                
                // Draw planets
                renderer::draw_all_planets(&context, &planets, None, 25.0);
            }
        });
    }

    let dragging = create_rw_signal(false);
    let last_x = create_rw_signal(0);
    let last_y = create_rw_signal(0);

    let on_mouse_down = {
        let dial = dial.clone();
        move |e: web_sys::MouseEvent| {
            dragging.set(true);
            last_x.set(e.client_x());
            last_y.set(e.client_y());
        }
    };

    let on_mouse_move = {
        let dial = dial.clone();
        move |e: web_sys::MouseEvent| {
            if dragging.get() {
                if let Some(canvas) = dial.get_canvas_element() {
                    let rect = canvas.get_bounding_client_rect();
                    let center_x = rect.left() + rect.width() / 2.0;
                    let center_y = rect.top() + rect.height() / 2.0;
                    
                    let curr_x = e.client_x() as f64;
                    let curr_y = e.client_y() as f64;
                    let prev_x = last_x.get() as f64;
                    let prev_y = last_y.get() as f64;
                    
                    let prev_angle = (prev_y - center_y).atan2(prev_x - center_x).to_degrees();
                    let curr_angle = (curr_y - center_y).atan2(curr_x - center_x).to_degrees();
                    let delta = curr_angle - prev_angle;
                    
                    let new_rotation = (shared_rotation.get() - delta) % 360.0;
                    set_shared_rotation.set(new_rotation);
                    
                    last_x.set(e.client_x());
                    last_y.set(e.client_y());
                }
            }
        }
    };

    let on_mouse_up = move |_| {
        dragging.set(false);
    };

    view! {
        <div>
            <canvas
                id={canvas_id}
                width="400"
                height="400"
                style="cursor: move; touch-action: none;"
                on:mousedown=on_mouse_down
                on:mousemove=on_mouse_move
                on:mouseup=on_mouse_up
                on:mouseleave=on_mouse_up
            ></canvas>
        </div>
    }
}