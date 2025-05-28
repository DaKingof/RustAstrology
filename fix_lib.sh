#!/bin/bash

# This script fixes the harmonic positioning and removes duplicate code
# The source file should be backed up first
BACKUP="/home/mend/Projects/RustAstrology/src/lib.rs.backup"
SOURCE="/home/mend/Projects/RustAstrology/src/lib.rs"
TEMP="/tmp/lib.rs.tmp"

# Step 1: Fix the basic midpoint calculation function
sed -i 's/fn calculate_midpoint(a1: f64, a2: f64) -> f64 {/fn calculate_360_midpoint(pos1: f64, pos2: f64) -> f64 {/' "$SOURCE"
sed -i 's/let diff = (a2 - a1 + 360.0) % 360.0;/let p1 = ((pos1 % 360.0) + 360.0) % 360.0;\n    let p2 = ((pos2 % 360.0) + 360.0) % 360.0;\n    \n    let mut midpoint = (p1 + p2) \/ 2.0;\n    \n    if (p1 - p2).abs() > 180.0 {\n        midpoint = (midpoint + 180.0) % 360.0;\n    }/' "$SOURCE"
sed -i 's/(a1 + diff \/ 2.0) % 360.0/midpoint/' "$SOURCE"

# Step 2: Add harmonic midpoint calculation function
sed -i '/fn calculate_360_midpoint/,/}/!b;/}/a\\
// Calculate the harmonic midpoint between two positions\\
fn calculate_harmonic_midpoint(pos1: f64, pos2: f64, harmonic_base: f64) -> f64 {\\
    // First calculate the 360° midpoint\\
    let midpoint_360 = calculate_360_midpoint(pos1, pos2);\\
    \\
    // Convert to harmonic position (fold into the harmonic range)\\
    let harmonic_range = 360.0 / harmonic_base;\\
    midpoint_360 % harmonic_range\\
}' "$SOURCE"

# Step 3: Fix the get_planet_pairs_for_midpoints to use calculate_360_midpoint
sed -i 's/let mp = calculate_midpoint/let mp = calculate_360_midpoint/' "$SOURCE"

# Step 4: Fix the RightDial's planet plotting
# Remove duplicated tick mark sections
grep -v -E "// Draw Inner Moving Harmonic Tick Marks.*duplicated section" "$SOURCE" > "$TEMP"
mv "$TEMP" "$SOURCE"

echo "Fixed lib.rs midpoint and harmonic positioning calculations"
