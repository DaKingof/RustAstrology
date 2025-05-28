// Harmonic types and related functionality
use std::fmt;

/// Represents different harmonic divisions of the 360° circle
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HarmonicType {
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6,
    Seventh = 7,
    Eighth = 8,
    Ninth = 9,
    Tenth = 10,
    Eleventh = 11,
    Twelfth = 12,
    Sixteenth = 16,
}

impl HarmonicType {
    /// Returns the numeric value of the harmonic
    pub fn value(&self) -> u32 {
        *self as u32
    }

    /// Returns a formatted display name for the harmonic (e.g., "2nd", "3rd")
    pub fn display_name(&self) -> String {
        match self {
            Self::Second => "2nd".to_string(),
            Self::Third => "3rd".to_string(),
            Self::Fourth => "4th".to_string(),
            Self::Fifth => "5th".to_string(),
            Self::Sixth => "6th".to_string(),
            Self::Seventh => "7th".to_string(),
            Self::Eighth => "8th".to_string(),
            Self::Ninth => "9th".to_string(),
            Self::Tenth => "10th".to_string(),
            Self::Eleventh => "11th".to_string(),
            Self::Twelfth => "12th".to_string(),
            Self::Sixteenth => "16th".to_string(),
        }
    }

    /// Returns the range of degrees for this harmonic (360° / harmonic value)
    pub fn harmonic_range(&self) -> f64 {
        360.0 / self.value() as f64
    }
    
    /// Returns a list of all available harmonic types
    pub fn all_types() -> Vec<HarmonicType> {
        vec![
            Self::Second,
            Self::Third,
            Self::Fourth,
            Self::Fifth,
            Self::Sixth,
            Self::Seventh,
            Self::Eighth,
            Self::Ninth,
            Self::Tenth,
            Self::Eleventh,
            Self::Twelfth,
            Self::Sixteenth,
        ]
    }
}

impl fmt::Display for HarmonicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Calculate a harmonic midpoint between two positions
pub fn calculate_harmonic_midpoint(pos1: f64, pos2: f64, harmonic_base: f64) -> f64 {
    let midpoint_360 = super::planet::calculate_360_midpoint(pos1, pos2);
    let harmonic_range = 360.0 / harmonic_base;
    midpoint_360 % harmonic_range
}
