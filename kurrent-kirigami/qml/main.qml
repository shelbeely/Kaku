// Main QML file for Kurrent Kirigami UI
//
// This provides the window chrome, settings UI, and other Qt/QML-based
// interface elements, while the terminal rendering is handled by Rust.

import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Kirigami.ApplicationWindow {
    id: root
    
    title: "Kurrent Terminal"
    width: 1200
    height: 800
    
    // TODO: Embed the Rust-based terminal renderer here
    // This will be done via a custom Qt Quick item that wraps
    // the Rust terminal widget
    
    pageStack.initialPage: terminalPage
    
    // Global drawer for navigation
    globalDrawer: Kirigami.GlobalDrawer {
        title: "Kurrent"
        titleIcon: "utilities-terminal"
        
        actions: [
            Kirigami.Action {
                text: "New Tab"
                icon.name: "tab-new"
                shortcut: "Ctrl+Shift+T"
                onTriggered: {
                    // TODO: Create new terminal tab
                }
            },
            Kirigami.Action {
                text: "New Window"
                icon.name: "window-new"
                shortcut: "Ctrl+Shift+N"
                onTriggered: {
                    // TODO: Create new window
                }
            },
            Kirigami.Action {
                separator: true
            },
            Kirigami.Action {
                text: "Settings"
                icon.name: "settings-configure"
                onTriggered: {
                    pageStack.push(settingsComponent)
                }
            },
            Kirigami.Action {
                text: "About"
                icon.name: "help-about"
                onTriggered: {
                    pageStack.push(aboutComponent)
                }
            }
        ]
    }
    
    // Main terminal page
    Component {
        id: terminalPage
        
        Kirigami.Page {
            title: "Terminal"
            
            // TODO: Replace with actual terminal widget
            ColumnLayout {
                anchors.fill: parent
                
                Kirigami.PlaceholderMessage {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    
                    icon.name: "utilities-terminal"
                    text: "Terminal Renderer"
                    explanation: "The Rust-based terminal will be embedded here"
                }
            }
        }
    }
    
    // Settings page component
    Component {
        id: settingsComponent
        
        SettingsPage {
            // Defined in SettingsPage.qml
        }
    }
    
    // About page component
    Component {
        id: aboutComponent
        
        Kirigami.AboutPage {
            aboutData: {
                "displayName": "Kurrent Terminal",
                "productName": "kurrent",
                "componentName": "kurrent",
                "shortDescription": "KDE Plasma-native terminal for AI coding",
                "homepage": "https://github.com/shelbeely/Kaku",
                "bugAddress": "https://github.com/shelbeely/Kaku/issues",
                "version": "0.3.1",
                "license": "MIT",
                "copyrightStatement": "© 2024-2026 Kurrent Contributors",
                "desktopFileName": "kurrent"
            }
        }
    }
}
