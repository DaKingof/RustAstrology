// Rendering utilities for the dial components
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;

use crate::models::planet::Planet;
use crate::utils::math::degrees_to_coords;
use wasm_bindgen::prelude::*;
use crate::utils::constants::*;

/// Clears the canvas and prepares it for rendering
pub fn clear_canvas(context: &CanvasRenderingContext2d) {
    context.clear_rect(0.0, 0.0, CANVAS_SIZE, CANVAS_SIZE);
    context.begin_path();
}

/// Draws the main circle outline of a dial
pub fn draw_circle(context: &CanvasRenderingContext2d) {
    context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&BASE_STROKE_COLOR));
    context.set_line_width(2.0);
    context.begin_path();
    context.arc(CENTER_X, CENTER_Y, RADIUS, 0.0, 2.0 * PI);
    context.stroke();
}

/// Draws tick marks at regular intervals around the circle
pub fn draw_tick_marks(context: &CanvasRenderingContext2d, rot: f64, interval: f64, count: usize) {
    context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&SECONDARY_STROKE_COLOR));
    context.set_line_width(1.0);
    
    for i in 0..count {
        let degree = i as f64 * interval;
        let angle = (degree + rot) % 360.0;
        let (outer_x, outer_y) = degrees_to_coords(angle, RADIUS);
        let (inner_x, inner_y) = degrees_to_coords(angle, RADIUS - 10.0);
        
        context.begin_path();
        context.move_to(outer_x, outer_y);
        context.line_to(inner_x, inner_y);
        context.stroke();
    }
}

/// Draws major axis lines (usually at 0°, 90°, 180°, 270°)
pub fn draw_axis_lines(context: &CanvasRenderingContext2d, rot: f64, count: usize) {
    context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&SECONDARY_STROKE_COLOR));
    context.set_line_width(1.5);
    
    for i in 0..count {
        let angle = (i as f64 * (360.0 / count as f64) + rot) % 360.0;
        let (x1, y1) = degrees_to_coords(angle, 0.0);
        let (x2, y2) = degrees_to_coords(angle, RADIUS);
        
        context.begin_path();
        context.move_to(x1, y1);
        context.line_to(x2, y2);
        context.stroke();
    }
}

/// Draws a planet symbol on the dial
pub fn draw_planet(
    context: &CanvasRenderingContext2d, 
    planet: &Planet, 
    position: f64, 
    radius_offset: f64
) {
    // Draw tick mark from circle to planet
    context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&PLANET_TICK_COLOR));
    context.set_line_width(1.0);
    
    let (inner_x, inner_y) = degrees_to_coords(position, RADIUS);
    let (outer_x, outer_y) = degrees_to_coords(position, RADIUS + radius_offset);
    
    context.begin_path();
    context.move_to(inner_x, inner_y);
    context.line_to(outer_x, outer_y);
    context.stroke();
    
    // Draw the planet symbol
    context.set_font(PLANET_SYMBOL_FONT_SIZE);
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&planet.color));
    
    let (px, py) = degrees_to_coords(position, RADIUS + radius_offset + 5.0);
    let _ = context.fill_text(&planet.symbol, px, py);
}

/// Draws midpoint connection lines between planets
pub fn draw_midpoint_line(
    context: &CanvasRenderingContext2d,
    pos1: f64,
    pos2: f64,
    radius_offset: f64
) {
    context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&MIDPOINT_STROKE_COLOR));
    context.set_line_width(1.0);
    
    let (x1, y1) = degrees_to_coords(pos1, RADIUS + radius_offset);
    let (x2, y2) = degrees_to_coords(pos2, RADIUS + radius_offset);
    
    context.begin_path();
    context.move_to(x1, y1);
    context.line_to(x2, y2);
    context.stroke();
}

/// Draws degree labels around the dial
pub fn draw_degree_labels(
    context: &CanvasRenderingContext2d,
    rot: f64,
    start_deg: f64,
    end_deg: f64,
    step: f64,
    radius_offset: f64
) {
    context.set_font(DEGREE_LABEL_FONT_SIZE);
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&BASE_STROKE_COLOR));
    
    let mut deg = start_deg;
    while deg < end_deg {
        let angle = (rot + deg) % 360.0;
        let (lx, ly) = degrees_to_coords(angle, RADIUS + radius_offset);
        let _ = context.fill_text(&format!("{}", deg as i32), lx, ly);
        deg += step;
    }
}

/// Draws all planets on the dial
pub fn draw_all_planets(
    context: &CanvasRenderingContext2d,
    planets: &[Planet],
    rotation: Option<f64>,
    radius_offset: f64
) {
    for planet in planets {
        let mut position = planet.longitude;
        if let Some(rot) = rotation {
            position = (position + rot) % 360.0;
        }
        draw_planet(context, planet, position, radius_offset);
    }
}
