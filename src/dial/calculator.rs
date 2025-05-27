use super::types::*;
use std::f64::consts::PI;

pub struct AstrologyCalculator;

impl AstrologyCalculator {
    pub fn new() -> Self {
        Self
    }

    /// Get the US Sibley chart data (July 4, 1776, 5:10 PM LMT, Philadelphia)
    /// These are the calculated positions for the Sibley chart in full zodiac degrees (0-360°)
    pub fn get_us_sibley_chart(&self) -> Vec<PlanetPosition> {
        vec![
            PlanetPosition::new(Planet::Sun, 103.19, 0.0, 0.0),         // 13° Cancer 19' = 90 + 13.19
            PlanetPosition::new(Planet::Moon, 327.1, 0.0, 0.0),         // 27° Aquarius 10' = 300 + 27.1
            PlanetPosition::new(Planet::Mercury, 114.12, 0.0, 0.0),     // 24° Cancer 12' = 90 + 24.12
            PlanetPosition::new(Planet::Venus, 63.06, 0.0, 0.0),        // 3° Gemini 06' = 60 + 3.06
            PlanetPosition::new(Planet::Mars, 81.22, 0.0, 0.0),         // 21° Gemini 22' = 60 + 21.22
            PlanetPosition::new(Planet::Jupiter, 95.56, 0.0, 0.0),      // 5° Cancer 56' = 90 + 5.56
            PlanetPosition::new(Planet::Saturn, 194.48, 0.0, 0.0),      // 14° Libra 48' = 180 + 14.48
            PlanetPosition::new(Planet::Uranus, 68.55, 0.0, 0.0),       // 8° Gemini 55' = 60 + 8.55
            PlanetPosition::new(Planet::Neptune, 172.25, 0.0, 0.0),     // 22° Virgo 25' = 150 + 22.25
            PlanetPosition::new(Planet::Pluto, 297.33, 0.0, 0.0),       // 27° Capricorn 33' = 270 + 27.33
            PlanetPosition::new(Planet::NorthNode, 126.35, 0.0, 0.0),   // 6° Leo 35' = 120 + 6.35
            PlanetPosition::new(Planet::SouthNode, 306.35, 0.0, 0.0),   // 6° Aquarius 35' = 300 + 6.35
            PlanetPosition::new(Planet::Ascendant, 252.21, 0.0, 0.0),   // 12° Sagittarius 21' = 240 + 12.21
            PlanetPosition::new(Planet::Midheaven, 181.19, 0.0, 0.0),   // 1° Libra 19' = 180 + 1.19
        ]
    }

    /// Generate sample planetary positions for demonstration
    /// In a real implementation, this would use Swiss Ephemeris
    pub fn calculate_positions(&self, datetime: chrono::DateTime<chrono::Utc>) -> Vec<PlanetPosition> {
        // For demo purposes, we'll create sample positions
        // In reality, this would use Swiss Ephemeris calculations
        let base_time = datetime.timestamp() as f64 / 86400.0; // Days since epoch
        
        Planet::all().into_iter().enumerate().map(|(_i, planet)| {
            let base_longitude = match planet {
                Planet::Sun => (base_time * 0.9856) % 360.0,
                Planet::Moon => (base_time * 13.176) % 360.0,
                Planet::Mercury => (base_time * 4.092) % 360.0,
                Planet::Venus => (base_time * 1.602) % 360.0,
                Planet::Mars => (base_time * 0.524) % 360.0,
                Planet::Jupiter => (base_time * 0.083) % 360.0,
                Planet::Saturn => (base_time * 0.033) % 360.0,
                Planet::Uranus => (base_time * 0.012) % 360.0,
                Planet::Neptune => (base_time * 0.006) % 360.0,
                Planet::Pluto => (base_time * 0.004) % 360.0,
                Planet::NorthNode => (360.0 - (base_time * 0.053) % 360.0) % 360.0,
                Planet::SouthNode => (180.0 - (base_time * 0.053) % 360.0) % 360.0,
                Planet::Chiron => (base_time * 0.020) % 360.0,
                Planet::Ascendant => (base_time * 4.0 + 90.0) % 360.0, // Sample ASC
                Planet::Midheaven => (base_time * 4.0) % 360.0, // Sample MC
            };
            
            PlanetPosition::new(
                planet,
                base_longitude,
                0.0, // Latitude (simplified for 90° dial)
                0.0, // Speed (not needed for static demonstration)
            )
        }).collect()
    }

    /// Calculate all midpoints between planets
    pub fn calculate_midpoints(&self, planets: &[PlanetPosition]) -> Vec<Midpoint> {
        let mut midpoints = Vec::new();
        
        for i in 0..planets.len() {
            for j in (i + 1)..planets.len() {
                let midpoint = Midpoint::new(
                    planets[i].planet,
                    planets[i].longitude,
                    planets[j].planet,
                    planets[j].longitude,
                );
                midpoints.push(midpoint);
            }
        }
        
        midpoints
    }

    /// Check for alignments with axis points within given orb
    pub fn check_alignments(&self, planets: &[PlanetPosition], midpoints: &[Midpoint], orb: f64) -> Vec<AxisAlignment> {
        let mut alignments = Vec::new();
        let axis_points = AxisPoint::all();

        // Check midpoint alignments
        for midpoint in midpoints {
            for axis in &axis_points {
                let orb_diff = self.calculate_orb_difference(midpoint.dial_position, axis.degrees());
                if orb_diff <= orb {
                    alignments.push(AxisAlignment::new_midpoint(*axis, midpoint.clone(), orb_diff));
                }
            }
        }

        // Check planet alignments
        for planet in planets {
            for axis in &axis_points {
                let orb_diff = self.calculate_orb_difference(planet.dial_position(), axis.degrees());
                if orb_diff <= orb {
                    alignments.push(AxisAlignment::new_planet(*axis, *planet, orb_diff));
                }
            }
        }

        alignments
    }

    /// Calculate the orb difference between a position and an axis point
    fn calculate_orb_difference(&self, position: f64, axis_degrees: f64) -> f64 {
        let diff = (position - axis_degrees).abs();
        diff.min(90.0 - diff) // Account for circular nature of 90° dial
    }

    /// Convert degree to radian
    pub fn deg_to_rad(&self, degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    /// Convert radian to degree
    pub fn rad_to_deg(&self, radians: f64) -> f64 {
        radians * 180.0 / PI
    }

    /// Calculate position on dial circle
    pub fn position_on_circle(&self, center_x: f64, center_y: f64, radius: f64, angle_degrees: f64) -> (f64, f64) {
        let angle_rad = self.deg_to_rad(angle_degrees - 90.0); // -90 to start at top
        let x = center_x + radius * angle_rad.cos();
        let y = center_y + radius * angle_rad.sin();
        (x, y)
    }
}
