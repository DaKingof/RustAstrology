use std::f64::consts::PI;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use serde::{Serialize, Deserialize};

/// Represents an angle in degrees with utility methods for astrological calculations
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Angle(f64);

impl Angle {
    /// Create a new angle from degrees
    pub fn from_degrees(degrees: f64) -> Self {
        Self(normalize_degrees(degrees))
    }
    
    /// Create a new angle from radians
    pub fn from_radians(radians: f64) -> Self {
        Self::from_degrees(radians.to_degrees())
    }
    
    /// Get the angle in degrees (0-360)
    pub fn degrees(&self) -> f64 {
        self.0
    }
    
    /// Get the angle in radians (0-2π)
    pub fn radians(&self) -> f64 {
        self.0.to_radians()
    }
    
    /// Get the sine of the angle
    pub fn sin(&self) -> f64 {
        self.radians().sin()
    }
    
    /// Get the cosine of the angle
    pub fn cos(&self) -> f64 {
        self.radians().cos()
    }
    
    /// Get the tangent of the angle
    pub fn tan(&self) -> f64 {
        self.radians().tan()
    }
    
    /// Calculate the shortest distance (in degrees) between two angles
    pub fn distance_to(&self, other: Angle) -> Angle {
        let diff = (other.0 - self.0).abs();
        Angle::from_degrees(if diff > 180.0 { 360.0 - diff } else { diff })
    }
    
    /// Check if this angle is within a given orb of another angle
    pub fn within_orb(&self, other: Angle, orb: f64) -> bool {
        self.distance_to(other).degrees() <= orb
    }
    
    /// Apply a harmonic to the angle (e.g., for harmonic dials)
    pub fn harmonic(&self, harmonic: u32) -> Angle {
        Angle::from_degrees((self.0 * harmonic as f64) % 360.0)
    }
    
    /// Get the opposite point (180° away)
    pub fn opposite(&self) -> Angle {
        Angle::from_degrees((self.0 + 180.0) % 360.0)
    }
    
    /// Calculate the midpoint between two angles
    pub fn midpoint(&self, other: Angle) -> Angle {
        let diff = (other.0 - self.0).abs();
        let sum = if diff > 180.0 {
            self.0 + other.0 + 360.0
        } else {
            self.0 + other.0
        };
        Angle::from_degrees(sum / 2.0)
    }
    
    /// Get the zodiac sign this angle falls in
    pub fn zodiac_sign(&self) -> (crate::astrology::models::zodiac::ZodiacSign, f64) {
        crate::astrology::models::zodiac::ZodiacSign::from_degree(self.0)
    }
    
    /// Format the angle as degrees, minutes, and seconds
    pub fn to_dms(&self) -> (i32, u32, f64) {
        let degrees = self.0 as i32;
        let minutes_fract = (self.0 - degrees as f64).abs() * 60.0;
        let minutes = minutes_fract as u32;
        let seconds = (minutes_fract - minutes as f64) * 60.0;
        
        (degrees, minutes, seconds)
    }
    
    /// Format the angle as a string in DMS format (e.g., "15°27'43.2\" Aries")
    pub fn to_dms_string(&self) -> String {
        let (degrees, minutes, seconds) = self.to_dms();
        let (sign, _) = self.zodiac_sign();
        
        format!(
            "{}°{:02}'{:04.1}\" {}",
            degrees.abs(),
            minutes,
            seconds,
            sign
        )
    }
}

// Implement basic arithmetic operations for Angle
impl Add for Angle {
    type Output = Angle;
    
    fn add(self, other: Angle) -> Angle {
        Angle::from_degrees(self.0 + other.0)
    }
}

impl Sub for Angle {
    type Output = Angle;
    
    fn sub(self, other: Angle) -> Angle {
        Angle::from_degrees(self.0 - other.0)
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;
    
    fn mul(self, scalar: f64) -> Angle {
        Angle::from_degrees(self.0 * scalar)
    }
}

impl Div<f64> for Angle {
    type Output = Angle;
    
