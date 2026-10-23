# PoC Specification — UI Framework Evaluation (ADR-008)

**Projeto:** Conexus Harmattan  
**Fase:** 1 — Prova de Conceito UI  
**Data:** 23 de outubro de 2026  
**Status:** **ESPECIFICAÇÃO OFICIAL** — H2 (Implementer) deve seguir exatamente  

---

## 1. Objetivo

Selecionar **um** framework UI Rust (Slint, Iced ou egui) mediante **evidência empírica**: cada framework deve implementar a **mesma tela Home** que **compile e execute** em **Linux (Wayland/X11) E Android (NDK)**, passando **todos** os critérios de qualidade.

**Framework vencedor = único que passar 100% dos testes em ambas plataformas.**

---

## 2. Escopo da PoC — Tela Única: `HomeScreen`

### 2.1 Component Tree (Obrigatório)

```
HomeScreen (root)
├── AppBar (topo)
│   ├── Title: "Conexus"
│   ├── ThemeToggle (Dark/Light/HighContrast)
│   └── SearchButton (abre SearchModal)
├── PivotTabs (horizontal swipe + tap)
│   ├── Tab "Eventos" → EventosView (placeholder)
│   ├── Tab "Apps" → AppsGridView (HOME GRID)
│   └── Tab "Configurações" → SettingsView (placeholder)
└── StatusBar (bottom, opcional)
    ├── Battery indicator
    ├── Clock
    └── Connectivity icons
```

### 2.2 AppsGridView — **CORE DA POC**

| Requisito | Detalhe |
|-----------|---------|
| **Grid Responsivo** | 360dp=4cols, 600dp=5cols, 900dp=6cols, 1200dp=8cols |
| **3 Tiles Visíveis** | 1× Small (1×1), 1× Medium (2×2), 1× Wide (4×2) |
| **Tile Flip Animation** | Y-axis 3D flip, 600ms ciclo, front/back content |
| **Tile States** | Default, Hover, Focus, Pressed, Flipping |
| **Drag-Drop** | Reorder tiles (mouse/touch), visual feedback |
| **Long Press** | Context menu: Resize, Remove, Pin |
| **Keyboard Nav** | Tab/Shift+Tab, Arrow keys, Enter/Space = select |

### 2.3 PivotTabs — Navegação Horizontal

| Requisito | Detalhe |
|-----------|---------|
| **3 Tabs** | "Eventos" · "Apps" · "Configurações" |
| **Swipe Horizontal** | Touch drag + mouse wheel + arrow keys |
| **Active Indicator** | Animated underline (motion 250ms easing) |
| **Keyboard** | Left/Right = switch tab, Enter = activate |
| **Touch** | Swipe 50px threshold = switch |

### 2.4 Theme System

| Tema | Tokens Aplicados |
|------|------------------|
| **Light** | `color.surface.light`, `color.onSurface.light`, `color.primary.light` |
| **Dark** | `color.surface.dark`, `color.onSurface.dark`, `color.primary.dark` |
| **High Contrast** | `color.surface.hc`, `color.onSurface.hc`, `color.primary.hc` |
| **Toggle** | Cycle: Light → Dark → High Contrast → Light |
| **Persistência** | Salva em config (memory OK para PoC) |

### 2.5 Design Tokens (Fonte Única)

Criar `design-tokens.json` (compartilhado pelos 3 PoCs):

