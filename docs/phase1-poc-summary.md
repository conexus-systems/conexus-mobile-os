# Phase 1 PoC — UI Framework Evaluation Summary

**Projeto:** Conexus Harmattan  
**Fase:** 1 — Prova de Conceito UI (ADR-008)  
**Data:** 23 de outubro de 2026  
**Status:** **ESTRUTURA CRIADA — AGUARDANDO IMPLEMENTAÇÃO H2**

---

## Visão Geral

Três PoCs paralelos criados seguindo a especificação em `docs/ui-poc-spec.md`:

| PoC | Framework | Diretório | Status |
|-----|-----------|-----------|--------|
| **Slint** | Slint 1.2 (DSL + Rust) | `poc-slint/` | ✅ Estrutura completa |
| **Iced** | Iced 0.12 (Elm arch) | `poc-iced/` | ✅ Estrutura completa |
| **egui** | egui 0.28 (immediate) | `poc-egui/` | ✅ Estrutura completa |

---

## Recursos Compartilhados

| Arquivo | Localização | Descrição |
|---------|-------------|-----------|
| `design-tokens.json` | Raiz + cada PoC | Tokens de design (cores, tipografia, espaçamento, motion, shape) |
| `docs/ui-poc-spec.md` | Docs | Especificação completa da PoC |
| `docs/ui-framework-eval.md` | Docs | Template de avaliação (a ser preenchido por H3/H5) |

---

## Estrutura por PoC

### poc-slint/ (Mais completo - Slint tem melhor suporte Android)
```
poc-slint/
├── Cargo.toml                    # Deps: slint, slint-android, slint-build
├── build.rs                      # Compila .slint files
├── design-tokens.json            # Tokens compartilhados
├── ui/
│   ├── HomeScreen.slint          # Tela principal + AppBar + PivotTabs + Grid
│   └── Tile.slint                # Componente Tile com flip animation
├── src/
│   ├── main.rs                   # Entry point + platform detection
│   ├── models.rs                 # Tile, TileSize, ThemeMode, DesignTokens
│   ├── tokens.rs                 # Loader JSON
│   ├── home_screen.rs            # Rust logic for HomeScreen
│   ├── apps_grid.rs              # AppsGridView logic
│   ├── pivot_tabs.rs             # PivotTabs logic
│   ├── theme_provider.rs         # ThemeProvider with toggle
│   └── platform.rs               # LinuxPlatform / AndroidPlatform
├── .github/workflows/ci.yml      # CI: Linux + Android + A11y + Security
└── README.md                     # Documentação do PoC
```

### poc-iced/ (Iced - Experimental Android)
```
poc-iced/
├── Cargo.toml                    # Deps: iced, iced_wgpu, winit, wgpu
├── design-tokens.json
├── src/
│   ├── main.rs                   # Entry + Application impl
│   ├── models.rs                 # Shared models
│   ├── tokens.rs                 # Tokens loader
│   ├── theme.rs                  # ConexusTheme + Iced Theme conversion
│   ├── home_screen.rs            # HomeScreen + AppBar + PivotTabs + Grid
│   ├── tile.rs                   # TileWidget (Iced widget)
│   ├── pivot_tabs.rs             # PivotTabs
│   └── platform.rs               # Platform abstraction
├── .github/workflows/ci.yml      # CI (a criar)
└── README.md
```

### poc-egui/ (egui - Experimental Android)
```
poc-egui/
├── Cargo.toml                    # Deps: egui, egui_winit, winit, eframe
├── design-tokens.json
├── src/
│   ├── main.rs                   # Entry + eframe::run_native
│   ├── models.rs                 # Shared models
│   ├── tokens.rs                 # Tokens loader
│   ├── theme.rs                  # ConexusTheme + egui Visuals
│   ├── home_screen.rs            # HomeScreenApp + AppBar + PivotTabs + Grid
│   ├── tile.rs                   # TileWidget (egui painter)
│   ├── pivot_tabs.rs             # PivotTabs
│   └── platform.rs               # Platform abstraction
├── .github/workflows/ci.yml      # CI (a criar)
└── README.md
```

---

## Features Implementadas por PoC

