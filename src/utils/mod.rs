//! Utility functions and types for the Uranian Astrology application

pub mod angle;

// Re-export commonly used types
pub use angle::{Angle, normalize_degrees, radians_to_degrees, degrees_to_radians, angle_difference};
