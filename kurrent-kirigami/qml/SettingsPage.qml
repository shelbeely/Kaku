// Settings page using Kirigami
import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.kirigami.delegates as KirigamiDelegates

Kirigami.ScrollablePage {
    id: settingsPage
    title: "Settings"
    
    ColumnLayout {
        width: parent.width
        spacing: Kirigami.Units.largeSpacing
        
        // Appearance Section
        Kirigami.FormLayout {
            Layout.fillWidth: true
            
            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: "Appearance"
            }
            
            QQC2.ComboBox {
                id: themeCombo
                Kirigami.FormData.label: "Color Scheme:"
                model: ["Follow KDE Plasma", "Light", "Dark", "Custom"]
                
                // TODO: Connect to Rust backend to apply theme
            }
            
            QQC2.SpinBox {
                id: fontSizeSpinner
                Kirigami.FormData.label: "Font Size:"
                from: 8
                to: 32
                value: 11
                
                // TODO: Connect to terminal config
            }
            
            QQC2.CheckBox {
                id: transparencyCheck
                Kirigami.FormData.label: "Enable Transparency:"
                checked: false
                
                // TODO: Connect to window opacity setting
            }
        }
        
        // Shell Section
        Kirigami.FormLayout {
            Layout.fillWidth: true
            
            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: "Shell"
            }
            
            QQC2.TextField {
                id: shellPath
                Kirigami.FormData.label: "Shell Command:"
                placeholderText: "/bin/bash"
                
                // TODO: Load from config
            }
            
            QQC2.CheckBox {
                id: starshipCheck
                Kirigami.FormData.label: "Enable Starship Prompt:"
                checked: true
            }
        }
        
        // KDE Integration Section
        Kirigami.FormLayout {
            Layout.fillWidth: true
            
            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: "KDE Integration"
            }
            
            QQC2.CheckBox {
                id: kdeColorsCheck
                Kirigami.FormData.label: "Use KDE Color Scheme:"
                checked: true
                
                onCheckedChanged: {
                    themeCombo.enabled = !checked
                }
            }
            
            QQC2.CheckBox {
                id: kwalletCheck
                Kirigami.FormData.label: "Store API Keys in KWallet:"
                checked: true
            }
            
            QQC2.CheckBox {
                id: activitiesCheck
                Kirigami.FormData.label: "Use Plasma Activities:"
                checked: false
                enabled: false  // TODO: Enable when implemented
                
                QQC2.ToolTip.text: "Different terminal configs per Plasma Activity"
                QQC2.ToolTip.visible: activitiesHover.hovered
                
                HoverHandler {
                    id: activitiesHover
                }
            }
        }
        
        // AI Assistant Section
        Kirigami.FormLayout {
            Layout.fillWidth: true
            
            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: "AI Assistant"
            }
            
            QQC2.TextField {
                id: openaiKeyField
                Kirigami.FormData.label: "OpenAI API Key:"
                placeholderText: "sk-..."
                echoMode: QQC2.TextField.Password
                
                // TODO: Load from/save to KWallet
            }
            
            QQC2.TextField {
                id: anthropicKeyField
                Kirigami.FormData.label: "Anthropic API Key:"
                placeholderText: "sk-ant-..."
                echoMode: QQC2.TextField.Password
            }
        }
        
        // Action buttons
        RowLayout {
            Layout.fillWidth: true
            Layout.topMargin: Kirigami.Units.largeSpacing
            
            QQC2.Button {
                text: "Reset to Defaults"
                icon.name: "edit-undo"
                
                onClicked: {
                    // TODO: Reset all settings
                }
            }
            
            Item { Layout.fillWidth: true }
            
            QQC2.Button {
                text: "Apply"
                icon.name: "dialog-ok-apply"
                
                onClicked: {
                    // TODO: Save and apply settings
                }
            }
        }
    }
}
