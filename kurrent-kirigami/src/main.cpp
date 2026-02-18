#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QIcon>
#include <QDir>

#include <KAboutData>
#include <KLocalizedContext>
#include <KLocalizedString>

#include "terminalbridge.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    // Standard KDE application metadata via KAboutData
    KAboutData aboutData(
        QStringLiteral("kurrent"),                          // component name
        i18n("Kurrent Terminal"),                            // display name
        QStringLiteral("0.3.1"),                            // version
        i18n("KDE Plasma-native terminal emulator for AI-assisted coding"), // description
        KAboutLicense::MIT,                                 // license
        i18n("© 2024–2026 Kurrent Contributors"),           // copyright
        QString(),                                          // other text
        QStringLiteral("https://github.com/shelbeely/Kaku"),// homepage
        QStringLiteral("https://github.com/shelbeely/Kaku/issues") // bug address
    );
    aboutData.setDesktopFileName(QStringLiteral("org.kde.kurrent"));
    aboutData.setOrganizationDomain(QStringLiteral("kde.org"));
    KAboutData::setApplicationData(aboutData);

    // Use the project icon
    QGuiApplication::setWindowIcon(QIcon::fromTheme(QStringLiteral("org.kde.kurrent")));

    // Create QML engine
    QQmlApplicationEngine engine;

    // Provide KDE i18n context to QML so that i18n() / i18nc() calls work
    engine.rootContext()->setContextObject(new KLocalizedContext(&engine));

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
