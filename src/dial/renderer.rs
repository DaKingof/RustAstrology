use super::types::*;
use super::calculator::AstrologyCalculator;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsValue;

pub struct DialRenderer {
    calculator: AstrologyCalculator,
}

impl DialRenderer {
    pub fn new() -> Self {
        Self {
            calculator: AstrologyCalculator::new(),
        }
    }

    /// Render the complete dial
    pub fn render(&self, canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d, state: &DialState) -> Result<(), JsValue> {
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = (width.min(height) / 2.0) * 0.8;

        // Clear canvas with a visible background first to test
        context.clear_rect(0.0, 0.0, width, height);
        
        // Fill canvas with a semi-transparent background
        context.set_fill_style(&wasm_bindgen::JsValue::from_str("rgba(127, 90, 240, 0.1)"));
        context.fill_rect(0.0, 0.0, width, height);
        
        // Draw FIXED elements (these do NOT rotate with the dial)
        self.draw_dial_background(context, center_x, center_y, radius)?;
        self.draw_planets(context, &state.planets, center_x, center_y, radius)?;
        
        // Calculate current alignments for interactive display
        let current_alignments = self.calculate_live_alignments(&state.planets, &state.midpoints, state.rotation);
        
        // Draw interactive midpoint connection lines (fixed layer)
        self.draw_midpoint_connection_lines(context, &current_alignments, center_x, center_y, radius)?;
        
        // Draw the rotating axis cross (this is the only thing that rotates)
        context.save();
        context.translate(center_x, center_y)?;
        context.rotate(state.rotation * std::f64::consts::PI / 180.0)?;
        context.translate(-center_x, -center_y)?;
        self.draw_axis_cross(context, center_x, center_y, radius)?;
        context.restore();
        
        // Draw axis alignment markers at fixed positions (not rotated)
        self.draw_axis_alignment_markers(context, &current_alignments, center_x, center_y, radius)?;
        
        // Draw UI elements (not rotated)
        self.draw_ui_elements(context, state, width, height)?;
        
        Ok(())
    }

    /// Draw the dial background with degree markings
    fn draw_dial_background(&self, context: &CanvasRenderingContext2d, center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        // Draw a filled circle background
        context.begin_path();
        context.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)?;
        context.set_fill_style(&wasm_bindgen::JsValue::from_str("rgba(127, 90, 240, 0.2)"));
        context.fill();
        
