import QtQuick
// Conexus Shell PoC — máquina de estados de navegação (SWIPE_NAVIGATION_SDD.md)
// Estados: locked | home | appDrawer | appSwitcher | events | controlCenter
// Tokens (tema escuro) conforme design-tokens.json na raiz do repo.
Window {
    id: root
    width: 720; height: 1280; visible: true
    color: tok.surface
    title: "Conexus Shell PoC"

    property string shellState: (typeof startState !== "undefined" && startState) ? startState : "home"
    readonly property var tok: ({ surface: "#121212", onSurface: "#E6E1E5",
                                primary: "#A6C8FF", outline: "#938F99", error: "#F2B8B5" })
    readonly property int tilesCount: tilesModel.count
    function go(s) { shellState = s }

    // ---- dados de tiles (dados reais locais; demos identificados) ----
    ListModel {
        id: tilesModel
        ListElement { name: "Relógio";    kind: "clock"; accent: "#0F6CBD" }
        ListElement { name: "Calendário"; kind: "demo";  accent: "#5D5A58" }
        ListElement { name: "Clima";      kind: "demo";  accent: "#008272" }
        ListElement { name: "Mensagens";  kind: "demo";  accent: "#4F6BED" }
        ListElement { name: "Fotos";      kind: "demo";  accent: "#767676" }
        ListElement { name: "Música";     kind: "demo";  accent: "#8764B8" }
        ListElement { name: "Configurações"; kind: "demo"; accent: "#0099BC" }
        ListElement { name: "Terminal";   kind: "demo";  accent: "#292929" }
    }

    // ---- transição de estado (motion 200ms, cancelável por troca de estado) ----
    Item { id: motion; property int motionMs: 200 }
    // cada tela: opacity + x animados; só a ativa recebe input
    // ---- LOCKED ----
    Rectangle {
        id: lockScreen
        anchors.fill: parent; color: "#000000"
        opacity: root.shellState === "locked" ? 1 : 0
        Behavior on opacity { NumberAnimation { duration: motion.motionMs } }
        enabled: root.shellState === "locked"
        Column {
            anchors.centerIn: parent; spacing: 12
            Text { text: Qt.formatTime(new Date(), "HH:mm"); color: "white"
                   font.pixelSize: 84; font.weight: Font.Light; anchors.horizontalCenter: parent.horizontalCenter
                   Timer { interval: 1000; running: true; repeat: true
                           onTriggered: parent.text = Qt.formatTime(new Date(), "HH:mm") } }
            Text { text: "Deslize para cima para desbloquear"; color: "#938F99"
                   font.pixelSize: 14; anchors.horizontalCenter: parent.horizontalCenter }
        }
        MouseArea {
            anchors.fill: parent
            property real startY: 0
            onPressed: startY = mouse.y
            onReleased: if (startY - mouse.y > 120) root.go("home")
        }
    }

    // ---- HOME (Start Screen) ----
    Item {
        id: home
        anchors.fill: parent
        opacity: root.shellState === "home" ? 1 : 0
        x: root.shellState === "home" ? 0 : -40
        Behavior on opacity { NumberAnimation { duration: motion.motionMs } }
        Behavior on x { NumberAnimation { duration: motion.motionMs; easing.type: Easing.OutQuad } }
        enabled: root.shellState === "home"
        Column {
            anchors { fill: parent; margins: 24; topMargin: 48 }
            spacing: 16
            Text { text: "Start"; color: root.tok.onSurface; font.pixelSize: 40; font.weight: Font.Light }
            Flow {
                width: parent.width; spacing: 8
                Repeater {
                    model: tilesModel
                    ConexusTile { name: model.name; kind: model.kind; accent: model.accent }
                }
            }
            Text { text: "Conexus Fluent Swipe · PoC Fase 1"; color: "#938F99"; font.pixelSize: 12 }
        }
    }

    // ---- APP_DRAWER (lista alfabética) ----
    Item {
        id: appDrawer
        anchors.fill: parent
        Rectangle { anchors.fill: parent; color: root.tok.surface }
        opacity: root.shellState === "appDrawer" ? 1 : 0
        x: root.shellState === "appDrawer" ? 0 : 40
        Behavior on opacity { NumberAnimation { duration: motion.motionMs } }
        Behavior on x { NumberAnimation { duration: motion.motionMs; easing.type: Easing.OutQuad } }
        enabled: root.shellState === "appDrawer"
        Column {
            anchors { fill: parent; margins: 24; topMargin: 48 }
            spacing: 16
            Text { text: "Todos os apps"; color: root.tok.onSurface; font.pixelSize: 34; font.weight: Font.Light }
            Repeater {
                model: ["Relógio", "Calendário", "Clima", "Fotos", "Música", "Mensagens", "Terminal", "Configurações"]
                Text { text: modelData; color: root.tok.onSurface; font.pixelSize: 20 }
            }
        }
    }

    // ---- APP_SWITCHER (multitarefa) ----
    Item {
        id: appSwitcher
        anchors.fill: parent
        opacity: root.shellState === "appSwitcher" ? 1 : 0
        Behavior on opacity { NumberAnimation { duration: motion.motionMs } }
        enabled: root.shellState === "appSwitcher"
        Rectangle { anchors.fill: parent; color: "#0A0A0A" }
        Text { text: "Multitarefa — 1 app (PoC)"; color: root.tok.onSurface; anchors.centerIn: parent; font.pixelSize: 22 }
    }

    // ---- EVENTS (feed) ----
    Item {
        id: events
        anchors.fill: parent; Rectangle { anchors.fill: parent; color: root.tok.surface }
        opacity: root.shellState === "events" ? 1 : 0
        x: root.shellState === "events" ? 0 : -width
        Behavior on opacity { NumberAnimation { duration: motion.motionMs } }
        Behavior on x { NumberAnimation { duration: motion.motionMs } }
        enabled: root.shellState === "events"
        Column {
            anchors { fill: parent; margins: 24; topMargin: 48 }
            Text { text: "Eventos"; color: root.tok.onSurface; font.pixelSize: 34; font.weight: Font.Light }
            Text { text: "Feed vazio (PoC — nenhum serviço conectado)"; color: "#938F99"; font.pixelSize: 14 }
        }
    }

    // ---- CONTROL_CENTER ----
    Item {
        id: controlCenter
        anchors.fill: parent
        opacity: root.shellState === "controlCenter" ? 1 : 0
        y: root.shellState === "controlCenter" ? 0 : -height
        Behavior on y { NumberAnimation { duration: motion.motionMs; easing.type: Easing.OutQuad } }
        enabled: root.shellState === "controlCenter"
        Rectangle { anchors.fill: parent; color: "#161616" }
        Column {
            anchors { top: parent.top; left: parent.left; right: parent.right; margins: 24; topMargin: 64 }
            spacing: 12
            Text { text: "Ações rápidas"; color: root.tok.onSurface; font.pixelSize: 24 }
            Row {
                spacing: 8
                Repeater {
                    model: ["Wi-Fi", "Bluetooth", "Dados", "Brilho"]
                    Rectangle { width: 150; height: 64; radius: 2; color: "#202020"
                        Text { text: modelData; color: root.tok.onSurface; anchors.centerIn: parent; font.pixelSize: 14 } }
                }
            }
        }
    }

    // ---- gestos de borda (SWIPE_NAVIGATION_SDD: left→home, right→switcher, top→controlCenter) ----
    MouseArea {
        z: 100; width: 24; height: parent.height; anchors.left: parent.left
        enabled: root.shellState !== "locked"
        property real startX: 0
        onPressed: startX = mouse.x
        onReleased: if (mouse.x - startX > 60) root.go("home")
    }
    MouseArea {
        z: 100; width: 24; height: parent.height; anchors.right: parent.right
        enabled: root.shellState !== "locked"
        property real startX: 0
        onPressed: startX = mouse.x
        onReleased: if (startX - mouse.x > 60) root.go("appSwitcher")
    }
    MouseArea {
        z: 100; height: 24; width: parent.width; anchors.top: parent.top
        enabled: root.shellState !== "locked"
        property real startY: 0
        onPressed: startY = mouse.y
        onReleased: if (mouse.y - startY > 60) root.go("controlCenter")
    }
}
