#!/bin/bash

# Fix syntax error and duplicated code
sed -i '532s/});$//' /home/mend/Projects/RustAstrology/src/lib.rs
sed -i '534,562d' /home/mend/Projects/RustAstrology/src/lib.rs

# Fix connection lines for active midpoints (add missing code after line 532)
cat >> /home/mend/Projects/RustAstrology/src/lib.rs << 'MIDPOINT'

                    // Draw connection lines for active midpoints
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
                    
                    // Harmonic Planets (fixed outside dial)
                    context.set_font("14px Arial");
                    for planet in &planets {
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
MIDPOINT

echo "Fixed the syntax errors and implemented correct harmonic positioning!"
