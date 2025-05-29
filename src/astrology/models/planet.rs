use std::collections::HashMap;
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};
use strum::IntoEnumIterator;
use serde::{Serialize, Deserialize};

// Import the Angle type if it exists, otherwise use a simple type alias
#[cfg(feature = "angle_module")]
use crate::utils::angle::Angle;

#[cfg(not(feature = "angle_module"))]
type Angle = f64;

// Re-export for convenience
pub use strum::EnumCount;
pub use strum::IntoEnumIterator;

/// Represents the type of celestial body
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString, Serialize, Deserialize, EnumCount)]
pub enum BodyType {
    Star,
    Planet,
    DwarfPlanet,
    Asteroid,
    Centaur,
    TransNeptunian,
    LunarNode,
    HouseCusp,
    Angle,
    Point,
}

/// Represents a celestial body in the astrological chart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Planet {
    // Luminaries
    Sun,
    Moon,
    
    // Personal Planets
    Mercury,
    Venus,
    Mars,
    
    // Social Planets
    Jupiter,
    Saturn,
    
    // Outer Planets
    Uranus,
    Neptune,
    Pluto,
    
    // Asteroids and Dwarf Planets
    Ceres,
    Pallas,
    Juno,
    Vesta,
    Chiron,
    
    // Lunar Nodes
    TrueNode,
    MeanNode,
    
    // Hamburg School Points
    Cupido,
    Hades,
    Zeus,
    Kronos,
    Apollon,
    Admetos,
    Vulkanus,
    Poseidon,
    
    // Other Points
    Vertex,
    EastPoint,
    
    // House Cusps (1-12)
    Ascendant,  // 1st House Cusp
    MC,         // 10th House Cusp
    
    // Additional Points
    BlackMoonLilith,
    WhiteMoonSelena,
    
    // Fixed Stars (selected)
    Regulus,
    Spica,
    Antares,
    Aldebaran,
}

impl Planet {
    /// Get the body type of this celestial body
    pub fn body_type(&self) -> BodyType {
        match self {
            // Luminaries
            Planet::Sun | Planet::Moon => BodyType::Star,
            
            // Planets
            Planet::Mercury | Planet::Venus | Planet::Mars | 
            Planet::Jupiter | Planet::Saturn | Planet::Uranus | 
            Planet::Neptune => BodyType::Planet,
            
            // Dwarf Planets and Asteroids
            Planet::Pluto | Planet::Ceres | Planet::Eris | 
            Planet::Makemake | Planet::Haumea => BodyType::DwarfPlanet,
            
            // Asteroids and Centaurs
            Planet::Pallas | Planet::Juno | Planet::Vesta => BodyType::Asteroid,
            Planet::Chiron => BodyType::Centaur,
            
            // Lunar Nodes
            Planet::TrueNode | Planet::MeanNode => BodyType::LunarNode,
            
            // Hamburg School Points
            Planet::Cupido | Planet::Hades | Planet::Zeus | 
            Planet::Kronos | Planet::Apollon | Planet::Admetos | 
            Planet::Vulkanus | Planet::Poseidon => BodyType::TransNeptunian,
            
            // House Cusps and Angles
            Planet::Ascendant | Planet::MC => BodyType::HouseCusp,
            Planet::Vertex | Planet::EastPoint => BodyType::Angle,
            
            // Other Points
            Planet::BlackMoonLilith | Planet::WhiteMoonSelena => BodyType::Point,
            
            // Fixed Stars
            Planet::Regulus | Planet::Spica | 
            Planet::Antares | Planet::Aldebaran => BodyType::Star,
        }
    }

