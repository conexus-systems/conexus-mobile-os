import QtQuick
// ConexusTile — tile do Start Screen (tokens: design-tokens.json, tema escuro)
// kind: "clock" (dado real: hora local) | "demo" (estado demonstrativo identificado — nunca como dado real)
Rectangle {
    id: tile
    property string name: ""
    property string kind: "demo"
    property bool isDemo: kind === "demo"
    property string accent: "#0F6CBD"
    property int flipIndex: 0
    width: 168; height: 168; radius: 2
    color: accent
    opacity: area.containsMouse ? 0.85 : 1.0
    Behavior on opacity { NumberAnimation { duration: 150 } }
    onFlipIndexChanged: bounce.restart()
    SequentialAnimation {
        id: bounce
        NumberAnimation { target: tile; property: "scale"; to: 0.95; duration: 120; easing.type: Easing.InOutQuad }
        NumberAnimation { target: tile; property: "scale"; to: 1.0; duration: 120; easing.type: Easing.InOutQuad }
    }
    Column {
        anchors { left: parent.left; bottom: parent.bottom; margins: 10 }
        spacing: 2
        Text {
            id: clockText
            visible: kind === "clock"
            text: Qt.formatTime(new Date(), "HH:mm")
            color: "white"; font.pixelSize: 34; font.weight: Font.Light
            Timer { interval: 1000; running: parent.visible; repeat: true
                    onTriggered: clockText.text = Qt.formatTime(new Date(), "HH:mm") }
        }
        Text { text: tile.name; color: "white"; font.pixelSize: 13 }
        Text { visible: tile.isDemo; text: "demo"; color: "#E6E1E5"; opacity: 0.7;
               font.pixelSize: 10; font.italic: true }
    }
    MouseArea { id: area; hoverEnabled: true; anchors.fill: parent
                onClicked: tile.flipIndex = (tile.flipIndex + 1) % 2 }
}
