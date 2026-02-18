// Settings page following the KDE Human Interface Guidelines
//
// - All labels wrapped in i18n() for translation
// - Accessible.name on every interactive control
// - Kirigami.FormLayout for consistent label alignment
// - Ellipsis on "Settings…" (upstream) per KDE HIG

import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Kirigami.ScrollablePage {
    id: settingsPage
    title: i18n("Settings")
    Accessible.name: i18n("Application settings page")

    // Default values used when resetting
    readonly property string defaultShell: "/bin/bash"
    readonly property int defaultFontSize: 11
    readonly property bool defaultTransparency: false
    readonly property bool defaultKdeColors: true
    readonly property bool defaultKwallet: true

    ColumnLayout {
        width: parent.width
        spacing: Kirigami.Units.largeSpacing

        // ── Appearance ──────────────────────────────────────────
        Kirigami.FormLayout {
            Layout.fillWidth: true

            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: i18n("Appearance")
            }

            QQC2.ComboBox {
                id: themeCombo
                Kirigami.FormData.label: i18n("Color scheme:")
                model: [
                    i18n("Follow KDE Plasma"),
                    i18n("Light"),
                    i18n("Dark"),
                    i18n("Custom")
                ]
                Accessible.name: i18n("Terminal color scheme")
                Accessible.description: i18n("Choose which color scheme the terminal uses")

                onCurrentTextChanged: {
                    terminalBridge.applyColorScheme(currentText)
                }
            }

            QQC2.SpinBox {
                id: fontSizeSpinner
                Kirigami.FormData.label: i18n("Font size:")
                from: 8
                to: 32
                value: settingsPage.defaultFontSize
                Accessible.name: i18n("Terminal font size")

                onValueChanged: {
                    terminalBridge.setFontSize(value)
                }
            }

            QQC2.CheckBox {
                id: transparencyCheck
                text: i18n("Enable transparency")
                Kirigami.FormData.label: ""
                checked: settingsPage.defaultTransparency
                Accessible.name: i18n("Enable terminal background transparency")

                onCheckedChanged: {
                    terminalBridge.setOpacity(checked ? 0.9 : 1.0)
                }
            }
        }

        // ── Shell ───────────────────────────────────────────────
        Kirigami.FormLayout {
            Layout.fillWidth: true

            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: i18n("Shell")
            }

            QQC2.TextField {
                id: shellPath
                Kirigami.FormData.label: i18n("Shell command:")
                placeholderText: settingsPage.defaultShell
                text: settingsPage.defaultShell
                Accessible.name: i18n("Path to the shell executable")
            }

            QQC2.CheckBox {
                id: starshipCheck
                text: i18n("Enable Starship prompt")
                Kirigami.FormData.label: ""
                checked: true
                Accessible.name: i18n("Use Starship cross-shell prompt")
            }
        }

        // ── KDE Integration ─────────────────────────────────────
        Kirigami.FormLayout {
            Layout.fillWidth: true

            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: i18n("KDE Integration")
            }

            QQC2.CheckBox {
                id: kdeColorsCheck
                text: i18n("Use KDE color scheme")
                Kirigami.FormData.label: ""
                checked: settingsPage.defaultKdeColors
                Accessible.name: i18n("Automatically follow the KDE Plasma color scheme")

                onCheckedChanged: {
                    themeCombo.enabled = !checked
                    if (checked) {
                        themeCombo.currentIndex = 0
                    }
                }
            }

            QQC2.CheckBox {
                id: kwalletCheck
                text: i18n("Store API keys in KWallet")
                Kirigami.FormData.label: ""
                checked: settingsPage.defaultKwallet
                Accessible.name: i18n("Use KDE Wallet to securely store API credentials")
            }

            QQC2.CheckBox {
                id: activitiesCheck
                text: i18n("Use Plasma Activities")
                Kirigami.FormData.label: ""
                checked: false
                enabled: true
                Accessible.name: i18n("Load different configurations per Plasma Activity")

                QQC2.ToolTip.text: i18n("Different terminal configurations per Plasma Activity")
                QQC2.ToolTip.visible: activitiesHover.hovered

                HoverHandler {
                    id: activitiesHover
                }
            }
        }

        // ── AI Assistant ────────────────────────────────────────
        Kirigami.FormLayout {
            Layout.fillWidth: true

            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: i18n("AI Assistant")
            }

            QQC2.TextField {
                id: openaiKeyField
                Kirigami.FormData.label: i18n("OpenAI API key:")
                placeholderText: "sk-…"
                echoMode: QQC2.TextField.Password
                Accessible.name: i18n("OpenAI API key input")
                Accessible.description: i18n("Enter your OpenAI API key; it will be stored securely via KWallet if enabled")
            }

            QQC2.TextField {
                id: anthropicKeyField
                Kirigami.FormData.label: i18n("Anthropic API key:")
                placeholderText: "sk-ant-…"
                echoMode: QQC2.TextField.Password
                Accessible.name: i18n("Anthropic API key input")
                Accessible.description: i18n("Enter your Anthropic API key; it will be stored securely via KWallet if enabled")
            }
        }

        // ── Actions ─────────────────────────────────────────────
        RowLayout {
            Layout.fillWidth: true
            Layout.topMargin: Kirigami.Units.largeSpacing

            QQC2.Button {
                text: i18n("Reset to Defaults")
                icon.name: "edit-undo"
                Accessible.name: i18n("Reset all settings to their default values")

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
                text: i18n("Apply")
                icon.name: "dialog-ok-apply"
                Accessible.name: i18n("Apply current settings")

                onClicked: {
                    terminalBridge.applyColorScheme(themeCombo.currentText)
                    terminalBridge.setFontSize(fontSizeSpinner.value)
                    terminalBridge.setOpacity(transparencyCheck.checked ? 0.9 : 1.0)
                }
            }
        }
    }
}
