use anyhow::Result;
use ::log::info; // Use fully qualified path to disambiguate
use qmetaobject::*;
use std::ffi::CString;

// Declare a QObject with some properties
#[derive(Default, QObject)]
pub struct AstrologyApp {
    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY name_changed),
    name_changed: qt_signal!(),
}

impl AstrologyApp {
    fn new() -> Self {
        let mut obj = Self::default();
        obj.set_name("Rust Astrology".into());
        obj
    }
    
    fn set_name(&mut self, name: QString) {
        self.name = name;
        self.name_changed();
    }
}

fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();
    
    info!("Starting Rust Astrology application...");
    
    // Initialize Qt application and engine
    let mut engine = QmlEngine::new();
    
    // Register our QML type
    qml_register_type::<AstrologyApp>(
        &CString::new("RustAstrology").unwrap(),
        1,
        0,
        &CString::new("AstrologyApp").unwrap()
    );
    
    // Load the main QML file
    engine.load_file("qml/main.qml".into());
    
    // Run the application using QmlEngine's exec method
    engine.exec();
    
    Ok(())
}