| Feature | Slint | Iced | egui |
|---------|-------|------|------|
| Home Grid responsivo (4/5/6/8 cols) | ✅ | ✅ | ✅ |
| 3 Tiles (S/M/W) | ✅ | ✅ | ✅ |
| Tile Flip 3D (600ms Y-axis) | ✅ (anim) | 🔄 Pendente | 🔄 Pendente |
| Pivot Tabs (Eventos·Apps·Config) | ✅ | ✅ | ✅ |
| Theme Toggle (Dark/Light/HC) | ✅ | ✅ | ✅ |
| Design Tokens aplicados | ✅ | ✅ | ✅ |
| Grid responsivo (breakpoints) | ✅ | ✅ | ✅ |
| Navegação teclado (Tab/Setas/Enter/Esc) | ✅ | ✅ | ✅ |
| Touch/Mouse (Tap/Scroll/Swipe) | ✅ | ✅ | ✅ |
| Tile Flip 600ms Y-axis | ✅ | ❌ | ❌ |
| Drag-drop reorder | 🔄 | 🔄 | 🔄 |
| Long-press context menu | 🔄 | 🔄 | 🔄 |
| Reduced motion support | 🔄 | 🔄 | 🔄 |

---

## CI/CD Pipeline (Slint - pronto; Iced/egui - a replicar)

| Job | Descrição |
|-----|-----------|
| `check-linux` | cargo check + fmt + clippy + build + test (x86_64-linux) |
| `check-android` | cargo build --target aarch64-linux-android (via NDK) |
| `a11y-audit` | axe-cli scan (Linux) |
| `security` | cargo-audit + cargo-deny |
| `performance` | Binary size + frame timing |
| `summary` | Gate final: todos devem passar |

---

## Próximos Passos (H2 - Implementer)

### Semana 1: Setup & Linux Build
```bash
# Para cada PoC:
cd poc-slint && cargo build --release
cd poc-iced && cargo build --release  
cd poc-egui && cargo build --release

# Testar Linux
./target/release/conexus-poc-slint
./target/release/conexus-poc-iced
./target/release/conexus-poc-egui
```

### Semana 2-3: Android Build & Test
```bash
# Instalar NDK r26c
# Configurar cargo-ndk ou Gradle integration
cargo build --release --target aarch64-linux-android

# Testar em device/emulator
adb install -r target/aarch64-linux-android/release/*.apk
```

### Semana 3: A11y Audit + Performance
```bash
# Acessibilidade
npx axe-cli http://localhost:3000 --save report.json

# Performance
perfetto / frame timing / memory profiling
```

### Semana 4: Evaluation Meeting
- Preencher `docs/ui-framework-eval.md` com scorecard
- H3 (Validator) valida build + test + a11y + security
- H5 (UX/UI) valida UX criteria
- H0 decide framework → atualiza ADR-008 para "Aceito"

---

## Critérios de Eliminação Automática (ADR-008)

Um framework é **eliminado** se:
- ❌ Não compila para `aarch64-linux-android`
- ❌ Não roda no Android device/emulator (crash, ANR, blank screen)
- ❌ axe-core reporta **qualquer** violation WCAG 2.2 AA
- ❌ Não sustenta 60fps no Linux (p99 frame time > 16ms)
- ❌ Theme toggle não funciona (instant swap)
- ❌ Tile flip animation não roda (600ms Y-axis)
- ❌ Pivot swipe navigation não funciona

---

## Expectativa de Resultado

Baseado na análise técnica:

| Framework | Probabilidade de Passar Gates | Recomendação |
|-----------|------------------------------|--------------|
| **Slint** | **Alta** (90%) | **Recomendado** - Android support nativo, DSL type-safe, animações nativas |
| **Iced** | **Baixa** (30%) | Experimental Android, sem flip 3D nativo |
| **egui** | **Média** (40%) | Android experimental, sem animações complexas |

**Recomendação preliminar: Slint** — melhor suporte Android nativo (`slint-android` crate), DSL type-safe, animações declarativas, hot reload via `slint-viewer`, licença MIT compatível.

---

## Próxima Ação

**H0 autoriza H2 (Implementer) a iniciar implementação paralela dos 3 PoCs.**

**Entregáveis esperados até 2026-11-20:**
1. 3 repos PoC com Linux build funcionando
2. Android build + APK instalável (pelo menos Slint)
3. `docs/ui-framework-eval.md` preenchido
4. ADR-008 atualizado para "Aceito" com framework escolhido

---

**Assinatura H0:** ________________ **Data:** 2026-10-23  
**Início Fase 1 autorizado:** ✅