# TEST_STRATEGY — Fase 1

## Pirâmide atual

| Camada | Ferramenta | Evidência |
|---|---|---|
| Unit/integração (QML) | QtTest (`tests/tst_shell.cpp`) | 4 passed / 0 failed (estado inicial, tiles ≥6, transições go()) |
| Visual | screenshots offscreen (`--screenshot`, QT_QPA_PLATFORM=offscreen) | home/locked/controlCenter/appSwitcher.png (720×1280) |
| Boot | QEMU + initramfs busybox, console serial | `evidence-qemu-boot.log` (`CONEXUS-BOOT-OK`, ~4 s) |

## Como reproduzir

```bash
cd shell/conexus-shell-poc
cmake -B build -S . && cmake --build build
QT_QPA_PLATFORM=offscreen ctest --test-dir build --output-on-failure
QT_QPA_PLATFORM=offscreen ./build/conexus-shell --screenshot home.png --state home
```

## Metas (do prompt §16) — ainda não medidas

p95 toque→resposta ≤ 100 ms · 60 fps · 24 h sem crash · boot→shell ≤ 30 s.
Instrumentação entra na Fase 2 (compositor + runtime de apps).

## Pendente

qmltestrunner ausente no host (usamos QtTest C++) · testes de gesto com dados
de entrada sintéticos · hardware-in-the-loop (Lumia 730 = Fase 6, gates de
`meridian-os/docs/lumia730-research.md`).
