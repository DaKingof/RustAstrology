#!/usr/bin/env bash
set -e

# Restore from backup if it exists
if [ -f "/home/mend/Projects/RustAstrology/src/lib.rs.backup" ]; then
  cp /home/mend/Projects/RustAstrology/src/lib.rs.backup /home/mend/Projects/RustAstrology/src/lib.rs
  echo "Restored lib.rs from backup"
else
  echo "No backup found, using current lib.rs"
fi

# Apply changes directly to specific areas using sed
# 1. Fix the planet plotting on harmonic dial to use direct harmonic positions
sed -i 's/\(let harmonic_pos_eff = planet\.longitude % harmonic_range;\).*let (px, py) = degrees_to_coords(.*)/\1\n                        \/\/ Use direct effective harmonic position for plotting\n                        let (px, py) = degrees_to_coords(harmonic_pos_eff, RADIUS + 25.0);/' /home/mend/Projects/RustAstrology/src/lib.rs

# 2. Fix the tick mark coords to use direct harmonic position 
sed -i 's/\(let (tx1, ty1) = degrees_to_coords(\).*\(, RADIUS);\)/\1harmonic_pos_eff\2/' /home/mend/Projects/RustAstrology/src/lib.rs
sed -i 's/\(let (tx2, ty2) = degrees_to_coords(\).*\(, RADIUS + 20.0);\)/\1harmonic_pos_eff\2/' /home/mend/Projects/RustAstrology/src/lib.rs

# 3. Add counter-clockwise numbering for inner tick marks - look for existing inner tick mark section
sed -i '/Draw Inner Moving Harmonic Tick Marks and Labels/{n;n;n;n;n;n;n;n;n;n;n;n;n;n;n;s/\/\/ h_deg is the harmonic degree value/\/\/ For counter-clockwise numbering:/}' /home/mend/Projects/RustAstrology/src/lib.rs
sed -i '/For counter-clockwise numbering:/,+10{/let tick_angle/s/.*/                        \/\/ When dial rotation is 0, 0° is at the top\n                        \/\/ The tick marks rotate with the dial\n                        let tick_angle = (rot + h_deg) % 360.0;/}' /home/mend/Projects/RustAstrology/src/lib.rs

# Now run the app in dev mode
echo "Starting development environment"
cd /home/mend/Projects/RustAstrology/ && ./dev.sh
