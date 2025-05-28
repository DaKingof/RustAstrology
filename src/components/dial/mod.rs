// Base dial component module
pub mod renderer;
pub mod utils;

use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::utils::constants::*;

/// The base Dial component that both LeftDial and RightDial extend
#[derive(Clone)]
pub struct Dial {
    pub canvas_id: &'static str,
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub rotation: f64,
}

impl Dial {
    /// Creates a new Dial with default settings
    pub fn new(canvas_id: &'static str) -> Self {
        Self {
            canvas_id,
            center_x: CENTER_X,
            center_y: CENTER_Y,
            radius: RADIUS,
            rotation: 0.0,
        }
    }

    /// Returns the canvas element for this dial
    pub fn get_canvas_element(&self) -> Option<HtmlCanvasElement> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let canvas = document.get_element_by_id(self.canvas_id)?;
        canvas.dyn_into::<HtmlCanvasElement>().ok()
    }

    /// Returns the 2D rendering context for this dial's canvas
    pub fn get_context(&self) -> Option<CanvasRenderingContext2d> {
        let canvas = self.get_canvas_element()?;
        canvas
            .get_context("2d")
            .ok()??
            .dyn_into::<CanvasRenderingContext2d>()
            .ok()
    }
}
