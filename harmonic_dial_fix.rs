// This file contains the fixed implementation for the harmonic dial
// Key requirements addressed:
// 1. Planets are plotted using direct harmonic positions (longitude % harmonic_range)
// 2. Inner tick marks use counter-clockwise numbering
// 3. Tick marks rotate properly with the dial

fn draw_right_dial_fixed(
    context: &web_sys::CanvasRenderingContext2d,
    planets: &[Planet],
    harmonic_range: f64,
    rot: f64
) {
    // Draw circle...
    
    // Draw Harmonic Planets (fixed outside dial)
    context.set_font("14px Arial");
    for planet in planets {
        // Calculate effective position within the harmonic range
        // For a planet at 181° on 4th harmonic (90° range), this would be 181 % 90 = 1°
        let harmonic_pos_eff = planet.longitude % harmonic_range;
        
        // Use direct effective harmonic position for plotting
        let (px, py) = degrees_to_coords(harmonic_pos_eff, RADIUS + 25.0);
        
        // Draw outer static tick mark for planet
        context.set_stroke_style(&"#999999".into());
        context.set_line_width(1.0);
        let (tx1, ty1) = degrees_to_coords(harmonic_pos_eff, RADIUS);
        let (tx2, ty2) = degrees_to_coords(harmonic_pos_eff, RADIUS + 20.0);
        context.begin_path();
        context.move_to(tx1, ty1);
        context.line_to(tx2, ty2);
        context.stroke();
        
        // Draw planet symbol
        context.set_fill_style(&planet.color.into());
        let _ = context.fill_text(&planet.symbol, px - 5.0, py + 5.0);
    }
    
    // Draw Inner Moving Harmonic Tick Marks and Labels (counter-clockwise numbering)
    context.set_fill_style(&"#FFFFFF".into());
    context.set_font("10px Arial");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    
    let label_step = match harmonic_range {
        hr if hr <= 30.0 => 1.0,  // e.g., 12th harmonic (30 deg range), label every 1 deg
        hr if hr <= 60.0 => 5.0,  // e.g., 6th harmonic (60 deg range), label every 5 deg
        hr if hr <= 90.0 => 5.0,  // e.g., 4th harmonic (90 deg range), label every 5 deg
        hr if hr <= 120.0 => 10.0, // e.g., 3rd harmonic (120 deg range), label every 10 deg
        _ => 15.0,                 // e.g., 2nd harmonic (180 deg range), label every 15 deg
    };
    
    let mut h_deg = 0.0;
    while h_deg < harmonic_range {
        // Draw inner tick mark that rotates with the dial
        // Note: We're using counter-clockwise numbering as requested
        // For the visual canvas, 0 is at the top and we count counter-clockwise
        
        // For counter-clockwise numbering:
        // When dial rotation is 0, 0° is at the top
        // The tick marks rotate with the dial
        let tick_angle = (rot + h_deg) % 360.0;
        
        context.set_stroke_style(&"#CCCCCC".into());
        context.set_line_width(1.0);
        let (tick_x1, tick_y1) = degrees_to_coords(tick_angle, RADIUS - 2.0);
        let (tick_x2, tick_y2) = degrees_to_coords(tick_angle, RADIUS - 15.0);
        context.begin_path();
        context.move_to(tick_x1, tick_y1);
        context.line_to(tick_x2, tick_y2);
        context.stroke();
        
        // Draw label
        let (lx, ly) = degrees_to_coords(tick_angle, RADIUS - 25.0);
        context.fill_text(&format!("{}", h_deg as i32), lx, ly).unwrap();
        
        h_deg += label_step;
    }
    
    // Calculate midpoint...
    for &(_, p1_idx, p2_idx) in &active_midpoints {
        let p1 = &planets[*p1_idx];
        let p2 = &planets[*p2_idx];
        
        // Calculate effective harmonic positions (folded into the segment)
        let p1_harm_pos_eff = p1.longitude % harmonic_range;
        let p2_harm_pos_eff = p2.longitude % harmonic_range;
        
        // Calculate coordinates using direct harmonic positions
        let (x1, y1) = degrees_to_coords(p1_harm_pos_eff, RADIUS + 25.0);
        let (x2, y2) = degrees_to_coords(p2_harm_pos_eff, RADIUS + 25.0);
        
        // Draw line between planets
        context.begin_path();
        context.move_to(x1, y1);
        context.line_to(x2, y2);
        context.stroke();
    }
}

// Helper function to calculate harmonic midpoint
fn calculate_harmonic_midpoint(pos1: f64, pos2: f64, harmonic_base: f64) -> f64 {
    let midpoint_360 = calculate_360_midpoint(pos1, pos2);
    let harmonic_range = 360.0 / harmonic_base;
    midpoint_360 % harmonic_range
}
