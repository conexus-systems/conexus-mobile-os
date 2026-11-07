// Testes do PoC (QtTest): estados da máquina de navegação + tiles + transição
#include <QtTest>
#include <memory>
#include <QQmlEngine>
#include <QQmlComponent>
#include <QQmlContext>

class ShellTest : public QObject
{
    Q_OBJECT
private slots:
    void estadoInicialEtiles() {
        QQmlEngine e;
        QQmlComponent c(&e, QUrl::fromLocalFile(QStringLiteral(SRC_DIR "/qml/Main.qml")));
        QVERIFY2(c.isReady(), qPrintable(c.errorString()));
        std::unique_ptr<QObject> root(c.create());
        QVERIFY(root);
        QCOMPARE(root->property("shellState").toString(), QStringLiteral("home"));
        QVERIFY(root->property("tilesCount").toInt() >= 6);
    }
    void transicaoDeEstados() {
        QQmlEngine e;
        QQmlComponent c(&e, QUrl::fromLocalFile(QStringLiteral(SRC_DIR "/qml/Main.qml")));
        QVERIFY2(c.isReady(), qPrintable(c.errorString()));
        std::unique_ptr<QObject> root(c.create());
        QVERIFY(root);
        QMetaObject::invokeMethod(root.get(), "go", Q_ARG(QVariant, QStringLiteral("controlCenter")));
        QCOMPARE(root->property("shellState").toString(), QStringLiteral("controlCenter"));
        QMetaObject::invokeMethod(root.get(), "go", Q_ARG(QVariant, QStringLiteral("locked")));
        QCOMPARE(root->property("shellState").toString(), QStringLiteral("locked"));
    }
};

QTEST_MAIN(ShellTest)
#include "tst_shell.moc"
