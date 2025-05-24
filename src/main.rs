use anyhow::Result;
use astro::AstrologyCalculator;

mod astro;

fn main() -> Result<()> {
    // Initialize the calculator with default ephemeris path
    let calculator = AstrologyCalculator::new(None)?;
    
    // Calculate sun position for May 24, 2024 at noon UTC
    let result = calculator.calculate_sun_position(2458993.0)?;
    println!("Sun position (longitude, latitude): {:?}", result);
    
    // Print formatted zodiac position
    let (longitude, _) = result;
    let zodiac_sign = match (longitude as i32) / 30 {
        0 => "Aries",
        1 => "Taurus",
        2 => "Gemini",
        3 => "Cancer",
        4 => "Leo",
        5 => "Virgo",
        6 => "Libra",
        7 => "Scorpio",
        8 => "Sagittarius",
        9 => "Capricorn",
        10 => "Aquarius",
        11 => "Pisces",
        _ => "Unknown"
    };
    
    println!("The Sun is in {} at {:.2}°", zodiac_sign, longitude % 30.0);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zodiac_calculation() -> Result<()> {
        let calculator = AstrologyCalculator::new(None)?;
        let (longitude, latitude) = calculator.calculate_sun_position(2458993.0)?;
        
        assert!(longitude >= 0.0 && longitude < 360.0);
        assert!(latitude >= -90.0 && latitude <= 90.0);
        
        Ok(())
    }
}