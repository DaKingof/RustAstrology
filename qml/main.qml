import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15
import RustAstrology 1.0

// Import our custom QML components
import "qrc:/qml"

ApplicationWindow {
    id: mainWindow
    visible: true
    width: 1000
    height: 700
    minimumWidth: 800
    minimumHeight: 600
    title: "Uranian Astrology Dial"
    color: "#f0f0f0"
    
    // Set default font for the entire application
    font.family: Qt.application.font.family
    font.pixelSize: Qt.application.font.pixelSize
    
    // Dial controller
    DialController {
        id: dialController
    }
    
    // Main layout
    ColumnLayout {
        anchors.fill: parent
        spacing: 0
        
        // Header
        Rectangle {
            id: header
            Layout.fillWidth: true
            height: 60
            color: "#6200ee"
            
            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 20
                anchors.rightMargin: 20
                spacing: 20
                
                Text {
                    text: "Uranian Astrology Dial"
                    color: "white"
                    font.pixelSize: 20
                    font.bold: true
                    font.family: Qt.platform.os === "windows" ? "Segoe UI, Arial, sans-serif" :
                                 Qt.platform.os === "osx" ? "SF Pro, Helvetica Neue, Arial, sans-serif" :
                                 "Ubuntu, Noto Sans, Arial, sans-serif"
                }
                
                Item { Layout.fillWidth: true }
                
                // Harmonic controls
                RowLayout {
                    spacing: 10
                    
                    // Harmonic label
                    Text {
                        text: "Harmonic:"
                        color: "white"
                        font.pixelSize: 14
                        verticalAlignment: Text.AlignVCenter
                        height: 40
                    }
                    
                    // Harmonic spin box
                    SpinBox {
                        id: harmonicSpinBox
                        from: 1
                        to: 90
                        value: 1
                        editable: true
                        
                        // Style the spin box to match Material Design
                        Material.foreground: "white"
                        Material.accent: "#BB86FC"
                        
                        // Update the dial when the harmonic changes
                        onValueChanged: {
                            if (value >= from && value <= to) {
                                dialController.setHarmonic(value);
                                uranianDial.harmonic = value;
                                uranianDial.update();
                            }
                        }
                        
                        // Custom text formatter to show 'x' after the number
                        textFromValue: function(value) {
                            return value + "x";
                        }
                        
                        // Parse the value from text input
                        valueFromText: function(text) {
                            return parseInt(text.replace(/[^0-9]/g, '')) || 1;
                        }
                    }
                    
                    // Reset harmonic to 1x
                    Button {
                        text: "Reset"
                        flat: true
                        Material.foreground: "white"
                        onClicked: {
                            harmonicSpinBox.value = 1;
                        }
                    }
                }
            }
        }
        
        // Main content
        RowLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 0
            
            // Left panel - controls
            Pane {
                Layout.preferredWidth: 300
                Layout.fillHeight: true
                Material.elevation: 2
                padding: 16
                
                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 15
                    
                    // Display options for the dial
                    GroupBox {
                        id: displayOptions
                        title: "Display Options"
                        Layout.fillWidth: true
                        
                        property bool showZodiacSigns: true
                        property bool showDegreeMarkers: true
                        property bool showPlanets: true
                        property bool showMidpoints: true
                        property bool showAspects: true
                        property bool showGrid: false
                        
                        ColumnLayout {
                            spacing: 8
                            
                            // Zodiac signs toggle
                            CheckBox {
                                text: "Show Zodiac Signs"
                                checked: displayOptions.showZodiacSigns
                                onToggled: {
                                    displayOptions.showZodiacSigns = checked;
                                    uranianDial.showZodiacSigns = checked;
                                }
                            }
                            
                            // Degree markers toggle
                            CheckBox {
                                text: "Show Degree Markers"
                                checked: displayOptions.showDegreeMarkers
                                onToggled: {
                                    displayOptions.showDegreeMarkers = checked;
                                    uranianDial.showDegreeMarkers = checked;
                                }
                            }
                            
                            // Planets toggle
                            CheckBox {
                                text: "Show Planets"
                                checked: displayOptions.showPlanets
                                onToggled: {
                                    displayOptions.showPlanets = checked;
                                    uranianDial.showPlanets = checked;
                                }
                            }
                            
                            // Midpoints toggle
                            CheckBox {
                                text: "Show Midpoints"
                                checked: displayOptions.showMidpoints
                                onToggled: {
                                    displayOptions.showMidpoints = checked;
                                    uranianDial.showMidpoints = checked;
                                }
                            }
                            
                            // Aspects toggle
                            CheckBox {
                                text: "Show Aspects"
                                checked: displayOptions.showAspects
                                onToggled: {
                                    displayOptions.showAspects = checked;
                                    uranianDial.showAspects = checked;
                                }
                            }
                            
                            // Grid toggle
                            CheckBox {
                                text: "Show Grid"
                                checked: displayOptions.showGrid
                                onToggled: {
                                    displayOptions.showGrid = checked;
                                    uranianDial.showGrid = checked;
                                }
                            }
                        }
                    }
                    
                    GroupBox {
                        title: "Planet Positions"
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        
                        ListView {
                            id: planetList
                            anchors.fill: parent
                            clip: true
                            model: dialController.planet_models
                            
                            delegate: ItemDelegate {
                                width: parent.width
                                height: 30
                                
                                RowLayout {
                                    anchors.fill: parent
                                    spacing: 10
                                    
                                    Text {
                                        text: modelData.symbol
                                        font.pixelSize: 16
                                        color: modelData.color
                                    }
                                    
                                    Text {
                                        text: modelData.name
                                        Layout.fillWidth: true
                                        elide: Text.ElideRight
                                    }
                                    
                                    Text {
                                        text: modelData.position.toFixed(2) + "°"
                                        font.family: "monospace"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Center panel - dial
            // Main dial area
            Pane {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Material.elevation: 1
                padding: 0
                
                // Uranian Dial Component
                UranianDial {
                    id: uranianDial
                    anchors.fill: parent
                    anchors.margins: 10
                    
                    // Connect to the controller
                    controller: dialController
                    
                    // Visual properties
                    showZodiacSigns: displayOptions.showZodiacSigns
                    showDegreeMarkers: displayOptions.showDegreeMarkers
                    showPlanets: displayOptions.showPlanets
                    showMidpoints: displayOptions.showMidpoints
                    
                    // Interaction properties
                    rotation: 0
                    zoom: 1.0
                    
                    // Handle rotation changes
                    onRotationChanged: {
                        // Sync with controller if needed
                    }
                }
                
                // Debug overlay (visible only in debug mode)
                Rectangle {
                    visible: false  // Set to true for debugging
                    anchors.fill: parent
                    color: "transparent"
                    border.color: "red"
                    border.width: 2
                    
                    Text {
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.margins: 5
                        text: `Harmonic: ${dialController.harmonicFactor}x`
                        color: "red"
                        font.pixelSize: 12
                    }
                }
            }
        }
        
        // Status bar
        Rectangle {
            Layout.fillWidth: true
            height: 30
            color: "#e0e0e0"
            border.color: "#cccccc"
            border.width: 1
            
            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 10
                anchors.rightMargin: 10
                spacing: 20
                
                Text {
                    text: "Rotation: " + dialComponent.rotation.toFixed(1) + "°"
                    font.pixelSize: 12
                }
                
                Text {
                    text: "Zoom: " + (dialComponent.zoom * 100).toFixed(0) + "%"
                    font.pixelSize: 12
                }
                
                Item { Layout.fillWidth: true }
                
                Text {
                    text: "Harmonic: " + dialComponent.harmonic
                    font.pixelSize: 12
                    font.bold: true
                }
            }
        }
    }
    
    // Add some sample data on startup
    Component.onCompleted: {
        // Initialize the dial with default settings
        dialController.initialize();
        
        // Add sample planets with their positions (in degrees)
        dialController.addPlanet("Sun", 45.0);
        dialController.addPlanet("Moon", 120.0);
        dialController.addPlanet("Mercury", 30.0);
        dialController.addPlanet("Venus", 60.0);
        dialController.addPlanet("Mars", 90.0);
        dialController.addPlanet("Jupiter", 210.0);
        dialController.addPlanet("Saturn", 240.0);
        dialController.addPlanet("Uranus", 15.0);
        dialController.addPlanet("Neptune", 330.0);
        dialController.addPlanet("Pluto", 285.0);
        dialController.addPlanet("North Node", 150.0);
        
        // Add sample midpoints
        dialController.addMidpoint("Sun/Moon", 82.5);
        dialController.addMidpoint("Mercury/Venus", 45.0);
        dialController.addMidpoint("Mars/Jupiter", 150.0);
        
        // Set initial harmonic
        dialController.setHarmonic(1);
        
        // Update the dial display
        uranianDial.update();
    }
}