```json
{
  "color": {
    "primary": { "light": "#0066CC", "dark": "#A6C8FF", "hc": "#003399" },
    "surface": { "light": "#FFFFFF", "dark": "#121212", "hc": "#FFFFFF" },
    "onSurface": { "light": "#1C1B1F", "dark": "#E6E1E5", "hc": "#000000" },
    "outline": { "light": "#79747E", "dark": "#938F99", "hc": "#000000" }
  },
  "typography": {
    "fontFamily": "Inter Variable, Roboto Flex, Noto Sans Variable, system-ui",
    "scale": {
      "displayLarge": { "size": 57, "weight": 400, "lineHeight": 64 },
      "headlineMedium": { "size": 28, "weight": 500, "lineHeight": 36 },
      "titleLarge": { "size": 22, "weight": 500, "lineHeight": 28 },
      "bodyLarge": { "size": 16, "weight": 400, "lineHeight": 24 },
      "labelLarge": { "size": 14, "weight": 500, "lineHeight": 20 }
    }
  },
  "spacing": { "base": 4, "scale": [0,4,8,12,16,20,24,32,40,48,64] },
  "shape": { "cornerNone": 0, "cornerSmall": 4, "cornerMedium": 8, "cornerLarge": 16, "cornerFull": 9999 },
  "motion": { "durationShort": 100, "durationMedium": 200, "durationLong": 300, "easingStandard": "cubic-bezier(0.2, 0, 0, 1)" }
}
```

---

## 3. Data Models (Tipos Compartilhados)

```rust
// Compartilhado entre frameworks (mesma struct)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize { Small, Medium, Wide, Large }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub id: String,
    pub size: TileSize,
    pub title: String,
    pub front_content: TileContent,
    pub back_content: Option<TileContent>, // para flip
    pub badge: Option<u32>, // 1-99, None = sem badge
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileContent {
    Icon { name: String, color: String },
    Text { text: String, align: TextAlign },
    Image { url: String, fit: ImageFit },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode { Light, Dark, HighContrast }

#[derive(Debug, Clone)]
pub struct DesignTokens { /* campos do JSON acima */ }
```

---

## 4. Interaction Specifications

### 4.1 Touch / Mouse
| Ação | Trigger | Feedback Visual |
|------|---------|-----------------|
| Tap tile | Press + release < 300ms | Ripple (100ms) → select |
| Long press tile | Press > 500ms | Scale 0.95 → context menu |
| Drag tile | Press + move > 10px | Ghost tile + drop zones |
| Swipe pivot | Drag horizontal > 50px | Tab switch + indicator animation |
| Theme toggle | Tap button | Immediate theme swap (0ms) |

### 4.2 Keyboard (Desktop)
| Tecla | Ação |
|-------|------|
| `Tab` / `Shift+Tab` | Focus próximo/anterior |
| `Arrow Left/Right` | Navegar tiles / switch pivot |
| `Arrow Up/Down` | Navegar grid rows |
| `Enter` / `Space` | Ativar tile focado / toggle theme |
| `Escape` | Fechar modal / context menu |
| `Ctrl+T` | Toggle theme (shortcut) |

### 4.3 Reduced Motion
Respeita `prefers-reduced-motion: reduce`:
- Flip animation: **disabled** (instant swap)
- Pivot indicator: **instant** (sem animação)
- Ripple: **disabled**
- Theme swap: **instant**

---

## 5. Accessibility Requirements (WCAG 2.2 AA — Obrigatório)

### 5.1 Checklist Automatizado (axe-core)
```bash
# Deve retornar 0 violations
npx axe-cli http://localhost:3000 --save report.json
```

### 5.2 Requisitos Específicos da PoC
| Critério WCAG | Implementação PoC |
|---------------|-------------------|
| **1.1.1** Non-text Content | Todos tiles/icons têm `accessible_name` = title |
| **1.3.1** Info & Relationships | Grid usa `role="grid"`, tiles `role="gridcell"` |
| **1.4.3** Contrast (AA) | 4.5:1 texto, 3:1 UI components (verificado axe) |
| **1.4.4** Resize Text | 200% zoom mantém layout funcional |
| **1.4.11** Non-text Contrast | Focus ring 2px, 3:1 contrast |
| **1.4.13** Content on Hover/Focus | Tooltip em long-press/focus |
| **2.1.1** Keyboard | Toda funcionalidade via Tab/Arrows/Enter |
| **2.1.2** No Keyboard Trap | `Escape` sempre sai de modal/menu |
| **2.4.3** Focus Order | Visual = Tab order (esq→dir, cima→baixo) |
| **2.4.7** Focus Visible | 2px outline, 3:1 contrast, não apenas cor |
| **2.5.1** Pointer Gestures | Single-pointer alternative (click = drag) |
| **2.5.5** Target Size | Tiles ≥ 44×44dp (Small = 48×48dp mínimo) |
| **4.1.2** Name/Role/Value | Custom components com ARIA roles |