    /// Get the standard symbol for the planet
    pub fn symbol(&self) -> &'static str {
        match self {
            // Luminaries
            Planet::Sun => "☉",
            Planet::Moon => "☽",
            
            // Personal Planets
            Planet::Mercury => "☿",
            Planet::Venus => "♀",
            Planet::Mars => "♂",
            
            // Social Planets
            Planet::Jupiter => "♃",
            Planet::Saturn => "♄",
            
            // Outer Planets
            Planet::Uranus => "♅",
            Planet::Neptune => "♆",
            Planet::Pluto => "♇",
            
            // Asteroids and Dwarf Planets
            Planet::Ceres => "⚳",
            Planet::Pallas => "⚴",
            Planet::Juno => "⚵",
            Planet::Vesta => "⚶",
            Planet::Chiron => "⚷",
            
            // Lunar Nodes
            Planet::TrueNode | Planet::MeanNode => "☊",
            
            // Hamburg School Points
            Planet::Cupido => "C",
            Planet::Hades => "H",
            Planet::Zeus => "Z",
            Planet::Kronos => "K",
            Planet::Apollon => "A",
            Planet::Admetos => "D",
            Planet::Vulkanus => "V",
            Planet::Poseidon => "P",
            
            // Other Points
            Planet::Vertex => "Vx",
            Planet::EastPoint => "EP",
            Planet::Ascendant => "AC",
            Planet::MC => "MC",
            
            // Additional Points
            Planet::BlackMoonLilith => "⚸",
            Planet::WhiteMoonSelena => "⚪",
            
            // Fixed Stars
            Planet::Regulus => "★",
            Planet::Spica => "✧",
            Planet::Antares => "☆",
            Planet::Aldebaran => "✫",
        }
    }

    /// Get the standard color for the planet (in hex format, e.g., "#RRGGBB")
    pub fn color(&self) -> &'static str {
        match self {
            // Luminaries
            Planet::Sun => "#FFD700", // Gold
            Planet::Moon => "#C0C0C0", // Silver
            
            // Personal Planets
            Planet::Mercury => "#A9A9A9", // Dark Gray
            Planet::Venus => "#FFA500", // Orange
            Planet::Mars => "#FF4500", // Orange Red
            
            // Social Planets
            Planet::Jupiter => "#DAA520", // Golden Rod
            Planet::Saturn => "#8B4513", // Saddle Brown
            
            // Outer Planets
            Planet::Uranus => "#00BFFF", // Deep Sky Blue
            Planet::Neptune => "#1E90FF", // Dodger Blue
            Planet::Pluto => "#9932CC", // Dark Orchid
            
            // Asteroids and Dwarf Planets
            Planet::Ceres => "#8B4513", // Saddle Brown
            Planet::Pallas => "#228B22", // Forest Green
            Planet::Juno => "#FF69B4", // Hot Pink
            Planet::Vesta => "#FF8C00", // Dark Orange
            Planet::Chiron => "#32CD32", // Lime Green
            
            // Lunar Nodes
            Planet::TrueNode | Planet::MeanNode => "#8A2BE2", // Blue Violet
            
            // Hamburg School Points
            Planet::Cupido | Planet::Hades | Planet::Zeus | 
            Planet::Kronos | Planet::Apollon | Planet::Admetos | 
            Planet::Vulkanus | Planet::Poseidon => "#4B0082", // Indigo
            
            // Angles and Points
            Planet::Vertex | Planet::EastPoint | 
            Planet::Ascendant | Planet::MC => "#000000", // Black
            
            // Additional Points
            Planet::BlackMoonLilith => "#000000", // Black
            Planet::WhiteMoonSelena => "#FFFFFF", // White
            
            // Fixed Stars
            Planet::Regulus | Planet::Spica | 
            Planet::Antares | Planet::Aldebaran => "#FFD700", // Gold
        }
    }
    
    /// Get the orbital period in Earth years
    pub fn orbital_period_years(&self) -> Option<f64> {
        match self {
            Planet::Sun => None, // Central point
            Planet::Moon => Some(0.0748), // Synodic month
            Planet::Mercury => Some(0.2408),
            Planet::Venus => Some(0.6152),
            Planet::Mars => Some(1.8809),
            Planet::Jupiter => Some(11.8626),
            Planet::Saturn => Some(29.4475),
            Planet::Uranus => Some(84.0168),
            Planet::Neptune => Some(164.7913),
            Planet::Pluto => Some(247.9207),
            Planet::Chiron => Some(50.7),
            _ => None, // For fixed stars and other points
        }
    }
    
    /// Check if this body is a main planet (Sun to Pluto)
    pub fn is_main_planet(&self) -> bool {
        matches!(
            self,
            Planet::Sun | Planet::Moon | Planet::Mercury | Planet::Venus | 
            Planet::Mars | Planet::Jupiter | Planet::Saturn | Planet::Uranus | 
            Planet::Neptune | Planet::Pluto
        )
    }
    
    /// Check if this body is a Hamburg School point
    pub fn is_uranian_point(&self) -> bool {
        matches!(
            self,
            Planet::Cupido | Planet::Hades | Planet::Zeus | Planet::Kronos | 
            Planet::Apollon | Planet::Admetos | Planet::Vulkanus | Planet::Poseidon
        )
    }
}

