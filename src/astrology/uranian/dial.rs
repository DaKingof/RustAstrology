use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::astrology::models::planet::{Planet, PlanetPosition};
use crate::astrology::models::zodiac::ZodiacSign;
use crate::utils::angle::Angle;

/// Represents a Uranian astrology dial with all required data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UranianDial {
    /// Zodiac degrees (0-360)
    pub degrees: Vec<f64>,
    /// Planetary positions in degrees
    pub planets: HashMap<Planet, Angle>,
    /// Current harmonic setting (1 = normal, 2 = half-dial, etc.)
    pub harmonic: u32,
    /// Current rotation of the dial in degrees
    pub rotation: f64,
    /// Zoom level (0.5 - 2.0)
    pub zoom: f64,
    /// Whether to show planets on the dial
    pub show_planets: bool,
    /// Whether to show midpoints on the dial
    pub show_midpoints: bool,
    /// Whether to show zodiac signs
    pub show_zodiac: bool,
    /// Whether to show degree markings
    pub show_degrees: bool,
}

impl Default for UranianDial {
    fn default() -> Self {
        Self {
            degrees: (0..360).collect(),
            planets: HashMap::new(),
            harmonic: 1,
            rotation: 0.0,
            zoom: 1.0,
            show_planets: true,
            show_midpoints: true,
            show_zodiac: true,
            show_degrees: true,
        }
    }
}

impl UranianDial {
    /// Create a new dial with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the harmonic factor for the dial
    pub fn set_harmonic(&mut self, harmonic: u32) -> Result<(), String> {
        if harmonic == 0 {
            return Err("Harmonic must be greater than 0".to_string());
        }
        self.harmonic = harmonic;
        Ok(())
    }
    
    /// Get the effective angle for a given position, applying the current harmonic
    pub fn get_effective_angle(&self, angle: f64) -> f64 {
        (angle * self.harmonic as f64) % 360.0
    }
    
    /// Rotate the dial by a certain number of degrees
    pub fn rotate_by(&mut self, degrees: f64) {
        self.rotation = (self.rotation + degrees) % 360.0;
    }
    
    /// Set the rotation of the dial to a specific angle
    pub fn set_rotation(&mut self, degrees: f64) {
        self.rotation = degrees % 360.0;
    }
    
    /// Zoom the dial by a factor
    pub fn zoom_by(&mut self, factor: f64) -> f64 {
        self.zoom = (self.zoom * factor).max(0.5).min(2.0);
        self.zoom
    }
    
    /// Set the zoom level
    pub fn set_zoom(&mut self, zoom: f64) -> f64 {
        self.zoom = zoom.max(0.5).min(2.0);
        self.zoom
    }
    
    /// Add or update a planet's position
    pub fn update_planet_position(&mut self, planet: Planet, position: f64) {
        self.planets.insert(planet, Angle::from_degrees(position));
    }
    
    /// Remove a planet from the dial
    pub fn remove_planet(&mut self, planet: &Planet) -> Option<Angle> {
        self.planets.remove(planet)
    }
    
    /// Get a planet's position, applying the current harmonic
    pub fn get_planet_position(&self, planet: &Planet) -> Option<f64> {
        self.planets.get(planet).map(|angle| {
            self.get_effective_angle(angle.degrees())
        })
    }
    
    /// Get all planet positions with their effective angles
    pub fn get_planet_positions(&self) -> Vec<(Planet, f64)> {
        self.planets.iter()
            .map(|(planet, angle)| (*planet, self.get_effective_angle(angle.degrees())))
            .collect()
    }
    
    /// Calculate midpoints between all pairs of planets
    pub fn calculate_midpoints(&self) -> Vec<Midpoint> {
        let mut midpoints = Vec::new();
        let planets: Vec<_> = self.planets.keys().collect();
        
        for i in 0..planets.len() {
            for j in (i + 1)..planets.len() {
                let p1 = planets[i];
                let p2 = planets[j];
                
                if let (Some(a1), Some(a2)) = (self.planets.get(p1), self.planets.get(p2)) {
                    let midpoint = a1.midpoint(*a2);
                    midpoints.push(Midpoint {
                        point1: *p1,
                        point2: *p2,
                        angle: midpoint,
                    });
                }
            }
        }
        
        midpoints
    }
    