### 5.3 Screen Readers (Teste Manual)
- [ ] **NVDA** (Windows/Linux): Navegação grid + tiles anunciados
- [ ] **TalkBack** (Android): Swipe navigation + tile actions
- [ ] **Orca** (Linux): Grid navigation + tile actions

---

## 6. Build & Run Requirements (BLOQUEANTES)

### 6.1 Linux (x86_64)
```bash
# Build
cargo build --release --target x86_64-unknown-linux-gnu

# Run (Wayland)
./target/release/poc-name

# Run (X11)
GDK_BACKEND=x11 ./target/release/poc-name

# Critérios:
# - Janela abre em < 2s
# - Render 60fps sustained (perfetto/frame timing)
# - Resize window: grid reflow responsivo
# - Theme toggle: instant swap
# - Keyboard navigation: 100% funcional
```

### 6.2 Android (aarch64)
```bash
# Build (via cargo-ndk ou Gradle)
cargo build --release --target aarch64-linux-android
# OU ./gradlew assembleRelease

# Install
adb install -r app/build/outputs/apk/release/poc-name.apk

# Run
adb shell am start -n com.conexus.poc/.MainActivity

# Critérios:
# - APK instala sem erros
# - Launcher aparece na home
# - Abre em < 3s (cold start)
# - Render 60fps (GPU rendering)
# - Touch: swipe pivot, tap tiles, long press
# - Theme toggle: funciona
# - Back gesture: fecha app / navega
```

---

## 7. PoC Project Structure (Cada Framework)

```
poc-<framework>/
├── Cargo.toml
├── design-tokens.json          # SHARED - copiado igual nos 3
├── src/
│   ├── main.rs                 # Entry point + platform detection
│   ├── models.rs               # Tile, TileSize, ThemeMode, DesignTokens
│   ├── tokens.rs               # Carrega design-tokens.json
│   ├── home_screen.rs          # HomeScreen component (MAIN)
│   ├── apps_grid.rs            # AppsGridView + Tile components
│   ├── pivot_tabs.rs           # PivotTabs component
│   ├── theme_provider.rs       # ThemeProvider + toggle logic
│   └── platform/
│       ├── linux.rs            # Linux-specific (window, input)
│       └── android.rs          # Android-specific (JNI, lifecycle)
├── android/                    # APENAS se framework precisar
│   ├── build.gradle.kts
│   ├── src/main/java/.../MainActivity.kt
│   └── src/main/AndroidManifest.xml
├── .github/workflows/ci.yml    # CI matrix Linux + Android
└── README.md                   # Como build/run
```

---

## 7. Evaluation Scorecard (Template)

### 7.1 Binary Gate (PASS/FAIL — elimina framework)
| Critério | Slint | Iced | egui |
|----------|-------|------|------|
| Linux build ✅ |  |  |  |
| Android build ✅ |  |  |  |
| Linux run 60fps ✅ |  |  |  |
| Android run ✅ |  |  |  |
| Axe-core 0 violations ✅ |  |  |  |
| Theme toggle works ✅ |  |  |  |
| Pivot swipe works ✅ |  |  |  |
| Tile flip works ✅ |  |  |  |

