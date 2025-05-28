// Utility functions specific to dial components
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::models::planet::Planet;
use crate::models::harmonic::HarmonicType;
use crate::utils::math::degrees_to_coords;
use crate::utils::constants::*;

/// Calculates appropriate label step size based on harmonic range
pub fn calculate_label_step(harmonic_range: f64) -> f64 {
    match harmonic_range {
        hr if hr <= 30.0 => 1.0,
        hr if hr <= 60.0 => 5.0,
        hr if hr <= 90.0 => 5.0,
        hr if hr <= 120.0 => 10.0,
        _ => 15.0,
    }
}

/// Determines which midpoints are active based on axis arms
pub fn filter_active_midpoints(
    midpoints: &[(f64, usize, usize)],
    arm_angles: &[f64],
    activation_threshold: f64,
) -> Vec<(f64, usize, usize)> {
    midpoints
        .iter()
        .filter(|(midpoint_angle, _, _)| {
            arm_angles.iter().any(|&arm_angle| {
                let diff = (midpoint_angle - arm_angle).abs() % 360.0;
                let normalized_diff = if diff > 180.0 { 360.0 - diff } else { diff };
                normalized_diff <= activation_threshold
            })
        })
        .cloned()
        .collect()
}

/// Sets up the canvas with the correct dimensions and scaling
pub fn setup_canvas(context: &CanvasRenderingContext2d) {
    // Make sure the canvas is properly sized
    if let Some(canvas) = context.canvas() {
        canvas.set_width(CANVAS_SIZE as u32);
        canvas.set_height(CANVAS_SIZE as u32);
    }
    
    // Set default text rendering properties
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_font(DEGREE_LABEL_FONT_SIZE);
}

/// Draws all planets on the dial with the correct positioning
pub fn draw_all_planets(
    context: &CanvasRenderingContext2d,
    planets: &[Planet],
    harmonic: Option<HarmonicType>,
    radius_offset: f64,
) {
    context.set_font(PLANET_SYMBOL_FONT_SIZE);
    
    for planet in planets {
        let position = if let Some(h) = harmonic {
            // For harmonic dial, apply modulo based on harmonic range
            planet.longitude % h.harmonic_range()
        } else {
            // For standard 360° dial
            planet.longitude
        };
        
        // Draw tick mark
        context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&PLANET_TICK_COLOR));
        context.set_line_width(1.0);
        
        let (inner_x, inner_y) = degrees_to_coords(position, RADIUS);
        let (outer_x, outer_y) = degrees_to_coords(position, RADIUS + radius_offset);
        
        context.begin_path();
        context.move_to(inner_x, inner_y);
        context.line_to(outer_x, outer_y);
        context.stroke();
        
        // Draw planet symbol
        context.set_fill_style(&wasm_bindgen::JsValue::from_str(&planet.color));
        let (px, py) = degrees_to_coords(position, RADIUS + radius_offset + 5.0);
        let _ = context.fill_text(&planet.symbol, px, py);
    }
}
