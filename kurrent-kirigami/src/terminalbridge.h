#ifndef TERMINALBRIDGE_H
#define TERMINALBRIDGE_H

#include <QObject>
#include <QString>
#include <QVariant>

/**
 * @brief Bridge between Qt/QML UI and Rust terminal backend
 *
 * This class provides the interface between the Kirigami UI (Qt/QML)
 * and the Rust-based terminal rendering engine.  Communication with
 * Rust happens through C ABI FFI functions declared in the extern block.
 */

// Forward-declare the Rust FFI surface.  The actual symbols are resolved at
// link time against libkurrent_gui.so (built with `cargo build --lib`).
extern "C" {
    void* kurrent_terminal_new();
    void  kurrent_terminal_destroy(void* handle);
    void  kurrent_terminal_send_input(void* handle, const char* data, size_t len);
    void  kurrent_terminal_resize(void* handle, int cols, int rows);
}

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
    QString m_currentDirectory;
    int m_tabCount;

    // Pointer to the Rust-side terminal state
    void* m_rustHandle;
};

#endif // TERMINALBRIDGE_H
