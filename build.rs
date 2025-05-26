fn main() {
    // Re-run if any QML files change
    println!("cargo:rerun-if-changed=qml");
    
    // Link to required Qt libraries
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=Qt5Core");
    println!("cargo:rustc-link-lib=Qt5Gui");
    println!("cargo:rustc-link-lib=Qt5Qml");
    println!("cargo:rustc-link-lib=Qt5Quick");
    println!("cargo:rustc-link-lib=Qt5QuickControls2");
    
    // Add Qt library paths to the linker search path
    if let Ok(qt_path) = std::env::var("QT_PATH") {
        println!("cargo:rustc-link-search={}/lib", qt_path);
    }
    
    // Set up the Qt plugin path
    println!("cargo:rustc-env=QT_PLUGIN_PATH={}", 
        std::env::var("QT_PLUGIN_PATH").unwrap_or_else(|_| "/usr/lib/x86_64-linux-gnu/qt5/plugins".to_string())
    );
}
