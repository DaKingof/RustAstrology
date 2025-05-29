use qmetaobject::*;
use qmetaobject::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::TryFrom;
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::astrology::models::planet::Planet;
use crate::astrology::uranian::dial::UranianDial;
use crate::utils::angle::Angle;
use crate::utils::ephemeris::Ephemeris;

/// Represents a celestial body's position and attributes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CelestialBody {
    pub name: String,
    pub symbol: String,
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed: f64,
    pub color: String,
    pub is_retrograde: bool,
    pub is_dignified: bool,
    pub house: Option<u8>,
}

impl Default for CelestialBody {
    fn default() -> Self {
        Self {
            name: String::new(),
            symbol: String::new(),
            longitude: 0.0,
            latitude: 0.0,
            distance: 0.0,
            speed: 0.0,
            color: "#FFFFFF".to_string(),
            is_retrograde: false,
            is_dignified: false,
            house: None,
        }
    }
}

/// Represents a celestial body in the QML interface
#[derive(Default, QObject)]
pub struct CelestialBodyViewModel {
    base: qt_base_class!(trait QObject),
    
    // Properties exposed to QML
    name: qt_property!(QString; NOTIFY data_changed),
    symbol: qt_property!(QString; NOTIFY data_changed),
    longitude: qt_property!(f64; NOTIFY data_changed),
    latitude: qt_property!(f64; NOTIFY data_changed),
    distance: qt_property!(f64; NOTIFY data_changed),
    speed: qt_property!(f64; NOTIFY data_changed),
    color: qt_property!(QString; NOTIFY data_changed),
    is_retrograde: qt_property!(bool; NOTIFY data_changed),
    is_dignified: qt_property!(bool; NOTIFY data_changed),
    house: qt_property!(i32; NOTIFY data_changed), // -1 for none/unknown
    display_position: qt_property!(f64; NOTIFY data_changed), // Position for display (considering harmonics)
    
    // Signal emitted when any property changes
    data_changed: qt_signal!(),
}

impl CelestialBodyViewModel {
    pub fn from_planet(planet: &Planet, body: &CelestialBody) -> Self {
        let mut obj = Self::default();
        obj.update_from_planet(planet, body);
        obj
    }
    
    pub fn update_from_planet(&mut self, planet: &Planet, body: &CelestialBody) {
        self.name = planet.to_string().into();
        self.symbol = planet.symbol().into();
        self.longitude = body.longitude;
        self.latitude = body.latitude;
        self.distance = body.distance;
        self.speed = body.speed;
        self.color = planet.color().into();
        self.is_retrograde = body.is_retrograde;
        self.is_dignified = body.is_dignified;
        self.house = body.house.map(|h| h as i32).unwrap_or(-1);
        self.display_position = body.longitude; // Will be updated by harmonic calculation
        
        self.data_changed();
    }
    
    pub fn update_position(&mut self, body: &CelestialBody) {
        self.longitude = body.longitude;
        self.latitude = body.latitude;
        self.distance = body.distance;
        self.speed = body.speed;
        self.is_retrograde = body.is_retrograde;
        self.is_dignified = body.is_dignified;
        self.house = body.house.map(|h| h as i32).unwrap_or(-1);
        self.display_position = body.longitude; // Will be updated by harmonic calculation
        
        self.data_changed();
    }
    
    pub fn update_display_position(&mut self, position: f64) {
        self.display_position = position;
        self.data_changed();
    }
}

/// Controller that connects the Rust UranianDial model to the QML interface
#[derive(Default, QObject)]
pub struct DialController {
    base: qt_base_class!(trait QObject),
    
    // The underlying dial model
    dial: Arc<Mutex<UranianDial>>,
    
    // Ephemeris for astronomical calculations
    ephemeris: Arc<Mutex<Option<Ephemeris>>>,
    
