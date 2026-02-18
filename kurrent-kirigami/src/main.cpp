#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QIcon>

#include "terminalbridge.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    
    // Set application metadata
    app.setApplicationName("Kurrent");
    app.setOrganizationName("Kurrent");
    app.setApplicationDisplayName("Kurrent Terminal");
    app.setDesktopFileName("kurrent");
    
    // Set icon theme for KDE
    QIcon::setThemeName("breeze");
    
    // Create QML engine
    QQmlApplicationEngine engine;
    
    // TODO: Initialize Rust terminal backend
    // This is where we'll create the connection between Qt/QML and Rust
    
    // Create bridge to Rust terminal
    TerminalBridge *terminalBridge = new TerminalBridge(&app);
    engine.rootContext()->setContextProperty("terminalBridge", terminalBridge);
    
    // Load main QML file
    const QUrl url(QStringLiteral("qrc:/qml/main.qml"));
    
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);
    
    engine.load(url);
    
    if (engine.rootObjects().isEmpty())
        return -1;
    
    return app.exec();
}