/// Represents a planet's position in the chart
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlanetPosition {
    /// The celestial body this position represents
    pub planet: Planet,
    
    // Positional data
    /// Ecliptic longitude in degrees (0-360)
    pub longitude: f64,
    /// Ecliptic latitude in degrees (-90 to 90)
    pub latitude: f64,
    
    // Distance data (in AU)
    /// Distance from Earth in AU (legacy field, use distance_au instead)
    pub distance: f64,
    /// Distance from Earth in AU
    pub distance_au: f64,
    /// Distance from Earth in kilometers
    pub distance_km: f64,
    
    // Motion data
    /// Daily speed in longitude (degrees per day)
    pub speed: f64,
    /// Daily speed in latitude (degrees per day)
    pub speed_latitude: f64,
    /// Whether the planet is in retrograde motion
    pub is_retrograde: bool,
    
    // Observational data
    /// Apparent size in arcseconds
    pub apparent_size: f64,
    /// Apparent magnitude
    pub magnitude: f64,
    /// Phase (0-1, for Moon and inner planets)
    pub phase: f64,
    /// Elongation from Sun in degrees (0-180)
    pub elongation: f64,
    
    // Horizontal coordinates (observer-dependent)
    /// Altitude above horizon in degrees (-90 to 90)
    pub altitude: f64,
    /// Azimuth in degrees (0-360, North=0, East=90)
    pub azimuth: f64,
    
    // Astrological data
    /// House number (1-12) if calculated, None otherwise
    pub house: Option<u8>,
    /// Zodiac sign (0-11 for Aries-Pisces)
    pub zodiac_sign: u8,
    /// Degree within the zodiac sign (0-29.999...)
    pub zodiac_degree: f64,
    
    // Harmonic data
    /// Position in the current harmonic (0-360)
    pub harmonic_position: f64,
    /// Current harmonic (1-90)
    pub harmonic: u32,
}

impl PlanetPosition {
    /// Creates a new PlanetPosition with the given parameters
    ///
    /// # Arguments
    /// * `planet` - The celestial body
    /// * `longitude` - Ecliptic longitude in degrees (0-360)
    /// * `latitude` - Ecliptic latitude in degrees (-90 to 90)
    /// * `distance_au` - Distance from Earth in AU
    /// * `speed` - Daily speed in longitude (degrees per day)
    /// * `speed_latitude` - Daily speed in latitude (degrees per day)
    /// * `is_retrograde` - Whether the planet is in retrograde motion
    /// * `apparent_size` - Apparent size in arcseconds
    /// * `magnitude` - Apparent magnitude
    /// * `phase` - Phase (0-1, for Moon and inner planets)
    /// * `elongation` - Elongation from Sun in degrees (0-180)
    /// * `harmonic` - Current harmonic (1-90)
    pub fn new(
        planet: Planet,
        longitude: f64,
        latitude: f64,
        distance_au: f64,
        speed: f64,
        speed_latitude: f64,
        is_retrograde: bool,
        apparent_size: f64,
        magnitude: f64,
        phase: f64,
        elongation: f64,
        harmonic: u32,
    ) -> Self {
        let normalized_longitude = longitude.rem_euclid(360.0);
        let normalized_latitude = latitude.max(-90.0).min(90.0);
        let zodiac_sign = (normalized_longitude / 30.0).floor() as u8 % 12;
        let zodiac_degree = normalized_longitude % 30.0;
        
        // Calculate harmonic position
        let harmonic = harmonic.max(1); // Ensure harmonic is at least 1
        let harmonic_position = (normalized_longitude * harmonic as f64) % 360.0;
        
        // Calculate distance in km (1 AU = 149,597,870.7 km)
        let distance_km = distance_au * 149_597_870.7;
        
        // Ensure phase is in valid range
        let phase = phase.max(0.0).min(1.0);
        
        // Ensure elongation is in valid range (0-180)
        let elongation = elongation.rem_euclid(360.0);
        let elongation = if elongation > 180.0 { 360.0 - elongation } else { elongation };
        
        Self {
            planet,
            longitude: normalized_longitude,
            latitude: normalized_latitude,
            distance: distance_au, // Legacy field
            speed,
            speed_latitude,
            distance_au,
            distance_km: distance_au * 149_597_870.7, // Convert AU to km
            is_retrograde,
            apparent_size,
            magnitude,
            phase,
            elongation,
            altitude: 0.0,  // Will be set later based on location
            azimuth: 0.0,   // Will be set later based on location
            house: None,    // Will be set later based on house system
            zodiac_sign,
            zodiac_degree,
            harmonic,
        }
        
        // Update observational data
        self.apparent_size = apparent_size;
        self.magnitude = magnitude;
        self.phase = phase.max(0.0).min(1.0);
        
        // Ensure elongation is in valid range (0-180)
        let normalized_elongation = elongation.rem_euclid(360.0);
        self.elongation = if normalized_elongation > 180.0 { 
            360.0 - normalized_elongation 
        } else { 
            normalized_elongation 
        };
        
        // Update derived fields
        self.zodiac_sign = (self.longitude / 30.0).floor() as u8 % 12;
        self.zodiac_degree = self.longitude % 30.0;
        self.harmonic_position = (self.longitude * self.harmonic as f64) % 360.0;
    }

    /// Get the zodiac sign as a string (Aries, Taurus, etc.)
    pub fn zodiac_sign_name(&self) -> &'static str {
        const ZODIAC_SIGNS: [&str; 12] = [
            "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
            "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
        ];
        