    // Properties exposed to QML
    celestial_bodies: qt_property!(QVariantList; NOTIFY data_changed),
    rotation: qt_property!(f64; NOTIFY view_changed),
    zoom: qt_property!(f64; NOTIFY view_changed),
    harmonic: qt_property!(u32; NOTIFY harmonic_changed),
    current_time: qt_property!(QString; NOTIFY time_changed),
    location_latitude: qt_property!(f64; NOTIFY location_changed),
    location_longitude: qt_property!(f64; NOTIFY location_changed),
    
    // View models for celestial bodies
    celestial_body_models: HashMap<Planet, QObjectPinned<CelestialBodyViewModel>>,
    
    // State
    is_initialized: bool,
    
    // Signals
    data_changed: qt_signal!(),
    view_changed: qt_signal!(),
    harmonic_changed: qt_signal!(),
    time_changed: qt_signal!(),
    location_changed: qt_signal!(),
    
    // Methods exposed to QML
    initialize: qt_method!(fn(&mut self, datetime: QString, lat: f64, lng: f64) -> bool),
    update_time: qt_method!(fn(&mut self, datetime: QString) -> bool),
    update_location: qt_method!(fn(&mut self, lat: f64, lng: f64) -> bool),
    rotate_by: qt_method!(fn(&mut self, degrees: f64)),
    set_rotation: qt_method!(fn(&mut self, degrees: f64)),
    zoom_by: qt_method!(fn(&mut self, factor: f64) -> f64),
    set_zoom: qt_method!(fn(&mut self, zoom: f64) -> f64),
    set_harmonic: qt_method!(fn(&mut self, harmonic: u32)),
    get_planet_positions: qt_method!(fn(&self) -> QVariantMap),
    calculate_aspects: qt_method!(fn(&self, planet1: QString, planet2: QString) -> QVariantMap),
    calculate_midpoints: qt_method!(fn(&self) -> QVariantList),
    get_planet_info: qt_method!(fn(&self, planet: QString) -> QVariantMap),
}

impl DialController {
    pub fn new() -> Self {
        let mut obj = Self::default();
        obj.dial = Arc::new(Mutex::new(UranianDial::new()));
        obj.ephemeris = Arc::new(Mutex::new(None));
        obj.rotation = 0.0;
        obj.zoom = 1.0;
        obj.harmonic = 1;
        obj.celestial_body_models = HashMap::new();
        obj.celestial_bodies = QVariantList::default();
        obj.is_initialized = false;
        obj.location_latitude = 0.0;
        obj.location_longitude = 0.0;
        obj.current_time = "".into();
        obj
    }
    
    // Implementation of QML-exposed methods
    
    /// Initialize the dial controller with the given datetime and location
    pub fn initialize(&mut self, datetime: QString, lat: f64, lng: f64) -> bool {
        // Parse the datetime string (expected format: "YYYY-MM-DD HH:MM:SS")
        let datetime_str = datetime.to_string();
        let dt = match DateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => dt,
            Err(_) => return false,
        };
        
        // Update location
        self.location_latitude = lat;
        self.location_longitude = lng;
        
