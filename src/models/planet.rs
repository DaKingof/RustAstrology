// Planet data structures and related functionality

/// Represents a celestial body with position and visual properties
#[derive(Debug, Clone)]
pub struct Planet {
    pub name: &'static str,
    pub symbol: &'static str,
    pub longitude: f64,
    pub color: &'static str,
}

impl Planet {
    /// Creates a new Planet instance
    pub fn new(name: &'static str, symbol: &'static str, longitude: f64, color: &'static str) -> Self {
        Self {
            name,
            symbol,
            longitude,
            color,
        }
    }
    
    /// Returns the longitude in formatted degrees and minutes
    pub fn formatted_longitude(&self) -> String {
        format_degree(self.longitude)
    }
}

/// Returns the US Sibley Chart data - a commonly used natal chart for the USA
pub fn get_sibley_chart() -> Vec<Planet> {
    vec![
        Planet { name: "Sun", symbol: "☉", longitude: 13.0, color: "#FFD700" },
        Planet { name: "Moon", symbol: "☽", longitude: 76.0, color: "#C0C0C0" },
        Planet { name: "Mercury", symbol: "☿", longitude: 95.0, color: "#FFA500" },
        Planet { name: "Venus", symbol: "♀", longitude: 63.0, color: "#90EE90" },
        Planet { name: "Mars", symbol: "♂", longitude: 52.0, color: "#FF6347" },
        Planet { name: "Jupiter", symbol: "♃", longitude: 18.0, color: "#1E90FF" },
        Planet { name: "Saturn", symbol: "♄", longitude: 350.0, color: "#8B4513" },
        Planet { name: "Uranus", symbol: "♅", longitude: 56.0, color: "#00CED1" },
        Planet { name: "Neptune", symbol: "♆", longitude: 162.0, color: "#4169E1" },
        Planet { name: "Pluto", symbol: "♇", longitude: 274.0, color: "#8B0000" },
    ]
}

/// Calculate the midpoint between two positions on a 360° circle
pub fn calculate_360_midpoint(pos1: f64, pos2: f64) -> f64 {
    let p1 = ((pos1 % 360.0) + 360.0) % 360.0;
    let p2 = ((pos2 % 360.0) + 360.0) % 360.0;
    let mut midpoint = (p1 + p2) / 2.0;
    if (p1 - p2).abs() > 180.0 {
        midpoint = (midpoint + 180.0) % 360.0;
    }
    midpoint
}

/// Calculate all midpoints between all pairs of planets
pub fn calculate_all_midpoints(planets: &[Planet]) -> Vec<(f64, usize, usize)> {
    let mut midpoints = Vec::new();
    for (i, p1) in planets.iter().enumerate() {
        for (j, p2) in planets.iter().enumerate().skip(i + 1) {
            let midpoint_360 = calculate_360_midpoint(p1.longitude, p2.longitude);
            midpoints.push((midpoint_360, i, j));
        }
    }
    midpoints
}

/// Gets planet pairs for midpoint calculations - convenience wrapper around calculate_all_midpoints
pub fn get_planet_pairs_for_midpoints(planets: &[Planet]) -> Vec<(f64, usize, usize)> {
    calculate_all_midpoints(planets)
}

/// Format a degree value into degrees and minutes format
pub fn format_degree(degree: f64) -> String {
    let d = degree as i32;
    let m = ((degree - d as f64) * 60.0) as i32;
    format!("{}°{:02}'", d, m)
}
