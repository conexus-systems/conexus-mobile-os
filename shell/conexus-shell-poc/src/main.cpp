// Conexus Shell PoC — Fase 1 (roadmap Conexus Mobile OS)
// Executa o shell em janela/offscreen; --screenshot <png> [--state <s>] captura evidência.
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickWindow>
#include <QQmlContext>
#include <QTimer>
#include <QImage>
#include <QCommandLineParser>

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    app.setApplicationName(QStringLiteral("Conexus Shell PoC"));
    QCommandLineParser p; p.addHelpOption();
    QCommandLineOption shotOpt("screenshot", "captura PNG e sai", "png");
    QCommandLineOption stateOpt("state", "estado inicial (locked|home|appDrawer|appSwitcher|events|controlCenter)", "state", "home");
    p.addOptions({shotOpt, stateOpt}); p.process(app);
    const QString shotPath = p.value(shotOpt);
    const QString startState = p.value(stateOpt);

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("startState"), startState);
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated, &app,
        [shotPath](QObject *obj, const QUrl &){
            if (!obj) { QCoreApplication::exit(2); return; }
            auto *win = qobject_cast<QQuickWindow*>(obj);
            if (!shotPath.isEmpty()) {
                QTimer::singleShot(400, win, [win, shotPath]{
                    const QImage img = win->grabWindow();
                    if (!img.save(shotPath)) { qCritical("falha ao salvar %s", qPrintable(shotPath)); QCoreApplication::exit(3); return; }
                    qInfo("screenshot: %s (%dx%d)", qPrintable(shotPath), img.width(), img.height());
                    QCoreApplication::exit(0);
                });
            }
    });
    engine.load(QUrl(QStringLiteral("qrc:/Conexus/Shell/qml/Main.qml")));
    if (engine.rootObjects().isEmpty()) return 2;
    return app.exec();
}
