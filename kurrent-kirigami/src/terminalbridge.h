#ifndef TERMINALBRIDGE_H
#define TERMINALBRIDGE_H

#include <QObject>
#include <QString>
#include <QVariant>

/**
 * @brief Bridge between Qt/QML UI and Rust terminal backend
 * 
 * This class provides the interface between the Kirigami UI (Qt/QML)
 * and the Rust-based terminal rendering engine.
 * 
 * Implementation Plan:
 * 1. Use Qt's Foreign Function Interface (FFI) or rust-qt-binding-generator
 * 2. Expose Rust functions to Qt via C ABI
 * 3. Create QQuickItem subclass that embeds Rust OpenGL/Vulkan rendering
 * 4. Handle events: keyboard, mouse, resize
 * 5. Pass configuration changes from QML to Rust
 */
class TerminalBridge : public QObject
{
    Q_OBJECT
    
    Q_PROPERTY(QString currentDirectory READ currentDirectory NOTIFY currentDirectoryChanged)
    Q_PROPERTY(int tabCount READ tabCount NOTIFY tabCountChanged)
    
public:
    explicit TerminalBridge(QObject *parent = nullptr);
    ~TerminalBridge();
    
    // Property getters
    QString currentDirectory() const { return m_currentDirectory; }
    int tabCount() const { return m_tabCount; }
    
public slots:
    // Terminal operations
    void createNewTab();
    void closeTab(int index);
    void selectTab(int index);
    
    // Configuration
    void applyColorScheme(const QString &schemeName);
    void setFontSize(int size);
    void setOpacity(qreal opacity);
    
    // Shell commands
    void sendCommand(const QString &command);
    
signals:
    void currentDirectoryChanged(const QString &directory);
    void tabCountChanged(int count);
    void terminalOutput(const QString &output);
    
private:
    // TODO: These will be replaced with actual Rust FFI calls
    QString m_currentDirectory;
    int m_tabCount;
    
    // TODO: Add pointer to Rust terminal state
    // void* rustTerminalHandle;
};

#endif // TERMINALBRIDGE_H
