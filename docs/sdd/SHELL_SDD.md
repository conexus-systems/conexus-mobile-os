# SHELL_SDD — Conexus Shell (PoC Fase 1 → MVP Fase 2)

Implementação: `shell/conexus-shell-poc/` (Qt 6 / QML, QtWaylandCompositor disponível no toolchain).

## Arquitetura do PoC

| Peça | Arquivo | Papel |
|---|---|---|
| Host app | `src/main.cpp` | QGuiApplication + QML engine; `--screenshot`/`--state` para evidências offscreen |
| Shell | `qml/Main.qml` | máquina de estados (`shellState`, `go()`), 6 telas, gestos de borda, tokens dark de `design-tokens.json` |
| Tile | `qml/ConexusTile.qml` | ConexusTile (168px, flip do relógio = dado real; demos marcados) |
| Testes | `tests/tst_shell.cpp` | QtTest: estado inicial, tiles, transições |

## Caminho para o MVP (Fase 2)

1. **Compositor**: trocar `Window` por `QtWaylandCompositor` (QML) — apps Wayland viram telas do modelo de navegação.
2. **Tiles engine**: TileProvider API (registro autorizado por app, D-Bus), cache persistente (sqlite), política de bateria, resize/dnd/pastas/badges.
3. **Design system**: tokens completos (light/dark/hc) carregados de `design-tokens.json` (hoje: subset dark embutido, mapeado 1:1).
4. **Notificações**: Notification broker (serviço Rust, Fase 3) → Action Center.
5. **A11y**: navegação por teclado, redução de movimento, contrastes dos tokens `hc`.

## Componentes do design system (§6 do prompt)

Implementados no PoC: ConexusTile. Próximos: ConexusTileGrid, ConexusAppList, ConexusSwipeContainer,
ConexusTaskSwitcher, ConexusNotification, ConexusQuickAction, ConexusDialog, ConexusSettingsRow.
Interfaces compartilháveis QML↔Flutter: tokens JSON + contratos de estado (mesmos nomes de estados
do SWIPE_NAVIGATION_SDD) — sem acoplamento de runtime.
