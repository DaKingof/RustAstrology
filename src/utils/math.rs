// Mathematical utility functions for astrological calculations
use std::f64::consts::PI;
use crate::utils::constants::{CENTER_X, CENTER_Y};

/// Converts degrees to cartesian coordinates based on a radius and center position
/// 
/// This uses the astrological convention where 0° is at the top (north),
/// and degrees increase clockwise
pub fn degrees_to_coords(degrees: f64, radius: f64) -> (f64, f64) {
    let radians = (90.0 - degrees) * PI / 180.0;
    let x = CENTER_X + radius * radians.cos();
    let y = CENTER_Y - radius * radians.sin();
    (x, y)
}

/// Normalizes an angle to the range [0, 360)
pub fn normalize_degrees(degrees: f64) -> f64 {
    ((degrees % 360.0) + 360.0) % 360.0
}

/// Calculates the shortest arc distance between two angles
pub fn angle_distance(angle1: f64, angle2: f64) -> f64 {
    let a1 = normalize_degrees(angle1);
    let a2 = normalize_degrees(angle2);
    let diff = (a1 - a2).abs();
    if diff > 180.0 {
        360.0 - diff
    } else {
        diff
    }
}

/// Determines if an angle is between two other angles along the shortest arc
pub fn is_angle_between(angle: f64, start: f64, end: f64) -> bool {
    let a = normalize_degrees(angle);
    let s = normalize_degrees(start);
    let e = normalize_degrees(end);
    
    if s < e {
        a >= s && a <= e
    } else {
        // Handles the case where the arc crosses 0°
        a >= s || a <= e
    }
}