        // Outer circle border
        context.begin_path();
        context.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)?;
        context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#7f5af0"));
        context.set_line_width(3.0);
        context.stroke();

        // Inner circle  
        context.begin_path();
        context.arc(center_x, center_y, radius * 0.7, 0.0, 2.0 * std::f64::consts::PI)?;
        context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#2cb67d"));
        context.set_line_width(2.0);
        context.stroke();

        // Degree markings - draw marks around the dial for 90° range
        // Count counterclockwise from top: 0° at top, then 1°, 2°, 3°... going left
        context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#fffffe"));
        context.set_line_width(1.0);
        
        // Draw 90 degree markings for the 90° dial (0-90 degrees)
        for i in 0..=90 {
            let dial_angle = i as f64; // 0-90 degrees on the dial
            // Convert to canvas angle: counterclockwise from top
            // 0° at top, increasing counterclockwise (left direction)
            let canvas_angle = 360.0 - (dial_angle * 4.0); // Reverse direction for counterclockwise
            let canvas_angle = if canvas_angle == 360.0 { 0.0 } else { canvas_angle }; // Handle 360° = 0°
            let angle_rad = (canvas_angle - 90.0) * std::f64::consts::PI / 180.0; // -90 to start at top
            
            let line_length = if i % 15 == 0 { 
                radius * 0.15 // Major marks every 15 degrees (at 0°, 15°, 30°, 45°, 60°, 75°, 90°)
            } else if i % 5 == 0 {
                radius * 0.10 // Medium marks every 5 degrees
            } else { 
                radius * 0.05 // Minor marks every degree
            };
            
            let x1 = center_x + (radius - line_length) * angle_rad.cos();
            let y1 = center_y + (radius - line_length) * angle_rad.sin();
            let x2 = center_x + radius * angle_rad.cos();
            let y2 = center_y + radius * angle_rad.sin();
            
            context.begin_path();
            context.move_to(x1, y1);
            context.line_to(x2, y2);
            context.stroke();
            
            // Add degree labels for major marks
            if i % 15 == 0 {
                let label_radius = radius - line_length - 15.0;
                let label_x = center_x + label_radius * angle_rad.cos();
                let label_y = center_y + label_radius * angle_rad.sin();
                
                context.set_fill_style(&wasm_bindgen::JsValue::from_str("#fffffe"));
                context.set_font("10px sans-serif");
                context.set_text_align("center");
                context.set_text_baseline("middle");
                context.fill_text(&format!("{}°", i), label_x, label_y)?;
            }
        }
        
        Ok(())
    }

    /// Draw the axis cross at the four critical points
    fn draw_axis_cross(&self, context: &CanvasRenderingContext2d, center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        context.save();
        context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#ff6b6b"));
        context.set_line_width(3.0);
        
        for axis in AxisPoint::all() {
            // Convert 90° dial positions to 360° canvas coordinates
            // 90° dial goes counter-clockwise: 0° (top) → 22.5° (left) → 45° (bottom) → 67.5° (right)
            let angle = match axis {
                AxisPoint::Zero => 0.0,           // 0° at top (0°)
                AxisPoint::TwentyTwoFive => 270.0, // 22.5° at left (270°)
                AxisPoint::FortyFive => 180.0,    // 45° at bottom (180°)
                AxisPoint::SixtySevenFive => 90.0, // 67.5° at right (90°)
            };
            
            let (x1, y1) = self.calculator.position_on_circle(center_x, center_y, radius * 0.6, angle);
            let (x2, y2) = self.calculator.position_on_circle(center_x, center_y, radius * 1.05, angle);
            
            context.begin_path();
            context.move_to(x1, y1);
            context.line_to(x2, y2);
            context.stroke();
            
            // Draw axis point label
            let (label_x, label_y) = self.calculator.position_on_circle(center_x, center_y, radius * 1.15, angle);
            context.set_fill_style(&wasm_bindgen::JsValue::from_str("#ff6b6b"));
            context.set_font("bold 14px sans-serif");
            context.set_text_align("center");
            context.fill_text(&format!("{:.1}°", axis.degrees()), label_x, label_y)?;
        }
        
        context.restore();
        Ok(())
    }

    /// Draw planets on the dial
    fn draw_planets(&self, context: &CanvasRenderingContext2d, planets: &[PlanetPosition], center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        // Colors for different planets
        let colors = ["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff"];
        
        for (i, planet) in planets.iter().enumerate().take(6) {
            let dial_angle = planet.dial_position(); // This gives 0-90° dial position
            // Convert to canvas angle: counterclockwise from top (same as degree markings)
            let canvas_angle = 360.0 - (dial_angle * 4.0); // Reverse direction for counterclockwise
            let canvas_angle = if canvas_angle == 360.0 { 0.0 } else { canvas_angle };
            let angle_rad = (canvas_angle - 90.0) * std::f64::consts::PI / 180.0; // -90 to start at top
            
            let planet_radius = radius * 0.85;
            let x = center_x + planet_radius * angle_rad.cos();
            let y = center_y + planet_radius * angle_rad.sin();
            
            // Draw planet circle
            context.begin_path();
            context.arc(x, y, 8.0, 0.0, 2.0 * std::f64::consts::PI)?;
            context.set_fill_style(&wasm_bindgen::JsValue::from_str(colors[i % colors.len()]));
            context.fill();
            
            // Draw planet symbol
            context.set_fill_style(&wasm_bindgen::JsValue::from_str("#ffffff"));
            context.set_font("14px Arial");
            context.set_text_align("center");
            context.set_text_baseline("middle");
            context.fill_text(planet.planet.glyph(), x, y)?;
        }
        
        Ok(())
    }

    /// Calculate current live alignments based on dial rotation
    fn calculate_live_alignments(&self, planets: &[PlanetPosition], midpoints: &[Midpoint], current_rotation: f64) -> Vec<LiveAlignment> {
        let mut alignments = Vec::new();
        
        // Fixed axis positions (these don't rotate)
        let axis_positions = [
            (0.0, "0°", AxisPoint::Zero),
            (270.0, "22.5°", AxisPoint::TwentyTwoFive), 
            (180.0, "45°", AxisPoint::FortyFive),
            (90.0, "67.5°", AxisPoint::SixtySevenFive),
        ];
        
        let orb_tolerance = 1.0; // 1 degree orb
        
        for (axis_canvas_angle, axis_label, axis_point) in &axis_positions {
            // Calculate what dial position this axis currently points to (accounting for rotation)
            let effective_dial_position = (current_rotation / 4.0) % 90.0;
            
            // Find axis value in dial degrees (0-90 range)
            let axis_dial_degrees = match axis_point {
                AxisPoint::Zero => 0.0,
                AxisPoint::TwentyTwoFive => 22.5,
                AxisPoint::FortyFive => 45.0,
                AxisPoint::SixtySevenFive => 67.5,
            };
            
            // Adjust for current rotation to find what dial position this axis points to
            let target_dial_position = (axis_dial_degrees + effective_dial_position) % 90.0;
            
            // Find midpoints that align with this axis position
            let mut aligned_midpoints = Vec::new();
            for midpoint in midpoints {
                let orb_diff = self.calculate_orb_difference_90(midpoint.dial_position, target_dial_position);
                if orb_diff <= orb_tolerance {
                    aligned_midpoints.push((midpoint.clone(), orb_diff));
                }
            }
            
            // Find planets that align with this axis position  
            let mut aligned_planets = Vec::new();
            for planet in planets {
                let orb_diff = self.calculate_orb_difference_90(planet.dial_position(), target_dial_position);
                if orb_diff <= orb_tolerance {
                    aligned_planets.push((*planet, orb_diff));
                }
            }
            
            alignments.push(LiveAlignment {
                axis_canvas_angle: *axis_canvas_angle,
                axis_label: axis_label.to_string(),
                axis_point: *axis_point,
                target_dial_position,
                aligned_midpoints,
                aligned_planets,
            });
        }
        
        alignments
    }

    /// Draw connection lines between planets to show midpoints
    fn draw_midpoint_connection_lines(&self, context: &CanvasRenderingContext2d, alignments: &[LiveAlignment], center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        context.save();
        context.set_stroke_style(&"#ffd93d".into());
        context.set_line_width(2.0);
        context.set_global_alpha(0.7);
        
        for alignment in alignments {
            for (midpoint, _orb) in &alignment.aligned_midpoints {
                // Find the positions of the two planets that form this midpoint
                if let (Some(planet1_pos), Some(planet2_pos)) = (
                    self.find_planet_canvas_position(&midpoint.planet1, center_x, center_y, radius),
                    self.find_planet_canvas_position(&midpoint.planet2, center_x, center_y, radius)
                ) {
                    // Draw line between the two planets
                    context.begin_path();
                    context.move_to(planet1_pos.0, planet1_pos.1);
                    context.line_to(planet2_pos.0, planet2_pos.1);
                    context.stroke();
                    
                    // Draw midpoint marker at the midpoint location
                    let midpoint_canvas_angle = 360.0 - (midpoint.dial_position * 4.0);
                    let midpoint_canvas_angle = if midpoint_canvas_angle == 360.0 { 0.0 } else { midpoint_canvas_angle };
                    let (mid_x, mid_y) = self.calculator.position_on_circle(center_x, center_y, radius * 0.75, midpoint_canvas_angle);
                    
                    context.begin_path();
                    context.arc(mid_x, mid_y, 4.0, 0.0, 2.0 * std::f64::consts::PI)?;
                    context.set_fill_style(&"#ffd93d".into());
                    context.fill();
                    context.set_stroke_style(&"#ff6b6b".into());
                    context.set_line_width(1.0);
                    context.stroke();
                    
                    // Draw line from midpoint to axis point
                    let (axis_x, axis_y) = self.calculator.position_on_circle(center_x, center_y, radius * 1.1, alignment.axis_canvas_angle);
                    context.begin_path();
                    context.move_to(mid_x, mid_y);
                    context.line_to(axis_x, axis_y);
                    context.set_stroke_style(&"#ff6b6b".into());
                    context.set_line_width(1.0);
                    context.stroke();
                }
            }
        }
        
        context.restore();
        Ok(())
    }

    /// Find the canvas position of a planet by looking up its dial position
    fn find_planet_canvas_position(&self, planet: &Planet, center_x: f64, center_y: f64, radius: f64) -> Option<(f64, f64)> {
        // We need to calculate this from the planet's dial position
        // This is a simplified version - in practice you'd look up the actual planet data
        let dial_position = match planet {
            Planet::Sun => 13.19,     // From US Sibley chart data
            Planet::Moon => 27.1,     
            Planet::Mercury => 24.12, 
            Planet::Venus => 3.06,    
            Planet::Mars => 21.22,    
            Planet::Jupiter => 5.56,  
            _ => return None, // Only show main planets for now
        };
        
        let canvas_angle = 360.0 - (dial_position * 4.0);
        let canvas_angle = if canvas_angle == 360.0 { 0.0 } else { canvas_angle };
        let planet_radius = radius * 0.85;
        let (x, y) = self.calculator.position_on_circle(center_x, center_y, planet_radius, canvas_angle);
        Some((x, y))
    }

    /// Draw simple axis alignment markers (no text overlay)
    fn draw_axis_alignment_markers(&self, context: &CanvasRenderingContext2d, alignments: &[LiveAlignment], center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        context.save();
        
        for alignment in alignments {
            // Draw indicator at fixed axis position
            let (axis_x, axis_y) = self.calculator.position_on_circle(center_x, center_y, radius * 1.1, alignment.axis_canvas_angle);
            
            // Draw axis marker
            context.begin_path();
            context.arc(axis_x, axis_y, 8.0, 0.0, 2.0 * std::f64::consts::PI)?;
            
            if !alignment.aligned_midpoints.is_empty() || !alignment.aligned_planets.is_empty() {
                context.set_fill_style(&"#ffd93d".into()); // Yellow for active alignments
            } else {
                context.set_fill_style(&"rgba(255, 107, 107, 0.3)".into()); // Dim red for no alignments
            }
            context.fill();
            
            context.set_stroke_style(&"#ff6b6b".into());
            context.set_line_width(2.0);
            context.stroke();
            
            // Draw axis label
            let (label_x, label_y) = self.calculator.position_on_circle(center_x, center_y, radius * 1.2, alignment.axis_canvas_angle);
            context.set_fill_style(&"#ff6b6b".into());
            context.set_font("bold 12px sans-serif");
            context.set_text_align("center");
            context.set_text_baseline("middle");
            context.fill_text(&alignment.axis_label, label_x, label_y)?;
        }
        
        context.restore();
        Ok(())
    }
    
    /// Calculate orb difference for 90° dial (with wraparound)
    fn calculate_orb_difference_90(&self, position1: f64, position2: f64) -> f64 {
        let diff = (position1 - position2).abs();
        diff.min(90.0 - diff) // Account for circular nature of 90° dial
    }

    /// Draw UI elements (rotation indicator, alignment list, etc.)
    fn draw_ui_elements(&self, context: &CanvasRenderingContext2d, state: &DialState, _width: f64, height: f64) -> Result<(), JsValue> {
        context.save();
        
        // Draw rotation indicator
        context.set_fill_style(&"#fffffe".into());
        context.set_font("16px sans-serif");
        context.set_text_align("left");
        context.fill_text(&format!("Rotation: {:.1}°", state.rotation), 10.0, 30.0)?;
        
        // Draw alignment count
        context.fill_text(&format!("Alignments: {}", state.alignments.len()), 10.0, 55.0)?;
        
        // Draw alignment list
        let mut y_offset = 80.0;
        for (i, alignment) in state.alignments.iter().enumerate() {
            if i >= 10 { break; } // Limit display to first 10 alignments
            
            let color = if alignment.orb < 0.5 {
                "#ff6b6b" // Red for tight orbs
            } else if alignment.orb < 1.0 {
                "#ffd93d" // Yellow for medium orbs
            } else {
                "#2cb67d" // Green for wide orbs
            };
            
            context.set_fill_style(&color.into());
            context.set_font("12px monospace");
            context.fill_text(&alignment.label(), 10.0, y_offset)?;
            y_offset += 20.0;
        }
        
        // Draw instructions
        let instructions = [
            "Drag to rotate dial",
            "Shift/Ctrl for fine control",
            "Shift+Ctrl for extra fine",
            "Wheel for precision rotation"
        ];
        
        let mut instr_y = height - 80.0;
        context.set_fill_style(&"#94a1b2".into());
        context.set_font("11px sans-serif");
        
        for instruction in &instructions {
            context.fill_text(instruction, 10.0, instr_y)?;
            instr_y += 15.0;
        }
        
        context.restore();
        Ok(())
    }
}
