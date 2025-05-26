import QtQuick 2.5
import QtQuick.Window 2.2
import RustAstrology 1.0

Window {
    id: mainWindow
    visible: true
    width: 800
    height: 600
    title: "Rust Astrology"
    color: "#f5f5f5"
    
    // Simple header
    Rectangle {
        id: header
        width: parent.width
        height: 50
        color: "#6200ee"
        
        Text {
            anchors.centerIn: parent
            text: "Rust Astrology"
            color: "white"
            font.pixelSize: 20
            font.bold: true
        }
    }
    
    // Main content - simple text
    Rectangle {
        anchors.top: header.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: 20
        color: "white"
        radius: 5
        
        Text {
            anchors.centerIn: parent
            text: "Welcome to Rust Astrology!\n\nThis is a test of the QT environment."
            font.pixelSize: 18
            horizontalAlignment: Text.AlignHCenter
        }
    }
}