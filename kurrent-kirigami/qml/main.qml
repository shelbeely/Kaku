// Main QML file for Kurrent Kirigami UI
//
// Follows the KDE Human Interface Guidelines:
// - All user-visible strings wrapped in i18n() for translation
// - Accessible.name / Accessible.description on interactive elements
// - Standard KDE keyboard shortcuts
// - KAboutData-driven About page

import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Kirigami.ApplicationWindow {
    id: root

    title: i18n("Kurrent Terminal")
    width: 1200
    height: 800

    pageStack.initialPage: terminalPage

    // Global drawer for navigation (KDE HIG: use GlobalDrawer for top-level nav)
    globalDrawer: Kirigami.GlobalDrawer {
        title: i18n("Kurrent")
        titleIcon: "org.kde.kurrent"
        isMenu: true
        actions: [
            Kirigami.Action {
                text: i18n("New Tab")
                icon.name: "tab-new"
                shortcut: StandardKey.AddTab
                onTriggered: terminalBridge.createNewTab()
                Accessible.name: i18n("Create a new terminal tab")
            },
            Kirigami.Action {
                text: i18n("New Window")
                icon.name: "window-new"
                shortcut: "Ctrl+Shift+N"
                onTriggered: {
                    var component = Qt.createComponent("qrc:/qml/main.qml");
                    if (component.status === Component.Ready) {
                        var win = component.createObject(null);
                        if (win) win.show();
                    } else {
                        console.error("Failed to create window:", component.errorString());
                    }
                }
                Accessible.name: i18n("Open a new terminal window")
            },
            Kirigami.Action {
                separator: true
            },
            Kirigami.Action {
                text: i18n("Settings…")
                icon.name: "settings-configure"
                shortcut: StandardKey.Preferences
                onTriggered: pageStack.push(settingsComponent)
                Accessible.name: i18n("Open application settings")
            },
            Kirigami.Action {
                text: i18n("About Kurrent")
                icon.name: "help-about"
                onTriggered: pageStack.push(aboutComponent)
                Accessible.name: i18n("Show application information")
            }
        ]
    }

    // Main terminal page
    Component {
        id: terminalPage

        Kirigami.Page {
            title: i18n("Terminal")
            Accessible.name: i18n("Terminal page")

            ColumnLayout {
                anchors.fill: parent

                Kirigami.PlaceholderMessage {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    icon.name: "utilities-terminal"
                    text: i18n("Terminal Renderer")
                    explanation: i18n("The Rust-based terminal will be embedded here.\nBuild with libkurrent_gui.so to enable rendering.")
                    Accessible.name: i18n("Terminal rendering area placeholder")
                }
            }
        }
    }

    // Settings page component
    Component {
        id: settingsComponent

        SettingsPage {}
    }

    // About page driven by KAboutData set in main.cpp
    Component {
        id: aboutComponent

        Kirigami.AboutPage {
            aboutData: {
                "displayName": i18n("Kurrent Terminal"),
                "productName": "kurrent",
                "componentName": "kurrent",
                "shortDescription": i18n("KDE Plasma-native terminal emulator for AI-assisted coding"),
                "homepage": "https://github.com/shelbeely/Kaku",
                "bugAddress": "https://github.com/shelbeely/Kaku/issues",
                "version": "0.3.1",
                "license": "MIT",
                "copyrightStatement": i18n("© 2024–2026 Kurrent Contributors"),
                "desktopFileName": "org.kde.kurrent"
            }
            Accessible.name: i18n("About Kurrent Terminal")
        }
    }
}
