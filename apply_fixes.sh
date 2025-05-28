#!/bin/bash
set -e

# Restore from the most recent backup
cp /home/mend/Projects/RustAstrology/src/lib.rs.new_backup /home/mend/Projects/RustAstrology/src/lib.rs

# 1. Update the comment for inner tick marks to indicate counter-clockwise numbering
sed -i 's/    \/\/ Draw Inner Moving Harmonic Tick Marks and Labels/    \/\/ Draw Inner Moving Harmonic Tick Marks and Labels (counter-clockwise numbering)/g' /home/mend/Projects/RustAstrology/src/lib.rs

# 2. Update the inner tick mark positioning code to clearly indicate counter-clockwise numbering
# Find the line with "// The tick mark's position" and replace it and the two lines after it
LINE_NUM=$(grep -n "// The tick mark's position" /home/mend/Projects/RustAstrology/src/lib.rs | head -1 | cut -d: -f1)
if [ ! -z "$LINE_NUM" ]; then
  # Replace with the counter-clockwise explanation and code
  sed -i "${LINE_NUM}s/.*/                        \/\/ For counter-clockwise numbering:/" /home/mend/Projects/RustAstrology/src/lib.rs
  sed -i "$((LINE_NUM+1))s/.*/                        \/\/ When dial rotation is 0, 0° is at the top/" /home/mend/Projects/RustAstrology/src/lib.rs
  sed -i "$((LINE_NUM+2))s/.*/                        \/\/ The tick marks rotate with the dial/" /home/mend/Projects/RustAstrology/src/lib.rs
fi

echo "Fixes applied successfully!"

# Build and run the application
cd /home/mend/Projects/RustAstrology/
./dev.sh