        // Create ephemeris for the given time and location
        match Ephemeris::new(dt.with_timezone(&Utc), lat, lng) {
            Ok(ephem) => {
                *self.ephemeris.lock().unwrap() = Some(ephem);
                self.current_time = datetime;
                self.is_initialized = true;
                self.update_celestial_bodies();
                self.time_changed();
                self.location_changed();
                true
            }
            Err(_) => false,
        }
    }
    
    fn update_time(&mut self, datetime: QString) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        let datetime_str = datetime.to_string();
        let dt = match DateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => dt,
            Err(_) => return false,
        };
        
        if let Some(ephem) = self.ephemeris.lock().unwrap().as_mut() {
            if ephem.update_time(dt.with_timezone(&Utc)).is_ok() {
                self.current_time = datetime;
                self.update_celestial_bodies();
                self.time_changed();
                return true;
            }
        }
        false
    }
    
    fn update_location(&mut self, lat: f64, lng: f64) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        self.location_latitude = lat;
        self.location_longitude = lng;
        
        if let Some(ephem) = self.ephemeris.lock().unwrap().as_mut() {
            if ephem.update_location(lat, lng).is_ok() {
                self.update_celestial_bodies();
                self.location_changed();
                return true;
            }
        }
        false
    }
    
    fn rotate_by(&mut self, degrees: f64) {
        let mut dial = self.dial.lock().unwrap();
        dial.rotate_by(degrees);
        self.rotation = dial.rotation;
        self.view_changed();
    }
    
    fn set_rotation(&mut self, degrees: f64) {
        let mut dial = self.dial.lock().unwrap();
        dial.set_rotation(degrees);
        self.rotation = dial.rotation;
        self.view_changed();
    }
    
    fn zoom_by(&mut self, factor: f64) -> f64 {
        let mut dial = self.dial.lock().unwrap();
        let zoom = dial.zoom_by(factor);
        self.zoom = zoom;
        self.view_changed();
        zoom
    }
    
    fn set_zoom(&mut self, zoom: f64) -> f64 {
        let new_zoom = self.dial.set_zoom(zoom);
        self.zoom = new_zoom;
        self.view_changed();
        new_zoom
    }
    
    fn set_harmonic(&mut self, harmonic: u32) {
        if harmonic >= 1 && harmonic <= 90 {
            self.harmonic = harmonic;
            self.dial.lock().unwrap().set_harmonic(harmonic);
            self.harmonic_changed();
            self.update_celestial_bodies();
        }
    }
    
    fn get_planet_positions(&self) -> QVariantMap {
        let mut result = QVariantMap::default();
        
        if let Some(ephem) = self.ephemeris.lock().unwrap().as_ref() {
            for (planet, body) in self.celestial_body_models.iter() {
                let body = body.borrow();
                let mut pos_map = QVariantMap::default();
                pos_map.insert("longitude".into(), body.longitude.into());
                pos_map.insert("latitude".into(), body.latitude.into());
                pos_map.insert("display_position".into(), body.display_position.into());
                pos_map.insert("is_retrograde".into(), body.is_retrograde.into());
                result.insert(planet.to_string().into(), pos_map.into());
            }
        }
        
        result
    }
    
    fn calculate_aspects(&self, planet1: QString, planet2: QString) -> QVariantMap {
        let mut result = QVariantMap::default();
        
        if let Some(ephem) = self.ephemeris.lock().unwrap().as_ref() {
            // Get planet positions and calculate aspects
            // This is a simplified version - actual implementation would use the ephemeris
            // to calculate precise aspects between the two planets
            
            let planet1_name = planet1.to_string();
            let planet2_name = planet2.to_string();
            
            // For demonstration, return a mock aspect
            result.insert("type".into(), "trine".into());
            result.insert("orb".into(), 2.5.into());
            result.insert("exact".into(), false.into());
            result.insert("planet1".into(), planet1_name.into());
            result.insert("planet2".into(), planet2_name.into());
        }
        
        result
    }
    
    fn calculate_midpoints(&self) -> QVariantList {
        let mut midpoints = QVariantList::default();
        
        if let Some(_ephem) = self.ephemeris.lock().unwrap().as_ref() {
            // Calculate midpoints between all planet pairs
            let planets: Vec<&Planet> = self.celestial_body_models.keys().collect();
            
            for i in 0..planets.len() {
                for j in (i + 1)..planets.len() {
                    let p1 = planets[i];
                    let p2 = planets[j];
                    
                    if let (Some(b1), Some(b2)) = (
                        self.celestial_body_models.get(p1),
                        self.celestial_body_models.get(p2)
                    ) {
                        let b1 = b1.borrow();
                        let b2 = b2.borrow();
                        
                        // Calculate midpoint (simplified)
                        let midpoint = (b1.longitude + b2.longitude) / 2.0 % 360.0;
                        
                        let mut mp_map = QVariantMap::default();
                        mp_map.insert("planet1".to_string(), p1.to_string().into());
                        mp_map.insert("planet2".to_string(), p2.to_string().into());
                        mp_map.insert("position".to_string(), midpoint.into());
                        
                        midpoints.push(mp_map.into());
                    }
                }
            }
        }
        
        midpoints
    }
    
    fn remove_planet(&mut self, planet: QString) -> bool {
        if let Ok(planet_enum) = planet.to_string().parse::<Planet>() {
            // Remove the planet from the dial model
            if let Ok(mut dial) = self.dial.lock() {
                dial.remove_planet(&planet_enum);
            }
            
            // Remove from the view models and update
            self.celestial_body_models.remove(&planet_enum);
            self.update_celestial_bodies();
            true
        } else {
            false
        }
    }
    
    fn get_planet_info(&self, planet: QString) -> QVariantMap {
        let mut result = QVariantMap::default();
        
        if let Ok(_ephem) = self.ephemeris.lock() {
            let planet_name = planet.to_string();
            
            // Find the planet in our models
            for (p, body) in &self.celestial_body_models {
                if p.to_string() == planet_name {
                    let body = body.borrow();
                    
                    result.insert("name".to_string(), body.name.clone().into());
                    result.insert("symbol".to_string(), body.symbol.clone().into());
                    result.insert("longitude".to_string(), body.longitude.into());
                    result.insert("latitude".to_string(), body.latitude.into());
                    result.insert("distance".to_string(), body.distance.into());
                    result.insert("speed".to_string(), body.speed.into());
                    result.insert("is_retrograde".to_string(), body.is_retrograde.into());
                    result.insert("is_dignified".to_string(), body.is_dignified.into());
                    result.insert("house".to_string(), body.house.map(|h| h as i32).unwrap_or(-1).into());
                    
                    break;
                }
            }
        }
        
        result
    }
    
    // Helper methods
    
    fn update_celestial_bodies(&mut self) {
        if !self.is_initialized {
            return;
        }
        
        let mut celestial_bodies = QVariantList::default();
        
        if let Some(ephem) = self.ephemeris.lock().unwrap().as_ref() {
            // Update positions for all known planets
            for planet in Planet::iter() {
                // Get position from ephemeris (simplified)
                let body = match ephem.get_planet_position(&planet) {
                    Ok(pos) => pos,
                    Err(_) => continue,
                };
                
                // Update or create the view model
                if let Some(vm) = self.celestial_body_models.get_mut(&planet) {
                    vm.borrow_mut().update_position(&body);
                } else {
                    let vm = CelestialBodyViewModel::from_planet(&planet, &body);
                    let obj = QObjectPinned::new(vm);
                    self.celestial_body_models.insert(planet, obj);
                }
                
                // Create a QVariantMap for the celestial body
                let mut body_map = QVariantMap::default();
                
                // Add the planet name as a string
                body_map.insert("name".to_string(), planet.to_string().into());
                
                // Add position data
                body_map.insert("longitude".to_string(), body.longitude.into());
                body_map.insert("latitude".to_string(), body.latitude.into());
                body_map.insert("distance".to_string(), body.distance.into());
                body_map.insert("is_retrograde".to_string(), body.is_retrograde.into());
                body_map.insert("is_dignified".to_string(), body.is_dignified.into());
                body_map.insert("house".to_string(), body.house.map(|h| h as i32).unwrap_or(-1).into());
                
                // Add the body map to the list
                celestial_bodies.push(body_map.into());
            }
            
            // Update the QML property
            self.celestial_bodies = celestial_bodies;
            self.data_changed();
        }
    }
}

// Register the QML types
pub fn register_qml_types() {
    // Convert string literals to CString
    let uri = CString::new("RustAstrology").unwrap();
    let dial_controller_name = CString::new("DialController").unwrap();
    let celestial_body_name = CString::new("CelestialBodyViewModel").unwrap();
    
    // Register the main controller
    unsafe {
        qmetaobject::qml_register_type::<DialController>(
            &uri,
            1,
            0,
            &dial_controller_name
        );
        
        // Register the celestial body view model
        qmetaobject::qml_register_type::<CelestialBodyViewModel>(
            &uri,
            1,
            0,
            &celestial_body_name
        );
    }
}
