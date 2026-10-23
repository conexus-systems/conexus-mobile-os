# ADR-008: Framework de UI Selecionado por Evidência

**Status:** Pendente — Requer Prova de Conceito (Fase 1)  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect), H5 (UX/UI)

---

## Contexto

A interface do Conexus Harmattan deve ser implementada em Rust (ADR-003). Existem três frameworks viáveis: **Slint**, **Iced** e **egui**. A escolha tem impacto profundo em arquitetura, performance, ecossistema, e capacidade de rodar no Android.

## Decisão

**Não fixar framework antes de prova de conceito.** Selecionar **Slint**, **Iced** ou **egui** após construir uma tela equivalente (Home Grid + Navegação + Tema) que **compile e execute em Linux (Wayland/X11) e Android (NDK)**.

## Critérios de Avaliação (Obrigatórios)

### 1. Cross-Platform Build & Run (BLOQUEANTE)
| Critério | Linux (x86_64/aarch64) | Android (aarch64) |
|----------|------------------------|-------------------|
| `cargo build` | ✅ | ✅ |
| Executa nativo | ✅ (Wayland/X11) | ✅ (APK instala, roda) |
| Render backend | OpenGL/Vulkan/Software | OpenGL ES / Vulkan |
| Windowing | Wayland, X11, winit | NDK native activity / winit |

### 2. UI Capabilities (Launcher Requirements)
| Feature | Requirement |
|---------|-------------|
| **Grid Layout** | Responsivo (360dp–1440dp), drag-drop, reorder, folders |
| **Navigation** | Pivôs horizontais, edge swipes, back gesture |
| **Components** | Button, Card, Tile (S/M/W/L flip), List, SearchBar, BottomSheet, NavigationRail |
| **Theming** | Dark/Light/High Contrast + Design Tokens JSON + Dynamic Color (Android) |
| **Animation** | 60fps, 120/250/350ms, easing, prefers-reduced-motion |
| **Gestures** | Touch, mouse, keyboard, stylus, edge swipes |
| **Acessibilidade** | WCAG 2.2 AA (axe-core 0 violations), screen readers, focus management |

### 3. Developer Experience
| Critério | Peso |
|----------|------|
| Hot reload / fast iteration | Alto |
| Type-safe UI (compile-time props) | Alto |
| Debugging tools (inspector, profiler) | Médio |
| Documentation / community | Médio |
| Learning curve | Baixo |

### 4. Licensing & Sustainability
| Critério | Requirement |
|----------|-------------|
| Licença | MIT / Apache-2.0 (compatível Apache-2.0 projeto) |
| Commercial support | Opcional (Slint tem) |
| Long-term maintenance | Ativo, releases regulares |

---

## Candidatos

### Slint
| Aspecto | Detalhes |
|---------|----------|
| **Linguagem** | `.slint` (DSL) + Rust |
| **Linux** | ✅ Native (OpenGL, Vulkan, Software) |
| **Android** | ✅ NDK (OpenGL ES 3+, Vulkan) |
| **Licença** | MIT (open source) / Commercial (enterprise) |
| **Maturidade** | Alta (1.0+ releases, usado em produção) |
| **Acessibilidade** | ✅ ARIA, screen readers, focus |
| **Hot Reload** | ✅ `slint-viewer` + live preview |
| **Type Safety** | ✅ Compile-time property validation |
| **Android Integration** | ✅ NDK, `ndk-glue`, `slint-android` crate |
| **Riscos** | Licença dual (commercial features), DSL learning curve |

### Iced
| Aspecto | Detalhes |
|---------|----------|
| **Linguagem** | Rust puro (Elm architecture) |
| **Linux** | ✅ wgpu (Vulkan/Metal/OpenGL) |
| **Android** | ⚠️ Experimental (wgpu-android, winit-android) |
| **Licença** | MIT |
| **Maturidade** | Média (0.x, breaking changes freq) |
| **Acessibilidade** | ⚠️ Parcial (em desenvolvimento) |
| **Hot Reload** | ⚠️ Via `cargo watch` + recompile |
| **Type Safety** | ✅ Rust nativo |
| **Android Integration** | ⚠️ `winit` + `ndk-glue` + `wgpu` — instável |
| **Riscos** | Android support imaturo, API instável |

### egui
| Aspecto | Detalhes |
|---------|----------|
| **Linguagem** | Rust puro (immediate mode) |
| **Linux** | ✅ wgpu/glow/epi |
| **Android** | ⚠️ Via `egui-winit` + `ndk-glue` |
| **Licença** | MIT |
| **Maturidade** | Média (0.x, amplamente usado em tools) |
| **Acessibilidade** | ⚠️ Básico (em desenvolvimento) |
| **Hot Reload** | ⚠️ Recompile necessário |
| **Type Safety** | ✅ Rust nativo |
| **Android Integration** | ⚠️ `winit` + `egui-winit` — funcional mas limitado |
| **Riscos** | Immediate mode = frame allocation, Android winit instável |

---

## Processo de Decisão (Fase 1)

### Semana 1–2: Setup PoC
```bash
# Cada framework: repo separado ou branch
git clone <framework-poc>
cd slint-poc / iced-poc / egui-poc

# 1. Setup workspace
cargo new conexus-ui-poc
# Adicionar deps do framework

# 2. Implementar tela HomeGrid
# - Grid 4 colunas (phone) / 6 (tablet)
# - 3 tiles: Small, Medium, Wide
# - Navegação: TabBar (Apps, Widgets, Settings)
# - Tema: Dark/Light toggle
# - Design tokens aplicados
```

### Semana 3: Cross-Platform Build
```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu
./target/release/conexus-ui-poc  # Deve abrir janela, render 60fps

# Android
cargo build --release --target aarch64-linux-android
# Ou via Gradle + cargo-ndk
# Gerar APK, instalar em device/emulator
adb install conexus-ui-poc.apk
# Deve aparecer no launcher, abrir, render
```

### Semana 4: Validation & Decision
| Teste | Slint | Iced | egui |
|-------|-------|------|------|
| Linux build | ✅/❌ | ✅/❌ | ✅/❌ |
| Android build | ✅/❌ | ✅/❌ | ✅/❌ |
| Linux run (60fps) | ✅/❌ | ✅/❌ | ✅/❌ |
| Android run (device) | ✅/❌ | ✅/❌ | ✅/❌ |
| A11y (axe-core) | ✅/❌ | ✅/❌ | ✅/❌ |
| Theme toggle | ✅/❌ | ✅/❌ | ✅/❌ |
| Gestures (swipe) | ✅/❌ | ✅/❌ | ✅/❌ |
| Hot reload | ✅/❌ | ✅/❌ | ✅/❌ |

**Critério de Pass:** Framework deve ter ✅ em **todas** linhas acima.

---

## Decisão Final

**Resultado do PoC documentado em `docs/ui-framework-eval.md`:**
- Framework selecionado: **[SLINT / ICED / EGU]**
- Justificativa técnica: [baseada em evidência]
- Riscos conhecidos e mitigações
- Migration path se mudar no futuro

**ADR-008 atualizado para "Aceito" com framework escolhido.**

---

## Fallback Strategy

Se **nenhum** framework passar todos os critérios:
1. **Slint** como default (melhor Android support)
2. **egui** para protótipos/tools internos
3. **Reavaliar** em 6 meses (ecossistema Rust UI evolui rápido)

---

**Assinatura:** H0, H1, H5  
**Próxima revisão:** Fim da Fase 1 (PoC completa) — **BLOQUEIA Fase 2**