    /// Get the current rotation of the dial in radians
    pub fn rotation_radians(&self) -> f64 {
        self.rotation.to_radians()
    }
    
    /// Get the current zoom factor
    pub fn zoom_factor(&self) -> f64 {
        self.zoom
    }
}

/// Represents a midpoint between two points on the dial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Midpoint {
    pub point1: Planet,
    pub point2: Planet,
    pub angle: Angle,
}

impl Midpoint {
    /// Get the effective angle of the midpoint, applying the current harmonic
    pub fn get_effective_angle(&self, harmonic: u32) -> f64 {
        (self.angle.degrees() * harmonic as f64) % 360.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use std::f64::consts::PI;

    #[test]
    fn test_dial_creation() {
        let dial = UranianDial::new();
        assert_eq!(dial.harmonic, 1);
        assert_eq!(dial.rotation, 0.0);
        assert_eq!(dial.zoom, 1.0);
        assert!(dial.planets.is_empty());
    }

    #[test]
    fn test_set_harmonic() {
        let mut dial = UranianDial::new();
        assert!(dial.set_harmonic(4).is_ok());
        assert_eq!(dial.harmonic, 4);
        
        assert!(dial.set_harmonic(0).is_err());
    }

    #[test]
    fn test_effective_angle() {
        let mut dial = UranianDial::new();
        assert_eq!(dial.get_effective_angle(45.0), 45.0);
        
        dial.set_harmonic(2).unwrap();
        assert_eq!(dial.get_effective_angle(45.0), 90.0);
        assert_eq!(dial.get_effective_angle(200.0), 40.0);
    }

    #[test]
    fn test_rotation() {
        let mut dial = UranianDial::new();
        dial.rotate_by(90.0);
        assert_eq!(dial.rotation, 90.0);
        
        dial.rotate_by(300.0);
        assert_eq!(dial.rotation, 30.0);
        
        dial.set_rotation(180.0);
        assert_eq!(dial.rotation, 180.0);
    }

    #[test]
    fn test_zoom() {
        let mut dial = UranianDial::new();
        assert_eq!(dial.zoom_by(1.5), 1.5);
        assert_eq!(dial.zoom_by(0.5), 0.75);
        assert_eq!(dial.set_zoom(2.5), 2.0);
        assert_eq!(dial.set_zoom(0.1), 0.5);
    }

    #[test]
    fn test_planet_management() {
        let mut dial = UranianDial::new();
        
        // Add planets
        dial.update_planet_position(Planet::Sun, 30.0);
        dial.update_planet_position(Planet::Moon, 60.0);
        
        // Check positions
        assert!(approx_eq!(f64, dial.get_planet_position(&Planet::Sun).unwrap(), 30.0, epsilon = 0.0001));
        assert!(dial.get_planet_position(&Planet::Mercury).is_none());
        
        // Test harmonic application
        dial.set_harmonic(2).unwrap();
        assert!(approx_eq!(f64, dial.get_planet_position(&Planet::Sun).unwrap(), 60.0, epsilon = 0.0001));
        
        // Remove planet
        dial.remove_planet(&Planet::Sun);
        assert!(dial.get_planet_position(&Planet::Sun).is_none());
    }

    #[test]
    fn test_midpoint_calculation() {
        let mut dial = UranianDial::new();
        dial.update_planet_position(Planet::Sun, 0.0);
        dial.update_planet_position(Planet::Moon, 60.0);
        dial.update_planet_position(Planet::Mercury, 90.0);
        
        let midpoints = dial.calculate_midpoints();
        assert_eq!(midpoints.len(), 3);
        
        // Check Sun-Moon midpoint (0° + 60°) / 2 = 30°
        let sun_moon = midpoints.iter()
            .find(|m| (m.point1 == Planet::Sun && m.point2 == Planet::Moon) || 
                     (m.point2 == Planet::Sun && m.point1 == Planet::Moon))
            .unwrap();
        assert!(approx_eq!(f64, sun_moon.angle.degrees(), 30.0, epsilon = 0.0001));
    }
}