### 7.2 Quality Score (0-10 cada — só se Binary Gate PASS)
| Critério | Peso | Slint | Iced | egui |
|----------|------|-------|------|------|
| Hot reload / iteration speed | 20% |  |  |  |
| Type safety (compile-time props) | 15% |  |  |  |
| Component library completeness | 15% |  |  |  |
| Animation performance (60fps) | 15% |  |  |  |
| Android integration maturity | 15% |  |  |  |
| Documentation / community | 10% |  |  |  |
| Licensing compatibility | 10% |  |  |  |
| **TOTAL** | **100%** |  |  |  |

---

## 8. Timeline & Entregas

| Semana | Atividade | Responsável | Entrega |
|--------|-----------|-------------|---------|
| **1** | Setup 3 repos PoC + design-tokens.json + CI | H2 | 3 repos prontos para dev |
| **1-2** | Implement HomeScreen + AppsGrid + PivotTabs + Theme | H2 | 3 PoCs funcionais (Linux) |
| **2-3** | Android build + test em device/emulator | H2 | 3 APKs instaláveis |
| **3** | A11y audit (axe-core + manual NVDA/TalkBack) | H5 | Relatórios a11y |
| **3** | Performance benchmark (frame timing, memory) | H3 | Métricas |
| **4** | Evaluation meeting + decision | H0/H1/H5 | **ADR-008 atualizado** |

---

## 8. Critérios de Eliminação Automática

Um framework é **eliminado imediatamente** se:
- ❌ Não compila para `aarch64-linux-android`
- ❌ Não roda no Android device/emulator (crash, ANR, blank screen)
- ❌ axe-core reporta **qualquer** violation WCAG 2.2 AA
- ❌ Não sustenta 60fps no Linux (p99 frame time > 16ms)
- ❌ Theme toggle não funciona (instant swap)
- ❌ Tile flip animation não roda (600ms Y-axis)
- ❌ Pivot swipe navigation não funciona

---

## 9. Entregáveis Finais da Fase 1

1. **3 repositórios PoC** (público no GitHub ou branch `poc/<framework>`)
2. **`docs/ui-framework-eval.md`** — relatório completo com:
   - Scorecard preenchido
   - Logs de build/run (Linux + Android)
   - Screenshots/vídeos
   - A11y reports
   - Benchmarks
3. **ADR-008 atualizado** para "Aceito" com framework escolhido
4. **Decisão documentada** com justificativa técnica baseada em evidência

---

## 10. Recursos para H2 (Implementer)

### Referências Rápidas
| Framework | Docs | Android Guide | Example Launcher |
|-----------|------|---------------|------------------|
| **Slint** | https://slint.dev/docs | https://github.com/slint-ui/slint/tree/master/examples/android | `slint-launcher` example |
| **Iced** | https://iced.rs/docs | https://github.com/iced-rs/iced/tree/master/examples/android | `iced-launcher` WIP |
| **egui** | https://docs.rs/egui | https://github.com/emilk/egui/tree/master/crates/egui_winit | `egui-launcher` WIP |

### Crates Úteis (Comuns)
```toml
# Comuns a todos
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
tracing = "0.1"
thiserror = "1"
async-trait = "0.1"

# Slint
slint = { version = "1.2", features = ["std"] }
# Android: slint-android = "0.1"

# Iced
iced = { version = "0.12", features = ["wgpu"] }
iced_wgpu = "0.12"
# Android: iced_wgpu + wgpu + winit + ndk-glue

# egui
egui = "0.28"
egui_winit = "0.28"
# Android: egui_winit + winit + ndk-glue
```

---

## 11. Aprovação para Iniciar

**Assinaturas necessárias para iniciar Fase 1:**

| Papel | Nome | Assinatura | Data |
|-------|------|------------|------|
| H0 Orchestrator |  |  |  |
| H1 Architect |  |  |  |
| H5 UX/UI |  |  |  |

---

**Próximo passo:** H2 (Implementer) cria 3 repositórios PoC e inicia implementação paralela.