use super::types::*;
use super::calculator::AstrologyCalculator;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsValue;

pub struct DialRenderer360 {
    calculator: AstrologyCalculator,
}

impl DialRenderer360 {
    pub fn new() -> Self {
        Self {
            calculator: AstrologyCalculator::new(),
        }
    }

    /// Render the complete 360° dial
    pub fn render(&self, canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d, state: &DialState) -> Result<(), JsValue> {
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = (width.min(height) / 2.0) * 0.8;

        // Clear canvas
        context.clear_rect(0.0, 0.0, width, height);
        
        // Fill canvas with background
        context.set_fill_style(&wasm_bindgen::JsValue::from_str("rgba(127, 90, 240, 0.1)"));
        context.fill_rect(0.0, 0.0, width, height);
        
        // Draw FIXED elements (these do NOT rotate with the dial)
        self.draw_dial_background(context, center_x, center_y, radius)?;
        self.draw_planets_360(context, &state.planets, center_x, center_y, radius)?;
        
        // Calculate current alignments for interactive display
        let current_alignments = self.calculate_360_alignments(&state.planets, &state.midpoints, state.rotation);
        
        // Draw interactive midpoint connection lines (fixed layer)
        self.draw_midpoint_connection_lines_360(context, &current_alignments, center_x, center_y, radius)?;
        
        // Draw the rotating axis cross (this is the only thing that rotates)
        context.save();
        context.translate(center_x, center_y)?;
        context.rotate(state.rotation * std::f64::consts::PI / 180.0)?;
        context.translate(-center_x, -center_y)?;
        self.draw_axis_cross_360(context, center_x, center_y, radius)?;
        context.restore();
        
        // Draw axis alignment markers at fixed positions (not rotated)
        self.draw_axis_alignment_markers_360(context, &current_alignments, center_x, center_y, radius)?;
        
        Ok(())
    }

    /// Draw the 360° dial background with degree markings
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

        // Degree markings for 360° dial
        context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#fffffe"));
        context.set_line_width(1.0);
        
        // Draw markings every degree for 360° range
        for i in 0..360 {
            let degree = i as f64;
            let angle_rad = (degree - 90.0) * std::f64::consts::PI / 180.0; // -90 to start at top
            
            let line_length = if i % 30 == 0 { 
                radius * 0.15 // Major marks every 30 degrees (zodiac signs)
            } else if i % 10 == 0 {
                radius * 0.10 // Medium marks every 10 degrees
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
            
            // Add degree labels for major marks (every 30°)
            if i % 30 == 0 {
                let label_radius = radius - line_length - 15.0;
                let label_x = center_x + label_radius * angle_rad.cos();
                let label_y = center_y + label_radius * angle_rad.sin();
                
                context.set_fill_style(&wasm_bindgen::JsValue::from_str("#fffffe"));
                context.set_font("12px sans-serif");
                context.set_text_align("center");
                context.set_text_baseline("middle");
                context.fill_text(&format!("{}°", i), label_x, label_y)?;
            }
        }
        
        Ok(())
    }

