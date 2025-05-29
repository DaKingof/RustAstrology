use serde::{Serialize, Deserialize};
use std::fmt;

/// Represents the 12 zodiac signs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZodiacSign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl ZodiacSign {
    /// Get the starting degree of the sign (0° Aries, 30° Taurus, etc.)
    pub fn start_degree(&self) -> f64 {
        match self {
            ZodiacSign::Aries => 0.0,
            ZodiacSign::Taurus => 30.0,
            ZodiacSign::Gemini => 60.0,
            ZodiacSign::Cancer => 90.0,
            ZodiacSign::Leo => 120.0,
            ZodiacSign::Virgo => 150.0,
            ZodiacSign::Libra => 180.0,
            ZodiacSign::Scorpio => 210.0,
            ZodiacSign::Sagittarius => 240.0,
            ZodiacSign::Capricorn => 270.0,
            ZodiacSign::Aquarius => 300.0,
            ZodiacSign::Pisces => 330.0,
        }
    }

    /// Get the symbol for the zodiac sign
    pub fn symbol(&self) -> &'static str {
        match self {
            ZodiacSign::Aries => "♈",
            ZodiacSign::Taurus => "♉",
            ZodiacSign::Gemini => "♊",
            ZodiacSign::Cancer => "♋",
            ZodiacSign::Leo => "♌",
            ZodiacSign::Virgo => "♍",
            ZodiacSign::Libra => "♎",
            ZodiacSign::Scorpio => "♏",
            ZodiacSign::Sagittarius => "♐",
            ZodiacSign::Capricorn => "♑",
            ZodiacSign::Aquarius => "♒",
            ZodiacSign::Pisces => "♓",
        }
    }

    /// Get the element of the zodiac sign
    pub fn element(&self) -> Element {
        match self {
            ZodiacSign::Aries => Element::Fire,
            ZodiacSign::Taurus => Element::Earth,
            ZodiacSign::Gemini => Element::Air,
            ZodiacSign::Cancer => Element::Water,
            ZodiacSign::Leo => Element::Fire,
            ZodiacSign::Virgo => Element::Earth,
            ZodiacSign::Libra => Element::Air,
            ZodiacSign::Scorpio => Element::Water,
            ZodiacSign::Sagittarius => Element::Fire,
            ZodiacSign::Capricorn => Element::Earth,
            ZodiacSign::Aquarius => Element::Air,
            ZodiacSign::Pisces => Element::Water,
        }
    }

    /// Get the modality of the zodiac sign
    pub fn modality(&self) -> Modality {
        match self {
            ZodiacSign::Aries => Modality::Cardinal,
            ZodiacSign::Taurus => Modality::Fixed,
            ZodiacSign::Gemini => Modality::Mutable,
            ZodiacSign::Cancer => Modality::Cardinal,
            ZodiacSign::Leo => Modality::Fixed,
            ZodiacSign::Virgo => Modality::Mutable,
            ZodiacSign::Libra => Modality::Cardinal,
            ZodiacSign::Scorpio => Modality::Fixed,
            ZodiacSign::Sagittarius => Modality::Mutable,
            ZodiacSign::Capricorn => Modality::Cardinal,
            ZodiacSign::Aquarius => Modality::Fixed,
            ZodiacSign::Pisces => Modality::Mutable,
        }
    }

    /// Get the zodiac sign for a given degree (0-360)
    pub fn from_degree(degree: f64) -> (Self, f64) {
        let normalized_degree = degree % 360.0;
        let sign_degree = normalized_degree % 30.0;
        let sign_idx = (normalized_degree / 30.0) as usize;
        
        let sign = match sign_idx {
            0 => ZodiacSign::Aries,
            1 => ZodiacSign::Taurus,
            2 => ZodiacSign::Gemini,
            3 => ZodiacSign::Cancer,
            4 => ZodiacSign::Leo,
            5 => ZodiacSign::Virgo,
            6 => ZodiacSign::Libra,
            7 => ZodiacSign::Scorpio,
            8 => ZodiacSign::Sagittarius,
            9 => ZodiacSign::Capricorn,
            10 => ZodiacSign::Aquarius,
            11 => ZodiacSign::Pisces,
            _ => unreachable!(),
        };
        
        (sign, sign_degree)
    }
}

impl fmt::Display for ZodiacSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ZodiacSign::Aries => "Aries",
            ZodiacSign::Taurus => "Taurus",
            ZodiacSign::Gemini => "Gemini",
            ZodiacSign::Cancer => "Cancer",
            ZodiacSign::Leo => "Leo",
            ZodiacSign::Virgo => "Virgo",
            ZodiacSign::Libra => "Libra",
            ZodiacSign::Scorpio => "Scorpio",
            ZodiacSign::Sagittarius => "Sagittarius",
            ZodiacSign::Capricorn => "Capricorn",
            ZodiacSign::Aquarius => "Aquarius",
            ZodiacSign::Pisces => "Pisces",
        };
        write!(f, "{}", name)
    }
}

/// Represents the four classical elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Earth,
    Air,
    Water,
}

impl Element {
    pub fn symbol(&self) -> &'static str {
        match self {
            Element::Fire => "🜂",
            Element::Earth => "🜃",
            Element::Air => "🜁",
            Element::Water => "🜄",
        }
    }
}

/// Represents the three modalities (qualities) of the signs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modality {
    Cardinal,
    Fixed,
    Mutable,
}

impl Modality {
    pub fn symbol(&self) -> &'static str {
        match self {
            Modality::Cardinal => "⯯",
            Modality::Fixed => "⯱",
            Modality::Mutable => "⯰",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_zodiac_sign_from_degree() {
        let (sign, degree) = ZodiacSign::from_degree(0.0);
        assert_eq!(sign, ZodiacSign::Aries);
        assert!(approx_eq!(f64, degree, 0.0, epsilon = 0.0001));

        let (sign, degree) = ZodiacSign::from_degree(30.0);
        assert_eq!(sign, ZodiacSign::Taurus);
        assert!(approx_eq!(f64, degree, 0.0, epsilon = 0.0001));

        let (sign, degree) = ZodiacSign::from_degree(45.0);
        assert_eq!(sign, ZodiacSign::Taurus);
        assert!(approx_eq!(f64, degree, 15.0, epsilon = 0.0001));

        let (sign, degree) = ZodiacSign::from_degree(360.0);
        assert_eq!(sign, ZodiacSign::Aries);
        assert!(approx_eq!(f64, degree, 0.0, epsilon = 0.0001));
    }

    #[test]
    fn test_zodiac_sign_properties() {
        assert_eq!(ZodiacSign::Aries.element(), Element::Fire);
        assert_eq!(ZodiacSign::Taurus.element(), Element::Earth);
        assert_eq!(ZodiacSign::Gemini.element(), Element::Air);
        assert_eq!(ZodiacSign::Cancer.element(), Element::Water);

        assert_eq!(ZodiacSign::Aries.modality(), Modality::Cardinal);
        assert_eq!(ZodiacSign::Taurus.modality(), Modality::Fixed);
        assert_eq!(ZodiacSign::Gemini.modality(), Modality::Mutable);
        assert_eq!(ZodiacSign::Cancer.modality(), Modality::Cardinal);
    }
}
