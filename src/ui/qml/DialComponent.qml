import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Shapes 1.15

Item {
    id: dialComponent
    width: 600
    height: 600
    
    // Properties exposed to the outside
    property real rotation: 0
    property real zoom: 1.0
    property bool showPlanets: true
    property bool showMidpoints: true
    property bool showZodiacSigns: true
    property bool showDegreeMarkers: true
    property bool showAspects: true
    property bool showGrid: false
    property int harmonic: 1
    property var controller: null
    
    // Colors
    property color backgroundColor: "#1e1e2e"
    property color foregroundColor: "#cdd6f4"
    property color accentColor: "#89b4fa"
    property color highlightColor: "#f5e0dc"
    property color gridColor: "#313244"
    
    // Signal emitted when the dial is rotated by the user
    signal dialRotated(real degrees)
    
    // Background circle
    Rectangle {
        id: dialCircle
        anchors.centerIn: parent
        width: Math.min(parent.width, parent.height) * zoom
        height: width
        radius: width / 2
        border.width: 2
        border.color: gridColor
        color: backgroundColor
        
        // Grid lines (visible when showGrid is true)
        Repeater {
            model: showGrid ? 12 : 0
            
            Rectangle {
                property real angle: index * 30
                x: dialCircle.width / 2 - width / 2
                y: 0
                width: 1
                height: dialCircle.width / 2
                color: gridColor
                
                transform: [
                    Rotation {
                        origin.x: width / 2
                        origin.y: 0
                        angle: angle
                    },
                    Translate {
                        x: 0
                        y: 0
                    }
                ]
            }
        }
        
        // Rotate the entire dial
        transform: Rotation {
            origin.x: dialCircle.width / 2
            origin.y: dialCircle.height / 2
            angle: dialComponent.rotation
        }
        
        // Zodiac signs (outer ring)
        Repeater {
            model: 12
            
            Item {
                property int signIndex: index
                property real angle: index * 30
                property real signStart: angle - 15
                property real signEnd: (angle + 15) % 360
                
                // Sign division line
                Rectangle {
                    x: dialCircle.width / 2 - width / 2
                    y: 0
                    width: 2
                    height: 30
                    color: gridColor
                    
                    transform: [
                        Rotation {
                            origin.x: width / 2
                            origin.y: 0
                            angle: angle
                        },
                        Translate {
                            x: 0
                            y: 0
                        }
                    ]
                }
                
                transform: [
                    Rotation {
                        origin.x: width / 2
                        origin.y: dialCircle.height / 2
                        angle: angle
                    },
                    Translate {
                        x: 0
                        y: 0
                    }
                ]
                
                // Zodiac sign symbol and name
                Item {
                    visible: showZodiacSigns
                    width: 40
                    height: 40
                    
                    // Sign symbol (large)
                    Text {
                        id: signSymbol
                        anchors.centerIn: parent
                        text: {
                            var signs = ["♈", "♉", "♊", "♋", "♌", "♍", "♎", "♏", "♐", "♑", "♒", "♓"]
                            return signs[signIndex]
                        }
                        font.pixelSize: 20
                        font.bold: true
                        color: foregroundColor
                        opacity: 0.9
                    }
                    
                    // Sign name (small, below symbol)
                    Text {
                        anchors {
                            top: signSymbol.bottom
                            horizontalCenter: signSymbol.horizontalCenter
                            topMargin: 2
                        }
                        text: {
                            var names = ["Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo", 
                                       "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"]
                            return names[signIndex]
                        }
                        font.pixelSize: 9
                        font.capitalization: Font.AllUppercase
                        color: foregroundColor
                        opacity: 0.7
                    }
                    
                    // Position around the outer edge
                    x: dialCircle.width / 2 + Math.cos((angle - 90) * Math.PI / 180) * (dialCircle.width / 2 - 30) - width / 2
                    y: dialCircle.height / 2 + Math.sin((angle - 90) * Math.PI / 180) * (dialCircle.height / 2 - 30) - height / 2
                    
                    // Rotate to face outward
                    rotation: angle + 90
                }
            }
        }
        
        // Degree marks (inner ring)
        Repeater {
            model: 360
            
            Item {
                id: degreeMarker
                property real degree: index
                property bool isMajor: index % 30 === 0  // Every 30 degrees (sign cusps)
                property bool isMinor: index % 5 === 0 && !isMajor  // Every 5 degrees
                
                visible: showDegreeMarkers && (isMajor || isMinor)
                
                // Degree line
                Rectangle {
                    width: isMajor ? 2 : 1
                    height: isMajor ? 25 : (isMinor ? 15 : 8)
                    color: isMajor ? accentColor : gridColor
                    opacity: isMajor ? 0.8 : 0.6
                    
                    anchors {
                        horizontalCenter: parent.horizontalCenter
                        top: parent.top
                    }
                }
                
                // Degree number (only for major divisions and every 5 degrees)
                Text {
                    visible: isMajor || isMinor
                    text: degree
                    font.pixelSize: isMajor ? 11 : 8
                    font.bold: isMajor
                    color: isMajor ? highlightColor : foregroundColor
                    opacity: isMajor ? 1.0 : 0.7
                    
                    anchors {
                        horizontalCenter: parent.horizontalCenter
                        top: parent.top
                        topMargin: isMajor ? 30 : 20
                    }
                }
                
                // Position and rotate the marker
                transform: [
                    Rotation {
                        origin.x: dialCircle.width / 2
                        origin.y: dialCircle.height / 2
                        angle: degree
                    },
                    Translate {
                        x: 0
                        y: 0
                    }
                ]
            }
            }
        }
        
        // Center point with crosshairs
        Item {
            id: centerPoint
            width: 16
            height: 16
            anchors.centerIn: parent
            
            // Crosshair lines
            Rectangle {
                width: 1
                height: parent.height
                color: gridColor
                anchors.centerIn: parent
            }
            
            Rectangle {
                width: parent.width
                height: 1
                color: gridColor
                anchors.centerIn: parent
            }
            
            // Center dot
            Rectangle {
                width: 6
                height: 6
                radius: 3
                color: accentColor
                anchors.centerIn: parent
            }
            
            // Small circle around center
            Rectangle {
                anchors.centerIn: parent
                width: 12
                height: 12
                radius: 6
                color: "transparent"
                border.width: 1
                border.color: gridColor
            }
        }
    }
    
    // Mouse area for interaction
    MouseArea {
        id: dialMouseArea
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        
        property real lastX: 0
        property real lastY: 0
        property bool isDragging: false
        property point clickPos: "0,0"
        
        // Mouse wheel for zoom and rotation
        onWheel: {
            if (wheel.modifiers & Qt.ControlModifier) {
                // Zoom with Ctrl+wheel
                var zoomFactor = 1.0 + (wheel.angleDelta.y > 0 ? 0.1 : -0.1);
                var newZoom = dialComponent.zoom * zoomFactor;
                dialComponent.zoom = Math.max(0.5, Math.min(3.0, newZoom));
            } else if (wheel.modifiers & Qt.ShiftModifier) {
                // Adjust harmonic with Shift+wheel
                var delta = wheel.angleDelta.y > 0 ? 1 : -1;
                var newHarmonic = Math.max(1, Math.min(90, dialComponent.harmonic + delta));
                if (newHarmonic !== dialComponent.harmonic) {
                    dialComponent.harmonic = newHarmonic;
                    if (controller) {
                        controller.setHarmonic(newHarmonic);
                    }
                    aspectCanvas.requestPaint();
                }
            } else {
                // Rotate with wheel (no modifier)
                dialComponent.dialRotated(wheel.angleDelta.y / 12);
            }
        }
        
        // Handle mouse press
        onPressed: (mouse) => {
            lastX = mouse.x;
            lastY = mouse.y;
            clickPos = Qt.point(mouse.x, mouse.y);
            isDragging = false;
            
            // Right-click context menu
            if (mouse.button === Qt.RightButton) {
                contextMenu.popup();
            }
        }
        
        // Handle mouse move
        onPositionChanged: (mouse) => {
            if (pressed) {
                var dx = mouse.x - lastX;
                var dy = mouse.y - lastY;
                
                // Check if we've moved enough to consider it a drag
                if (!isDragging && (Math.abs(dx) > 5 || Math.abs(dy) > 5)) {
                    isDragging = true;
                }
                
                if (isDragging) {
                    // Calculate angle change based on drag
                    var center = Qt.point(width / 2, height / 2);
                    var before = Math.atan2(lastY - center.y, lastX - center.x);
                    var after = Math.atan2(mouse.y - center.y, mouse.x - center.x);
                    var angleChange = (after - before) * 180 / Math.PI;
                    
                    dialComponent.dialRotated(angleChange);
                    
                    lastX = mouse.x;
                    lastY = mouse.y;
                }
            }
        }
        
        // Handle click (not drag)
        onClicked: (mouse) => {
            if (!isDragging && Math.abs(mouse.x - clickPos.x) < 5 && Math.abs(mouse.y - clickPos.y) < 5) {
                // Single click - select planet if clicked on one
                var center = Qt.point(width / 2, height / 2);
                var radius = Math.min(width, height) * 0.4;
                var clickAngle = (Math.atan2(mouse.y - center.y, mouse.x - center.x) * 180 / Math.PI + 450) % 360;
                
                // Check if clicked near a planet
                if (controller) {
                    var planets = controller.getPlanetPositions();
                    for (var i = 0; i < planets.length; i++) {
                        var planet = planets[i];
                        var planetAngle = getHarmonicPosition(planet.longitude, dialComponent.harmonic);
                        var angleDiff = Math.abs(planetAngle - clickAngle);
                        if (angleDiff < 5 || angleDiff > 355) { // 5° orb for selection
                            console.log(`Selected ${planet.name} at ${planet.longitude.toFixed(2)}°`);
                            // Emit signal or update selection
                            break;
                        }
                    }
                }
            }
        }
    }
    
    // Canvas for drawing aspects and other dynamic elements
    Canvas {
        id: aspectCanvas
        anchors.fill: parent
        antialiasing: true
        renderTarget: Canvas.Image
        renderStrategy: Canvas.Cooperative
        
        onPaint: {
            var ctx = getContext('2d');
            ctx.clearRect(0, 0, width, height);
            
            // Draw aspect lines if enabled
            if (showAspects && controller) {
                var planets = controller.getPlanetPositions();
                var centerX = width / 2;
                var centerY = height / 2;
                var radius = Math.min(width, height) * 0.4;
                
                // Draw aspect lines between planets
                for (var i = 0; i < planets.length; i++) {
                    for (var j = i + 1; j < planets.length; j++) {
                        var planet1 = planets[i];
                        var planet2 = planets[j];
                        
                        // Calculate angle difference (accounting for harmonic)
                        var angle1 = getHarmonicPosition(planet1.longitude, harmonic);
                        var angle2 = getHarmonicPosition(planet2.longitude, harmonic);
                        var diff = Math.abs(angle1 - angle2);
                        
                        // Check for major aspects (0°, 60°, 90°, 120°, 180°)
                        var aspect = null;
                        if (Math.abs(diff % 30) < 1) { // 1° orb for aspect detection
                            if (Math.abs(diff - 0) < 1) aspect = {name: 'Conjunction', color: '#ff6b6b'};
                            else if (Math.abs(diff - 60) < 1) aspect = {name: 'Sextile', color: '#51cf66'};
                            else if (Math.abs(diff - 90) < 1) aspect = {name: 'Square', color: '#ff922b'};
                            else if (Math.abs(diff - 120) < 1) aspect = {name: 'Trine', color: '#20c997'};
                            else if (Math.abs(diff - 180) < 1) aspect = {name: 'Opposition', color: '#ff8787'};
                        }
                        
                        // Draw aspect line if found
                        if (aspect) {
                            var pos1 = polarToCartesian(centerX, centerY, radius, angle1);
                            var pos2 = polarToCartesian(centerX, centerY, radius, angle2);
                            drawAspectLine(ctx, pos1, pos2, aspect.color, 1);
                        }
                    }
                }
            }
        }
    }
    
    // Display current harmonic in the corner with a nice background
    Rectangle {
        id: harmonicDisplay
        width: 80
        height: 40
        radius: 20
        color: Qt.rgba(0, 0, 0, 0.6)
        border.color: accentColor
        border.width: 1
        
        anchors {
            top: parent.top
            right: parent.right
            margins: 20
        }
        
        Text {
            anchors.centerIn: parent
            text: `H${harmonic}`
            font.pixelSize: 18
            font.bold: true
            color: highlightColor
        }
        
        // Tooltip for harmonic info
        ToolTip.visible: mouseArea.containsMouse
        ToolTip.text: `Harmonic ${harmonic}x\nRange: 0° - ${(360/harmonic).toFixed(2)}°`
        
        MouseArea {
            id: mouseArea
            anchors.fill: parent
            hoverEnabled: true
        }
    }
    
    // Utility functions
    function polarToCartesian(centerX, centerY, radius, angleInDegrees) {
        // Convert angle to radians and adjust for 0° at the top
        var angleInRadians = (angleInDegrees - 90) * Math.PI / 180.0;
        return Qt.point(
            centerX + (radius * Math.cos(angleInRadians)),
            centerY + (radius * Math.sin(angleInRadians))
        );
    }
    
    // Function to calculate harmonic position
    function getHarmonicPosition(degree, harmonic) {
        if (harmonic <= 1) return degree;
        var harmonicRange = 360 / harmonic;
        return (degree % harmonicRange) * harmonic;
    }
    
    // Function to draw aspect lines
    function drawAspectLine(ctx, point1, point2, color, width) {
        ctx.save();
        ctx.beginPath();
        ctx.strokeStyle = color;
        ctx.lineWidth = width;
        ctx.moveTo(point1.x, point1.y);
        ctx.lineTo(point2.x, point2.y);
        ctx.stroke();
        ctx.restore();
    }
    
    // Function to draw a planet symbol
    function drawPlanetSymbol(ctx, x, y, symbol, color, size) {
        ctx.save();
        ctx.font = `${size}px Arial, sans-serif`;
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillStyle = color;
        ctx.fillText(symbol, x, y);
        ctx.restore();
    }
    
    // Context menu for dial actions
    Menu {
        id: contextMenu
        
        MenuItem {
            text: "Reset View"
            onTriggered: {
                dialComponent.rotation = 0;
                dialComponent.zoom = 1.0;
            }
        }
        
        MenuSeparator {}
        
        MenuItem {
            text: "Show Grid"
            checkable: true
            checked: showGrid
            onTriggered: showGrid = !showGrid
        }
        
        MenuItem {
            text: "Show Aspects"
            checkable: true
            checked: showAspects
            onTriggered: showAspects = !showAspects
        }
        
        MenuItem {
            text: "Show Degree Markers"
            checkable: true
            checked: showDegreeMarkers
            onTriggered: showDegreeMarkers = !showDegreeMarkers
        }
        
        MenuItem {
            text: "Show Zodiac Signs"
            checkable: true
            checked: showZodiacSigns
            onTriggered: showZodiacSigns = !showZodiacSigns
        }
    }
    
    // Update canvas when needed
    onShowAspectsChanged: aspectCanvas.requestPaint()
    onHarmonicChanged: aspectCanvas.requestPaint()
    
    // Function to update the view
    function update() {
        aspectCanvas.requestPaint();
    }
}
