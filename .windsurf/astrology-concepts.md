# Astrology Concepts

## Core Concepts

### 1. Planets and Points
- **Personal Planets**: Sun, Moon, Mercury, Venus, Mars
- **Social Planets**: Jupiter, Saturn
- **Outer Planets**: Uranus, Neptune, Pluto
- **Lunar Nodes**: North Node, South Node
- **Chart Angles**: Ascendant, Midheaven, etc.

### 2. Zodiac Signs
- 12 signs with unique characteristics
- Elemental groups (Fire, Earth, Air, Water)
- Modalities (Cardinal, Fixed, Mutable)

### 3. Houses
- 12 houses representing life areas
- House systems supported:
  - Placidus
  - Koch
  - Equal House
  - Whole Sign

### 4. Aspects
- Major aspects (Conjunction, Sextile, Square, Trine, Opposition)
- Minor aspects (Semi-sextile, Quincunx, etc.)
- Orbs and exactitude

## Calculations

### 1. Planetary Positions
- Geocentric vs. heliocentric
- True/Mean node options
- Sidereal/Tropical zodiac

### 2. House Systems
- Calculation methods
- Interpolation for accuracy
- Polar region handling

### 3. Transits and Progressions
- Secondary progressions
- Solar returns
- Transits to natal chart

## Implementation Notes

### Data Structures
```rust
pub struct PlanetPosition {
    pub planet: Planet,
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed: f64,
}

pub struct Aspect {
    pub planet1: Planet,
    pub planet2: Planet,
    pub aspect_type: AspectType,
    pub orb: f64,
    pub exact_degree: f64,
}
```

### Performance Considerations
- Cache calculations where possible
- Use fixed-point arithmetic for angles
- Batch calculations for multiple charts

## References
- Swiss Ephemeris documentation
- Astronomical Algorithms by Jean Meeus
- Professional Astrology Software API References
