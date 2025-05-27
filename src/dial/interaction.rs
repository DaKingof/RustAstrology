use super::types::*;
use nalgebra::Vector2;
use web_sys::{MouseEvent, TouchEvent, WheelEvent, HtmlCanvasElement};

pub struct InteractionHandler {
    pub sensitivity: f64,
    pub fine_sensitivity: f64,
    pub extra_fine_sensitivity: f64,
}

impl Default for InteractionHandler {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            fine_sensitivity: 0.1,
            extra_fine_sensitivity: 0.01,
        }
    }
}

impl InteractionHandler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get canvas-relative coordinates from mouse event
    pub fn get_canvas_coords(&self, event: &MouseEvent, canvas: &HtmlCanvasElement) -> Option<(f64, f64)> {
        // Cast to Element to access getBoundingClientRect
        let element: &web_sys::Element = canvas.as_ref();
        let rect = element.get_bounding_client_rect();
        let canvas_x = event.client_x() as f64 - rect.left();
        let canvas_y = event.client_y() as f64 - rect.top();
        Some((canvas_x, canvas_y))
    }

    /// Calculate angle from center to point (in degrees, 0-360)
    pub fn calculate_angle_from_center(&self, x: f64, y: f64, center_x: f64, center_y: f64) -> f64 {
        let dx = x - center_x;
        let dy = y - center_y;
        let angle_rad = dy.atan2(dx);
        let angle_deg = angle_rad * 180.0 / std::f64::consts::PI;
        // Normalize to 0-360 range
        (angle_deg + 360.0) % 360.0
    }

    /// Handle mouse down event
    pub fn handle_mouse_down(&self, state: &mut DialState, event: &MouseEvent, canvas: &HtmlCanvasElement) {
        if let Some((canvas_x, canvas_y)) = self.get_canvas_coords(event, canvas) {
            state.is_dragging = true;
            state.last_mouse_pos = Some(Vector2::new(canvas_x, canvas_y));
        }
    }

    /// Handle mouse move event  
    pub fn handle_mouse_move(&self, state: &mut DialState, event: &MouseEvent, canvas: &HtmlCanvasElement) {
        if !state.is_dragging {
            return;
        }

        if let Some((canvas_x, canvas_y)) = self.get_canvas_coords(event, canvas) {
            if let Some(last_pos) = state.last_mouse_pos {
                let center_x = canvas.width() as f64 / 2.0;
                let center_y = canvas.height() as f64 / 2.0;
                
                // Calculate angles from center to both positions
                let last_angle = self.calculate_angle_from_center(last_pos.x, last_pos.y, center_x, center_y);
                let current_angle = self.calculate_angle_from_center(canvas_x, canvas_y, center_x, center_y);
                
                // Calculate angular difference (handle wraparound)
                let mut angle_delta = current_angle - last_angle;
                if angle_delta > 180.0 {
                    angle_delta -= 360.0;
                } else if angle_delta < -180.0 {
                    angle_delta += 360.0;
                }
                
                let sensitivity = self.get_sensitivity_for_modifiers(event.shift_key(), event.ctrl_key());
                let rotation_delta = angle_delta * sensitivity;
                state.add_rotation(rotation_delta);
                state.last_mouse_pos = Some(Vector2::new(canvas_x, canvas_y));
            }
        }
    }

    /// Handle mouse up event
    pub fn handle_mouse_up(&self, state: &mut DialState, _event: &MouseEvent) {
        state.is_dragging = false;
        state.last_mouse_pos = None;
    }

    /// Handle wheel event for fine rotation control
    pub fn handle_wheel(&self, state: &mut DialState, event: &WheelEvent) {
        let sensitivity = self.get_sensitivity_for_modifiers(event.shift_key(), event.ctrl_key());
        let rotation_delta = -event.delta_y() * sensitivity * 0.1;
        state.add_rotation(rotation_delta);
    }

    /// Handle touch start
    pub fn handle_touch_start(&self, state: &mut DialState, event: &TouchEvent) {
        if let Some(touch) = event.touches().get(0) {
            state.is_dragging = true;
            state.last_mouse_pos = Some(Vector2::new(touch.client_x() as f64, touch.client_y() as f64));
        }
    }

    /// Handle touch move
    pub fn handle_touch_move(&self, state: &mut DialState, event: &TouchEvent, canvas: &HtmlCanvasElement) {
        if !state.is_dragging {
            return;
        }

        if let Some(touch) = event.touches().get(0) {
            if let Some(last_pos) = state.last_mouse_pos {
                let center_x = canvas.width() as f64 / 2.0;
                let center_y = canvas.height() as f64 / 2.0;
                
                let last_angle = self.calculate_angle_from_center(last_pos.x, last_pos.y, center_x, center_y);
                let current_angle = self.calculate_angle_from_center(touch.client_x() as f64, touch.client_y() as f64, center_x, center_y);
                
                let mut angle_delta = current_angle - last_angle;
                if angle_delta > 180.0 {
                    angle_delta -= 360.0;
                } else if angle_delta < -180.0 {
                    angle_delta += 360.0;
                }
                
                let rotation_delta = angle_delta * self.sensitivity;
                state.add_rotation(rotation_delta);
                state.last_mouse_pos = Some(Vector2::new(touch.client_x() as f64, touch.client_y() as f64));
            }
        }
    }

    /// Handle touch end
    pub fn handle_touch_end(&self, state: &mut DialState, _event: &TouchEvent) {
        state.is_dragging = false;
        state.last_mouse_pos = None;
    }

    /// Get sensitivity based on modifier keys
    fn get_sensitivity_for_modifiers(&self, shift: bool, ctrl: bool) -> f64 {
        match (shift, ctrl) {
            (true, true) => self.extra_fine_sensitivity, // Shift+Ctrl = extra fine
            (true, false) => self.fine_sensitivity,       // Shift = fine
            (false, true) => self.fine_sensitivity,       // Ctrl = fine
            (false, false) => self.sensitivity,           // Normal
        }
    }
}
