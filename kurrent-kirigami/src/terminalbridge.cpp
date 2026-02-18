#include "terminalbridge.h"
#include <QDebug>

TerminalBridge::TerminalBridge(QObject *parent)
    : QObject(parent)
    , m_currentDirectory(QDir::homePath())
    , m_tabCount(1)
    , m_rustHandle(nullptr)
{
    m_rustHandle = kurrent_terminal_new();
    if (!m_rustHandle) {
        qWarning() << "TerminalBridge: failed to create Rust terminal backend";
    }
    qDebug() << "TerminalBridge initialized";
}

TerminalBridge::~TerminalBridge()
{
    if (m_rustHandle) {
        kurrent_terminal_destroy(m_rustHandle);
        m_rustHandle = nullptr;
    }
}

void TerminalBridge::createNewTab()
{
    m_tabCount++;
    emit tabCountChanged(m_tabCount);
    qDebug() << "Created new tab, count:" << m_tabCount;
}

void TerminalBridge::closeTab(int index)
{
    if (m_tabCount > 1) {
        m_tabCount--;
        emit tabCountChanged(m_tabCount);
    }
    qDebug() << "Closed tab" << index << ", count:" << m_tabCount;
}

void TerminalBridge::selectTab(int index)
{
    qDebug() << "Selected tab" << index;
}

void TerminalBridge::applyColorScheme(const QString &schemeName)
{
    qDebug() << "Applying color scheme:" << schemeName;
}

void TerminalBridge::setFontSize(int size)
{
    qDebug() << "Setting font size:" << size;
}

void TerminalBridge::setOpacity(qreal opacity)
{
    qDebug() << "Setting opacity:" << opacity;
}

void TerminalBridge::sendCommand(const QString &command)
{
    if (m_rustHandle) {
        QByteArray utf8 = command.toUtf8();
        kurrent_terminal_send_input(m_rustHandle, utf8.constData(),
                                     static_cast<size_t>(utf8.size()));
    }
    qDebug() << "Sent command:" << command;
}