        // Use modulo to ensure we have a valid index even if zodiac_sign is out of bounds
        let index = (self.zodiac_sign as usize) % ZODIAC_SIGNS.len();
        ZODIAC_SIGNS[index]
    }
    
    /// Format the position as a string (e.g., "15°30' Aries")
    pub fn to_formatted_string(&self) -> String {
        let degrees = self.zodiac_degree.floor() as u8;
        let minutes = ((self.zodiac_degree - degrees as f64) * 60.0).floor() as u8;
        
        format!(
            "{}°{}' {}{}",
            degrees,
            if minutes < 10 { format!("0{}", minutes) } else { format!("{}", minutes) },
            self.zodiac_sign_name(),
            if self.is_retrograde { " (R)" } else { "" }
        )
    }
    
    /// Update the position with new coordinates and properties
    pub fn update_position(
        &mut self,
        longitude: f64,
        latitude: f64,
        distance_au: f64,
        speed: f64,
        speed_latitude: f64,
        is_retrograde: bool,
        apparent_size: f64,
        magnitude: f64,
        phase: f64,
        elongation: f64,
    ) {
        // Update basic position and motion data
        self.longitude = longitude.rem_euclid(360.0);
        self.latitude = latitude.max(-90.0).min(90.0);
        
        // Update distance data
        self.distance = distance_au; // Legacy field
        self.distance_au = distance_au;
        self.distance_km = distance_au * 149_597_870.7; // Convert AU to km
        
        // Update motion data
        self.speed = speed;
        self.speed_latitude = speed_latitude;
        self.is_retrograde = is_retrograde;
        
        // Update observational data
        self.apparent_size = apparent_size;
        self.magnitude = magnitude;
        self.phase = phase.max(0.0).min(1.0);
        
        // Ensure elongation is in valid range (0-180)
        let normalized_elongation = elongation.rem_euclid(360.0);
        self.elongation = if normalized_elongation > 180.0 { 
            360.0 - normalized_elongation 
        } else { 
            normalized_elongation 
        };
        
        // Update derived fields
        self.zodiac_sign = (self.longitude / 30.0).floor() as u8 % 12;
        self.zodiac_degree = self.longitude % 30.0;
        self.harmonic_position = (self.longitude * self.harmonic as f64) % 360.0;
    }
    
    /// Set the harmonic for this position and recalculate harmonic position
    pub fn set_harmonic(&mut self, harmonic: u32) {
        self.harmonic = harmonic.max(1).min(90); // Cap at 90th harmonic
        self.harmonic_position = (self.longitude * self.harmonic as f64) % 360.0;
    }
    
    /// Set the house number for this position (1-12)
    pub fn set_house(&mut self, house: u8) {
        if (1..=12).contains(&house) {
            self.house = Some(house);
        }
    }
    
    /// Set the horizontal coordinates (altitude and azimuth)
    pub fn set_horizontal_coords(&mut self, altitude: f64, azimuth: f64) {
        self.altitude = altitude.max(-90.0).min(90.0);
        self.azimuth = azimuth.rem_euclid(360.0);
    }
    
    /// Get the position in the specified harmonic
    pub fn get_harmonic_position(&self, harmonic: u32) -> f64 {
        if harmonic == 0 {
            self.longitude
        } else {
            (self.longitude * harmonic as f64) % 360.0
        }
    }
}

/// Represents a collection of planet positions at a specific moment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChartPositions {
    pub positions: HashMap<Planet, PlanetPosition>,
    pub datetime: chrono::DateTime<chrono::Utc>,
    pub location: (f64, f64), // (latitude, longitude)
}

impl ChartPositions {
    pub fn new(datetime: chrono::DateTime<chrono::Utc>, location: (f64, f64)) -> Self {
        Self {
            positions: HashMap::new(),
            datetime,
            location,
        }
    }

    pub fn add_position(&mut self, position: PlanetPosition) {
        self.positions.insert(position.planet, position);
    }

    pub fn get_position(&self, planet: &Planet) -> Option<&PlanetPosition> {
        self.positions.get(planet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_planet_symbols() {
        // Verify all planets have valid symbols
        for planet in Planet::iter() {
            assert!(!planet.symbol().is_empty());
        }
    }

    #[test]
    fn test_planet_colors() {
        // Verify all planets have valid colors
        for planet in Planet::iter() {
            assert!(!planet.color().is_empty());
        }
    }

    #[test]
    fn test_planet_position_normalization() {
        let pos = PlanetPosition::new(Planet::Sun, 370.0, 95.0, 1.0, 1.0);
        assert_eq!(pos.longitude, 10.0);
        assert_eq!(pos.latitude, 90.0);
    }
}
