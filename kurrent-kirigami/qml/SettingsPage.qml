// Settings page using Kirigami
import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.kirigami.delegates as KirigamiDelegates

Kirigami.ScrollablePage {
    id: settingsPage
    title: "Settings"

    // Default values used when resetting
    readonly property string defaultShell: "/bin/bash"
    readonly property int defaultFontSize: 11
    readonly property bool defaultTransparency: false
    readonly property bool defaultKdeColors: true
    readonly property bool defaultKwallet: true

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

                onCurrentTextChanged: {
                    terminalBridge.applyColorScheme(currentText)
                }
            }

            QQC2.SpinBox {
                id: fontSizeSpinner
                Kirigami.FormData.label: "Font Size:"
                from: 8
                to: 32
                value: settingsPage.defaultFontSize

                onValueChanged: {
                    terminalBridge.setFontSize(value)
                }
            }

            QQC2.CheckBox {
                id: transparencyCheck
                Kirigami.FormData.label: "Enable Transparency:"
                checked: settingsPage.defaultTransparency

                onCheckedChanged: {
                    terminalBridge.setOpacity(checked ? 0.9 : 1.0)
                }
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
                placeholderText: settingsPage.defaultShell
                text: settingsPage.defaultShell
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
                checked: settingsPage.defaultKdeColors

                onCheckedChanged: {
                    themeCombo.enabled = !checked
                    if (checked) {
                        themeCombo.currentIndex = 0
                    }
                }
            }

            QQC2.CheckBox {
                id: kwalletCheck
                Kirigami.FormData.label: "Store API Keys in KWallet:"
                checked: settingsPage.defaultKwallet
            }

            QQC2.CheckBox {
                id: activitiesCheck
                Kirigami.FormData.label: "Use Plasma Activities:"
                checked: false
                enabled: true

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
                    themeCombo.currentIndex = 0
                    fontSizeSpinner.value = settingsPage.defaultFontSize
                    transparencyCheck.checked = settingsPage.defaultTransparency
                    shellPath.text = settingsPage.defaultShell
                    starshipCheck.checked = true
                    kdeColorsCheck.checked = settingsPage.defaultKdeColors
                    kwalletCheck.checked = settingsPage.defaultKwallet
                    activitiesCheck.checked = false
                    openaiKeyField.text = ""
                    anthropicKeyField.text = ""
                }
            }

            Item { Layout.fillWidth: true }

            QQC2.Button {
                text: "Apply"
                icon.name: "dialog-ok-apply"

                onClicked: {
                    terminalBridge.applyColorScheme(themeCombo.currentText)
                    terminalBridge.setFontSize(fontSizeSpinner.value)
                    terminalBridge.setOpacity(transparencyCheck.checked ? 0.9 : 1.0)
                }
            }
        }
    }
}
