use anyhow::{Context, Result};
use log::{info, error};
use qmetaobject::*;
use qmetaobject::prelude::*;
use std::{ffi::CString, path::PathBuf, rc::Rc, cell::RefCell};

// Import our modules
mod astrology {
    pub mod models {
        pub mod planet;
        pub mod zodiac;
        pub mod celestial_body;
        pub mod ephemeris;
    }
    pub mod uranian {
        pub mod dial;
    }
}

mod ui {
    pub mod components {
        pub mod dial_controller;
        pub mod celestial_body_view_model;
    }
}

mod utils {
    pub mod angle;
    pub mod math_utils;
}

use ui::components::dial_controller;
use std::sync::Arc;
use std::sync::Mutex;

fn main() -> Result<()> {
    // Initialize the logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    info!("Starting Uranian Astrology Dial application...");
    
    // Create the dial controller first
    let dial_controller = Rc::new(RefCell::new(dial_controller::DialController::new()));
    
    // Initialize Qt application and engine
    let mut engine = QmlEngine::new();
    
    // Register our QML types
    dial_controller::register_qml_types();
    
    // Set up QML imports path to include our QML files
    let mut import_path = std::env::current_dir()
        .context("Failed to get current directory")?;
    import_path.push("src/ui/qml");
    
    let qml_import_path = import_path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid QML import path"))?;
    
    // Add the import path to the engine
    engine.add_import_path(qml_import_path.into());
    
    // Set the controller as a QML property
    let root_ctx = engine.root_context();
    
    // Create a QML property for the dial controller
    let controller_obj = dial_controller.borrow().qobject();
    root_ctx.set_property("dialController", controller_obj);
    
    // Initialize with current time and default location (can be overridden by QML)
    let now = chrono::Local::now();
    let datetime_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let datetime = QString::from(datetime_str);
    
    // Default location (can be changed in UI)
    let default_lat = 0.0;  // Equator
    let default_lng = 0.0;  // Prime Meridian
    
    // Initialize the controller
    if !dial_controller.borrow_mut().initialize(datetime, default_lat, default_lng) {
        error!("Failed to initialize dial controller");
        return Err(anyhow::anyhow!("Failed to initialize dial controller"));
    }
    
    // Load the main QML file
    let qml_path = format!("qrc:/qml/main.qml");
    engine.load(qml_path.into());
    
    // Check for QML errors
    if let Some(error) = engine.get_error() {
        let error_str = error.to_string();
        error!("QML Error: {}", error_str);
        return Err(anyhow::anyhow!("Failed to load QML: {}", error_str));
    }
    
    // Run the application using QmlEngine's exec method
    engine.exec();
    
    Ok(())
}

// Re-export for use in other modules
pub use astrology::models::planet::Planet;
pub use astrology::uranian::dial::UranianDial;