# MASTER_SDD — Conexus Mobile OS (One-Shot v1.0)

Mapa dos entregáveis obrigatórios (§20 do prompt) → onde vivem. Última execução: 2026-11-07.

| Entregável | Status | Onde |
|---|---|---|
| MASTER_SDD.md | ✅ | este arquivo |
| PDF_REQUIREMENTS_MATRIX.md | ⚠️ sem PDF na sessão — registrado | `PDF_REQUIREMENTS_MATRIX.md` |
| ARCHITECTURE.md | ✅ preexistente | `SDD.md` (§2 camadas) + `docs/architecture.md` |
| ARCHITECTURE_DIAGRAMS.md | ✅ preexistente | `SDD.md` §2.1, `SDD-HARMATTAN.md` |
| DEVICE_SUPPORT_MATRIX.md | ✅ preexistente + atualizado | `docs/architecture/DEVICE_SUPPORT.md`, `docs/lumia-730/` |
| HARDWARE_ABSTRACTION.md | ✅ preexistente | `docs/architecture/` + `drivers/` (traits Rust) |
| SHELL_SDD.md | ✅ novo | `SHELL_SDD.md` (implementação: `shell/conexus-shell-poc/`) |
| LIVE_TILES_SDD.md | ⚠️ parcial | `SHELL_SDD.md` §tiles (engine completa é Fase 2) |
| SWIPE_NAVIGATION_SDD.md | ✅ novo | `SWIPE_NAVIGATION_SDD.md` |
| APPLICATION_RUNTIME_SDD.md | ⚠️ parcial | `SDD.md` §runtime; container Android = Fase 5 |
| SECURITY_MODEL.md | ✅ preexistente | `SECURITY.md` |
| DESIGN_SYSTEM.md | ✅ preexistente | `design-tokens.json` + `docs/ui-poc-spec.md` |
| TEST_STRATEGY.md | ✅ novo | `TEST_STRATEGY.md` |
| CI_CD.md | ✅ novo | `CI_CD.md` + `.github/workflows/shell-poc.yml` |
| ROADMAP.md | ✅ preexistente | `ROADMAP.md` (Fase 1 em execução) |
| BACKLOG.md | ✅ novo | `BACKLOG.md` |
| RELEASE_CHECKLIST.md | ⚠️ Fase 7 | pendente (marcado no backlog) |
| IMPLEMENTATION_REPORT.md | ✅ novo | `IMPLEMENTATION_REPORT.md` |
| KNOWN_LIMITATIONS.md | ✅ novo | `KNOWN_LIMITATIONS.md` |
| NEXT_ACTIONS.md | ✅ novo | `NEXT_ACTIONS.md` |

## Estado da Fase 1 (PoC) — evidências

1. **Shell PoC Qt6/QML** (`shell/conexus-shell-poc/`): Start com Live Tiles (relógio = dado real; demais marcados `demo`), máquina de estados de navegação (locked/home/appDrawer/appSwitcher/events/controlCenter), gestos de borda. Build Qt 6.11.2 OK.
2. **Testes**: `tst_shell` (QtTest) — estado inicial, contagem de tiles, transições `go()` — **4 passed, 0 failed**.
3. **Screenshots offscreen**: home/locked/controlCenter/appSwitcher (720×1280).
4. **Boot em QEMU**: initramfs busybox + kernel 6.18.55-1-MANJARO → banner CONEXUS + `CONEXUS-BOOT-OK` + powerdown limpo em ~4 s (`evidence-qemu-boot.log`). Sem rootfs distro — próximo passo.

Regras honradas: nenhum dado fictício sem marcador `demo`; nenhum flash físico; nenhuma release.