    /// Draw planets on the 360° dial using their full longitude
    fn draw_planets_360(&self, context: &CanvasRenderingContext2d, planets: &[PlanetPosition], center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        // Colors for different planets
        let colors = ["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff"];
        
        for (i, planet) in planets.iter().enumerate().take(6) {
            let longitude = planet.longitude; // Use full longitude (0-360°)
            let angle_rad = (longitude - 90.0) * std::f64::consts::PI / 180.0; // -90 to start at top
            
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

    /// Draw the four cardinal axis points for 360° dial
    fn draw_axis_cross_360(&self, context: &CanvasRenderingContext2d, center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        context.save();
        context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#ff6b6b"));
        context.set_line_width(3.0);
        
        // Four cardinal points: 0° (Aries), 90° (Cancer), 180° (Libra), 270° (Capricorn)
        let axis_positions = [0.0, 90.0, 180.0, 270.0];
        let axis_labels = ["0° (♈)", "90° (♋)", "180° (♎)", "270° (♑)"];
        
        for (i, &angle) in axis_positions.iter().enumerate() {
            let (x1, y1) = self.calculator.position_on_circle(center_x, center_y, radius * 0.6, angle);
            let (x2, y2) = self.calculator.position_on_circle(center_x, center_y, radius * 1.05, angle);
            
            context.begin_path();
            context.move_to(x1, y1);
            context.line_to(x2, y2);
            context.stroke();
            
            // Draw axis point label
            let (label_x, label_y) = self.calculator.position_on_circle(center_x, center_y, radius * 1.15, angle);
            context.set_fill_style(&wasm_bindgen::JsValue::from_str("#ff6b6b"));
            context.set_font("bold 12px sans-serif");
            context.set_text_align("center");
            context.fill_text(&axis_labels[i], label_x, label_y)?;
        }
        
        context.restore();
        Ok(())
    }

    /// Calculate alignments for 360° dial
    fn calculate_360_alignments(&self, planets: &[PlanetPosition], midpoints: &[Midpoint], current_rotation: f64) -> Vec<Live360Alignment> {
        let mut alignments = Vec::new();
        
        // Cardinal axis positions for 360° dial
        let axis_positions = [
            (0.0, "0° (Aries)", Cardinal360::Aries),
            (90.0, "90° (Cancer)", Cardinal360::Cancer),
            (180.0, "180° (Libra)", Cardinal360::Libra),
            (270.0, "270° (Capricorn)", Cardinal360::Capricorn),
        ];
        
        let orb_tolerance = 1.0; // 1 degree orb
        
        for (axis_angle, axis_label, cardinal) in &axis_positions {
            // Calculate what 360° position this axis currently points to (accounting for rotation)
            let target_position = (axis_angle + current_rotation) % 360.0;
            
            // Find midpoints that align with this axis position
            let mut aligned_midpoints = Vec::new();
            for midpoint in midpoints {
                let orb_diff = self.calculate_orb_difference_360(midpoint.position, target_position);
                if orb_diff <= orb_tolerance {
                    aligned_midpoints.push((midpoint.clone(), orb_diff));
                }
            }
            
            // Find planets that align with this axis position  
            let mut aligned_planets = Vec::new();
            for planet in planets {
                let orb_diff = self.calculate_orb_difference_360(planet.longitude, target_position);
                if orb_diff <= orb_tolerance {
                    aligned_planets.push((*planet, orb_diff));
                }
            }
            
            alignments.push(Live360Alignment {
                axis_angle: *axis_angle,
                axis_label: axis_label.to_string(),
                cardinal: *cardinal,
                target_position,
                aligned_midpoints,
                aligned_planets,
            });
        }
        
        alignments
    }

    /// Calculate orb difference for 360° dial (with wraparound)
    fn calculate_orb_difference_360(&self, position1: f64, position2: f64) -> f64 {
        let diff = (position1 - position2).abs();
        diff.min(360.0 - diff) // Account for circular nature of 360° dial
    }

    /// Draw connection lines for 360° dial
    fn draw_midpoint_connection_lines_360(&self, context: &CanvasRenderingContext2d, alignments: &[Live360Alignment], center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        context.save();
        context.set_stroke_style(&"#ffd93d".into());
        context.set_line_width(2.0);
        context.set_global_alpha(0.7);
        
        for alignment in alignments {
            for (midpoint, _orb) in &alignment.aligned_midpoints {
                // Find the positions of the two planets that form this midpoint
                if let (Some(planet1_pos), Some(planet2_pos)) = (
                    self.find_planet_canvas_position_360(&midpoint.planet1, center_x, center_y, radius),
                    self.find_planet_canvas_position_360(&midpoint.planet2, center_x, center_y, radius)
                ) {
                    // Draw line between the two planets
                    context.begin_path();
                    context.move_to(planet1_pos.0, planet1_pos.1);
                    context.line_to(planet2_pos.0, planet2_pos.1);
                    context.stroke();
                    
                    // Draw midpoint marker at the midpoint location
                    let angle_rad = (midpoint.position - 90.0) * std::f64::consts::PI / 180.0;
                    let (mid_x, mid_y) = self.calculator.position_on_circle(center_x, center_y, radius * 0.75, midpoint.position);
                    
                    context.begin_path();
                    context.arc(mid_x, mid_y, 4.0, 0.0, 2.0 * std::f64::consts::PI)?;
                    context.set_fill_style(&"#ffd93d".into());
                    context.fill();
                    context.set_stroke_style(&"#ff6b6b".into());
                    context.set_line_width(1.0);
                    context.stroke();
                    
                    // Draw line from midpoint to axis point
                    let (axis_x, axis_y) = self.calculator.position_on_circle(center_x, center_y, radius * 1.05, alignment.axis_angle);
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

    /// Find the canvas position of a planet in 360° dial
    fn find_planet_canvas_position_360(&self, planet: &Planet, center_x: f64, center_y: f64, radius: f64) -> Option<(f64, f64)> {
        // Use the actual longitude values from US Sibley chart
        let longitude = match planet {
            Planet::Sun => 103.19,     
            Planet::Moon => 327.1,     
            Planet::Mercury => 114.12, 
            Planet::Venus => 63.06,    
            Planet::Mars => 81.22,    
            Planet::Jupiter => 95.56,  
            _ => return None,
        };
        
        let planet_radius = radius * 0.85;
        let (x, y) = self.calculator.position_on_circle(center_x, center_y, planet_radius, longitude);
        Some((x, y))
    }

    /// Draw simple axis alignment markers for 360° dial
    fn draw_axis_alignment_markers_360(&self, context: &CanvasRenderingContext2d, alignments: &[Live360Alignment], center_x: f64, center_y: f64, radius: f64) -> Result<(), JsValue> {
        context.save();
        
        for alignment in alignments {
            // Draw indicator at fixed axis position
            let (axis_x, axis_y) = self.calculator.position_on_circle(center_x, center_y, radius * 1.1, alignment.axis_angle);
            
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
        }
        
        context.restore();
        Ok(())
    }
}