    fn div(self, scalar: f64) -> Angle {
        Angle::from_degrees(self.0 / scalar)
    }
}

// Implement assignment operators
impl AddAssign for Angle {
    fn add_assign(&mut self, other: Angle) {
        *self = Angle::from_degrees(self.0 + other.0);
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Angle) {
        *self = Angle::from_degrees(self.0 - other.0);
    }
}

impl MulAssign<f64> for Angle {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Angle::from_degrees(self.0 * scalar);
    }
}

impl DivAssign<f64> for Angle {
    fn div_assign(&mut self, scalar: f64) {
        *self = Angle::from_degrees(self.0 / scalar);
    }
}

/// Normalize an angle to the range [0, 360)
pub fn normalize_degrees(degrees: f64) -> f64 {
    let normalized = degrees % 360.0;
    if normalized < 0.0 {
        normalized + 360.0
    } else {
        normalized
    }
}

/// Convert radians to degrees and normalize to [0, 360)
pub fn radians_to_degrees(radians: f64) -> f64 {
    normalize_degrees(radians.to_degrees())
}

/// Convert degrees to radians
pub fn degrees_to_radians(degrees: f64) -> f64 {
    normalize_degrees(degrees).to_radians()
}

/// Calculate the difference between two angles (signed, in degrees)
pub fn angle_difference(a: f64, b: f64) -> f64 {
    let diff = (b - a) % 360.0;
    if diff < -180.0 {
        diff + 360.0
    } else if diff > 180.0 {
        diff - 360.0
    } else {
        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use crate::astrology::models::zodiac::ZodiacSign;

    #[test]
    fn test_angle_creation() {
        let angle = Angle::from_degrees(45.0);
        assert!(approx_eq!(f64, angle.degrees(), 45.0, epsilon = 0.0001));
        
        let angle = Angle::from_radians(PI);
        assert!(approx_eq!(f64, angle.degrees(), 180.0, epsilon = 0.0001));
    }

    #[test]
    fn test_angle_normalization() {
        let angle = Angle::from_degrees(370.0);
        assert!(approx_eq!(f64, angle.degrees(), 10.0, epsilon = 0.0001));
        
        let angle = Angle::from_degrees(-10.0);
        assert!(approx_eq!(f64, angle.degrees(), 350.0, epsilon = 0.0001));
    }

    #[test]
    fn test_angle_distance() {
        let a = Angle::from_degrees(10.0);
        let b = Angle::from_degrees(20.0);
        assert!(approx_eq!(f64, a.distance_to(b).degrees(), 10.0, epsilon = 0.0001));
        
        let a = Angle::from_degrees(350.0);
        let b = Angle::from_degrees(10.0);
        assert!(approx_eq!(f64, a.distance_to(b).degrees(), 20.0, epsilon = 0.0001));
    }

    #[test]
    fn test_angle_harmonic() {
        let angle = Angle::from_degrees(30.0);
        assert!(approx_eq!(f64, angle.harmonic(2).degrees(), 60.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, angle.harmonic(4).degrees(), 120.0, epsilon = 0.0001));
        
        let angle = Angle::from_degrees(100.0);
        assert!(approx_eq!(f64, angle.harmonic(3).degrees(), 300.0, epsilon = 0.0001));
    }

    #[test]
    fn test_angle_midpoint() {
        let a = Angle::from_degrees(10.0);
        let b = Angle::from_degrees(30.0);
        assert!(approx_eq!(f64, a.midpoint(b).degrees(), 20.0, epsilon = 0.0001));
        
        let a = Angle::from_degrees(350.0);
        let b = Angle::from_degrees(10.0);
        assert!(approx_eq!(f64, a.midpoint(b).degrees(), 0.0, epsilon = 0.0001));
    }

    #[test]
    fn test_angle_zodiac() {
        let angle = Angle::from_degrees(0.0);
        let (sign, _) = angle.zodiac_sign();
        assert_eq!(sign, ZodiacSign::Aries);
        
        let angle = Angle::from_degrees(30.0);
        let (sign, _) = angle.zodiac_sign();
        assert_eq!(sign, ZodiacSign::Taurus);
        
        let angle = Angle::from_degrees(359.9);
        let (sign, _) = angle.zodiac_sign();
        assert_eq!(sign, ZodiacSign::Pisces);
    }
}
