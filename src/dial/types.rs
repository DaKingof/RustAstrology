use serde::{Deserialize, Serialize};
use nalgebra::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Planet {
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    NorthNode,
    SouthNode,
    Chiron,
    Ascendant,
    Midheaven,
}

impl Planet {
    pub fn glyph(&self) -> &'static str {
        match self {
            Planet::Sun => "☉",
            Planet::Moon => "☽",
            Planet::Mercury => "☿",
            Planet::Venus => "♀",
            Planet::Mars => "♂",
            Planet::Jupiter => "♃",
            Planet::Saturn => "♄",
            Planet::Uranus => "♅",
            Planet::Neptune => "♆",
            Planet::Pluto => "♇",
            Planet::NorthNode => "☊",
            Planet::SouthNode => "☋",
            Planet::Chiron => "⚷",
            Planet::Ascendant => "AC",
            Planet::Midheaven => "MC",
        }
    }

    pub fn all() -> Vec<Planet> {
        vec![
            Planet::Sun,
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
            Planet::Pluto,
            Planet::NorthNode,
            Planet::Chiron,
            Planet::Ascendant,
            Planet::Midheaven,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlanetPosition {
    pub planet: Planet,
    pub longitude: f64,
    pub latitude: f64,
    pub speed: f64,
}

impl PlanetPosition {
    pub fn new(planet: Planet, longitude: f64, latitude: f64, speed: f64) -> Self {
        Self {
            planet,
            longitude,
            latitude,
            speed,
        }
    }

    /// Convert to 90° dial position (longitude mod 90)
    pub fn dial_position(&self) -> f64 {
        self.longitude % 90.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Midpoint {
    pub planet1: Planet,
    pub planet2: Planet,
    pub position: f64,
    pub dial_position: f64,
}

impl Midpoint {
    pub fn new(planet1: Planet, pos1: f64, planet2: Planet, pos2: f64) -> Self {
        let position = ((pos1 + pos2) / 2.0 + 360.0) % 360.0;
        let dial_position = position % 90.0;
        
        Self {
            planet1,
            planet2,
            position,
            dial_position,
        }
    }

    pub fn label(&self) -> String {
        format!("{}/{}", self.planet1.glyph(), self.planet2.glyph())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisPoint {
    Zero = 0,
    TwentyTwoFive = 225, // 22.5 * 10 for integer representation
    FortyFive = 450,     // 45.0 * 10
    SixtySevenFive = 675, // 67.5 * 10
}

impl AxisPoint {
    pub fn degrees(&self) -> f64 {
        match self {
            AxisPoint::Zero => 0.0,
            AxisPoint::TwentyTwoFive => 22.5,
            AxisPoint::FortyFive => 45.0,
            AxisPoint::SixtySevenFive => 67.5,
        }
    }

    pub fn all() -> Vec<AxisPoint> {
        vec![
            AxisPoint::Zero,
            AxisPoint::TwentyTwoFive,
            AxisPoint::FortyFive,
            AxisPoint::SixtySevenFive,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            AxisPoint::Zero => "Cardinal (0°)",
            AxisPoint::TwentyTwoFive => "Semi-Square (22.5°)",
            AxisPoint::FortyFive => "Semi-Square (45°)",
            AxisPoint::SixtySevenFive => "Sesquisquare (67.5°)",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AxisAlignment {
    pub axis: AxisPoint,
    pub midpoint: Option<Midpoint>,
    pub planet: Option<PlanetPosition>,
    pub orb: f64,
}

impl AxisAlignment {
    pub fn new_midpoint(axis: AxisPoint, midpoint: Midpoint, orb: f64) -> Self {
        Self {
            axis,
            midpoint: Some(midpoint),
            planet: None,
            orb,
        }
    }

    pub fn new_planet(axis: AxisPoint, planet: PlanetPosition, orb: f64) -> Self {
        Self {
            axis,
            midpoint: None,
            planet: Some(planet),
            orb,
        }
    }

    pub fn label(&self) -> String {
        if let Some(midpoint) = &self.midpoint {
            format!("{} on {} (±{:.1}°)", midpoint.label(), self.axis.name(), self.orb)
        } else if let Some(planet) = &self.planet {
            format!("{} on {} (±{:.1}°)", planet.planet.glyph(), self.axis.name(), self.orb)
        } else {
            "Unknown alignment".to_string()
        }
    }
}

/// Live alignment information for current dial rotation
#[derive(Debug, Clone)]
pub struct LiveAlignment {
    pub axis_canvas_angle: f64,
    pub axis_label: String,
    pub axis_point: AxisPoint,
    pub target_dial_position: f64,
    pub aligned_midpoints: Vec<(Midpoint, f64)>,
    pub aligned_planets: Vec<(PlanetPosition, f64)>,
}

/// Cardinal points for 360° dial
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cardinal360 {
    Aries,   // 0°
    Cancer,  // 90°
    Libra,   // 180°
    Capricorn, // 270°
}

/// Live alignment information for 360° dial
#[derive(Debug, Clone)]
pub struct Live360Alignment {
    pub axis_angle: f64,
    pub axis_label: String,
    pub cardinal: Cardinal360,
    pub target_position: f64,
    pub aligned_midpoints: Vec<(Midpoint, f64)>,
    pub aligned_planets: Vec<(PlanetPosition, f64)>,
}

#[derive(Debug, Clone)]
pub struct DialState {
    pub rotation: f64,
    pub planets: Vec<PlanetPosition>,
    pub midpoints: Vec<Midpoint>,
    pub alignments: Vec<AxisAlignment>,
    pub orb_tolerance: f64,
    pub is_dragging: bool,
    pub last_mouse_pos: Option<Vector2<f64>>,
}

impl Default for DialState {
    fn default() -> Self {
        Self {
            rotation: 0.0,
            planets: Vec::new(),
            midpoints: Vec::new(),
            alignments: Vec::new(),
            orb_tolerance: 1.0, // 1 degree orb as requested
            is_dragging: false,
            last_mouse_pos: None,
        }
    }
}

impl DialState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation % 360.0;
    }

    pub fn add_rotation(&mut self, delta: f64) {
        self.rotation = (self.rotation + delta + 360.0) % 360.0;
        if self.rotation < 0.0 {
            self.rotation += 360.0;
        }
    }
}
