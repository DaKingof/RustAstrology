import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Shapes 1.15

Item {
    id: dialComponent
    width: 600
    height: 600
    
    // Properties
    property real rotation: 0
    property real zoom: 1.0
    property bool showPlanets: true
    property bool showZodiac: true
    property bool showDegrees: true
    property bool showHouses: true
    property int harmonic: 1
    
    // Signal when the dial is rotated by user interaction
    signal rotated(real degrees)
    
    // Main dial circle
    Rectangle {
        id: dialCircle
        anchors.centerIn: parent
        width: Math.min(parent.width, parent.height) * 0.9 * zoom
        height: width
        radius: width / 2
        border.width: 2
        border.color: "#333333"
        color: "#f5f5f5"
        
        // Rotate based on the rotation property
        transform: Rotation {
            origin.x: dialCircle.width / 2
            origin.y: dialCircle.height / 2
            angle: dialComponent.rotation
        }
        
        // Zodiac signs (30° each)
        Repeater {
            model: 12
            
            // Zodiac sign glyphs
            Text {
                property real angle: index * 30 - 60  // Offset to start at Aries (0°)
                property real radius: dialCircle.width / 2 - 30
                
                x: dialCircle.width / 2 + Math.cos((angle - 90) * Math.PI / 180) * radius - 10
                y: dialCircle.height / 2 + Math.sin((angle - 90) * Math.PI / 180) * radius - 10
                
                text: ["♈", "♉", "♊", "♋", "♌", "♍", 
                       "♎", "♏", "♐", "♑", "♒", "♓"][index]
                font.pixelSize: 20
                font.bold: true
                
                // Sign name tooltip
                ToolTip.visible: mouseArea.containsMouse
                ToolTip.text: ["Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
                              "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"][index]
                
                MouseArea {
                    id: mouseArea
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                }
            }
        }
        
        // Degree markings (every 5 degrees)
        Repeater {
            model: 72  // 360° / 5° = 72 marks
            
            Rectangle {
                property real angle: index * 5
                
                x: dialCircle.width / 2 - width / 2
                y: 0
                width: index % 6 === 0 ? 2 : 1  // Thicker line every 30°
                height: index % 6 === 0 ? 15 : (index % 3 === 0 ? 10 : 5)
                color: index % 6 === 0 ? "#000000" : "#555555"
                
                transform: [
                    Rotation {
                        origin.x: width / 2
                        origin.y: dialCircle.height / 2
                        angle: angle
                    },
                    Translate {
                        x: dialCircle.width / 2 - width / 2
                        y: 0
                    }
                ]
                
                // Degree numbers (every 10°)
                Text {
                    visible: index % 2 === 0 && showDegrees
                    text: angle === 0 ? "0°" : (angle % 30 === 0 ? angle + "°" : "")
                    x: parent.x - 10
                    y: index % 6 === 0 ? 20 : 15
                    rotation: -angle
                    font.pixelSize: index % 6 === 0 ? 10 : 8
                    color: "#333333"
                }
            }
        }
        
        // Houses (if enabled)
        Repeater {
            model: showHouses ? 12 : 0
            
            Rectangle {
                property real angle: index * 30
                
                x: dialCircle.width / 2 - 1
                y: 0
                width: 2
                height: dialCircle.height / 2
                color: "#FF5722"
                opacity: 0.3
                
                transform: Rotation {
                    origin.x: 0
                    origin.y: 0
                    angle: angle
                }
                
                // House numbers
                Text {
                    x: 10
                    y: 10
                    text: index + 1
                    color: "#FF5722"
                    font.bold: true
                    rotation: -angle
                }
            }
        }
    }
    
    // Mouse area for rotation
    MouseArea {
        anchors.fill: parent
        
        property real lastX: 0
        property real lastY: 0
        
        onPressed: {
            lastX = mouse.x
            lastY = mouse.y
        }
        
        onPositionChanged: {
            if (pressed) {
                var center = Qt.point(width / 2, height / 2)
                var before = Math.atan2(lastY - center.y, lastX - center.x)
                var after = Math.atan2(mouse.y - center.y, mouse.x - center.x)
                var angleChange = (after - before) * 180 / Math.PI
                
                // Update rotation and emit signal
                rotation += angleChange
                rotated(rotation)
                
                lastX = mouse.x
                lastY = mouse.y
            }
        }
        
        onWheel: {
            // Zoom with Ctrl+wheel
            if (wheel.modifiers & Qt.ControlModifier) {
                zoom += wheel.angleDelta.y / 1200
                zoom = Math.max(0.5, Math.min(3.0, zoom))
            } else {
                // Rotate with wheel
                rotation += wheel.angleDelta.y / 12
                rotated(rotation)
            }
        }
    }
    
    // Planet layer
    Repeater {
        id: planetRepeater
        model: ListModel {}
        
        Item {
            id: planetItem
            width: 24
            height: 24
            
            property real planetAngle: model.angle || 0
            property real radius: dialCircle.width / 2 - 30
            
            x: dialCircle.x + dialCircle.width / 2 + Math.cos((planetAngle - 90) * Math.PI / 180) * radius - width / 2
            y: dialCircle.y + dialCircle.height / 2 + Math.sin((planetAngle - 90) * Math.PI / 180) * radius - height / 2
            
            Rectangle {
                id: planetSymbolBg
                anchors.centerIn: parent
                width: 24
                height: 24
                radius: 12
                color: model.color || "#333333"
                border.width: 1
                border.color: "#000000"
                
                Text {
                    anchors.centerIn: parent
                    text: model.symbol || "?"
                    font.pixelSize: 12
                    font.bold: true
                    color: "white"
                }
                
                // Retrograde indicator
                Text {
                    visible: model.isRetrograde || false
                    text: "℞"
                    anchors {
                        right: parent.right
                        bottom: parent.bottom
                        margins: -4
                    }
                    font.pixelSize: 12
                    font.bold: true
                    color: "red"
                }
                
                // Planet name tooltip
                ToolTip {
                    visible: planetMouseArea.containsMouse
                    text: (model.name || "Planet") + " at " + (model.longitude || 0).toFixed(2) + "°"
                    delay: 500
                }
                
                MouseArea {
                    id: planetMouseArea
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    
                    onClicked: {
                        // Emit signal or handle planet click
                        console.log("Planet clicked:", model.name)
                    }
                }
            }
        }
    }
    
    // Function to update planet positions from the controller
    function updatePlanets(planets) {
        planetRepeater.model.clear()
        
        for (var i = 0; i < planets.length; i++) {
            var planet = planets[i]
            planetRepeater.model.append({
                name: planet.name,
                symbol: planet.symbol,
                angle: planet.longitude,
                longitude: planet.longitude,
                latitude: planet.latitude,
                color: planet.color,
                isRetrograde: planet.isRetrograde || false
            })
        }
    }
    
    // Function to update a single planet's position
    function updatePlanetPosition(planetName, longitude, latitude) {
        for (var i = 0; i < planetRepeater.model.count; i++) {
            if (planetRepeater.model.get(i).name === planetName) {
                planetRepeater.model.setProperty(i, "longitude", longitude)
                planetRepeater.model.setProperty(i, "latitude", latitude)
                planetRepeater.model.setProperty(i, "angle", longitude) // Update display angle
                break
            }
        }
    }
    
    // Function to clear all planets
    function clearPlanets() {
        planetRepeater.model.clear()
    }
}
