#!/bin/bash
set -e

# First, replace the way harmonic position is calculated for planets in RightDial
# This ensures planets at positions like 181° appear at 1° on a 4th harmonic dial (90° range)
sed -i '583,585s/let p1_harm_pos = (p1.longitude % harmonic_range) \* (360.0 \/ harmonic_range);/let p1_harm_pos_eff = p1.longitude % harmonic_range;/' /home/mend/Projects/RustAstrology/src/lib.rs
sed -i '586,588s/let p2_harm_pos = (p2.longitude % harmonic_range) \* (360.0 \/ harmonic_range);/let p2_harm_pos_eff = p2.longitude % harmonic_range;/' /home/mend/Projects/RustAstrology/src/lib.rs

# Replace the coordinates calculation for midpoint lines
sed -i '591,593s/let (x1, y1) = degrees_to_coords(p1_harm_pos, RADIUS + 25.0);/let (x1, y1) = degrees_to_coords(p1_harm_pos_eff, RADIUS + 25.0);/' /home/mend/Projects/RustAstrology/src/lib.rs
sed -i '594,596s/let (x2, y2) = degrees_to_coords(p2_harm_pos, RADIUS + 25.0);/let (x2, y2) = degrees_to_coords(p2_harm_pos_eff, RADIUS + 25.0);/' /home/mend/Projects/RustAstrology/src/lib.rs

# Fix the planet positioning in the harmonic dial to use direct harmonic position
sed -i '609,611s/let harmonic_pos = (planet.longitude % harmonic_range) \* (360.0 \/ harmonic_range);/let harmonic_pos_eff = planet.longitude % harmonic_range;/' /home/mend/Projects/RustAstrology/src/lib.rs
sed -i '612,613s/let (px, py) = degrees_to_coords(harmonic_pos, RADIUS + 25.0);/let (px, py) = degrees_to_coords(harmonic_pos_eff, RADIUS + 25.0);/' /home/mend/Projects/RustAstrology/src/lib.rs

# Fix the tick marks to also use direct harmonic position
sed -i '618,619s/let (tx1, ty1) = degrees_to_coords(harmonic_pos, RADIUS);/let (tx1, ty1) = degrees_to_coords(harmonic_pos_eff, RADIUS);/' /home/mend/Projects/RustAstrology/src/lib.rs
sed -i '620,621s/let (tx2, ty2) = degrees_to_coords(harmonic_pos, RADIUS + 20.0);/let (tx2, ty2) = degrees_to_coords(harmonic_pos_eff, RADIUS + 20.0);/' /home/mend/Projects/RustAstrology/src/lib.rs

# Add counter-clockwise numbering for the inner tick marks
sed -i 's/Draw Inner Moving Harmonic Tick Marks and Labels/Draw Inner Moving Harmonic Tick Marks and Labels with Counter-Clockwise Numbering/' /home/mend/Projects/RustAstrology/src/lib.rs

# Add explanatory comments for the counter-clockwise numbering
sed -i '/The tick mark'"'"'s position is h_deg/c\                        \/\/ For counter-clockwise numbering:\n                        \/\/ When dial rotation is 0, 0° is at the top\n                        \/\/ The tick marks rotate with the dial' /home/mend/Projects/RustAstrology/src/lib.rs

echo "Applied harmonic positioning and counter-clockwise tick mark changes"
