use std::env;
use std::fs;
use std::path::Path;

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Re-run if any QML files change
    println!("cargo:rerun-if-changed=qml");
    println!("cargo:rerun-if-changed=src/ui/qml");
    
    // Link to required Qt libraries
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=Qt5Core");
    println!("cargo:rustc-link-lib=Qt5Gui");
    println!("cargo:rustc-link-lib=Qt5Qml");
    println!("cargo:rustc-link-lib=Qt5Quick");
    println!("cargo:rustc-link-lib=Qt5QuickControls2");
    println!("cargo:rustc-link-lib=Qt5QuickTemplates2");
    println!("cargo:rustc-link-lib=Qt5QuickControls2Impl");
    
    // Add Qt library paths to the linker search path
    if let Ok(qt_path) = env::var("QT_PATH") {
        println!("cargo:rustc-link-search={}/lib", qt_path);
    }
    
    // Set up the Qt plugin path
    let qt_plugin_path = env::var("QT_PLUGIN_PATH").unwrap_or_else(|_| "/usr/lib/x86_64-linux-gnu/qt5/plugins".to_string());
    println!("cargo:rustc-env=QT_PLUGIN_PATH={}", qt_plugin_path);
    
    // Set QML import path
    let out_dir = env::var("OUT_DIR").unwrap();
    let qml_import_path = format!("{}/qml_imports", out_dir);
    println!("cargo:rustc-env=QML_IMPORT_PATH={}", qml_import_path);
    
    // Create necessary directories
    let qml_dest_dir = format!("{}/qml", env::var("OUT_DIR").unwrap());
    fs::create_dir_all(&qml_dest_dir)?;
    
    // Copy QML files to the build directory
    if Path::new("qml").exists() {
        copy_dir_all(Path::new("qml"), Path::new(&qml_dest_dir))?;
    }
    
    // Copy our custom QML components
    let src_qml_dir = "src/ui/qml";
    if Path::new(src_qml_dir).exists() {
        copy_dir_all(Path::new(src_qml_dir), Path::new(&qml_dest_dir))?;
    }
    
    // Set the QML2_IMPORT_PATH environment variable
    println!("cargo:rustc-env=QML2_IMPORT_PATH={}", qml_import_path);
    
    // Set the QML_IMPORT_TRACE environment variable for debugging
    if env::var("DEBUG_QML").is_ok() {
        println!("cargo:rustc-env=QML_IMPORT_TRACE=1");
    }
    
    Ok(())
}
