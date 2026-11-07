# SWIPE_NAVIGATION_SDD — máquina de estados (implementada no PoC)

Fonte da verdade de implementação: `shell/conexus-shell-poc/qml/Main.qml` (propriedade `shellState` + `go(state)`).

## Estados (§8 do prompt)

| Estado | Tela | Implementado |
|---|---|---|
| LOCKED | lock screen (relógio + swipe up ≥120 px → HOME) | ✅ PoC |
| HOME | Start com Live Tiles | ✅ PoC |
| APP_DRAWER | lista de apps | ✅ PoC |
| APP_FOREGROUND | app em foco | ⏳ Fase 2 (sem runtime de apps ainda) |
| APP_SWITCHER | multitarefa | ✅ PoC (visual; cards reais Fase 2) |
| EVENTS | feed de eventos | ✅ PoC (vazio, sem serviço) |
| CONTROL_CENTER | ações rápidas | ✅ PoC |
| OVERVIEW | visão geral | ⏳ Fase 2 |

## Gestos (borda → destino)

| Gesto | Transição | Detecção |
|---|---|---|
| swipe borda esquerda → direita | → HOME | MouseArea 24 px, limiar 60 px |
| swipe borda direita → esquerda | → APP_SWITCHER | idem |
| swipe topo → baixo | → CONTROL_CENTER | MouseArea 24 px, limiar 60 px |
| swipe up (lock) | LOCKED → HOME | limiar 120 px |

## Requisitos pendentes (Fase 2)

Animações canceláveis por gesto em andamento · áreas de exclusão (dentro de apps) · prevenção de
gesto acidental (velocidade mínima) · haptics · teclado/mouse paralelo · acessibilidade (alternativa
sem gesto) · testes de latência (meta: p95 ≤ 100 ms) · redução de movimento (token `motion`).
