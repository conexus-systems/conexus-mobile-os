# ADR-003: Interface Implementada em Rust

**Status:** Aceito; framework pendente de prova de conceito  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect), H5 (UX/UI)

---

## Contexto

A interface do Conexus Harmattan deve ser inspirada no MeeGo Harmattan (N9), modernizada para 2026 (Material 3 Expressive, Fluent 2, dynamic color). A stack histórica Qt/QML não é alvo — o objetivo é Rust-native para compartilhar código com core e evitar dependências C++/Qt.

## Decisão

**Reconstruir a interface com um framework gráfico compatível com Rust**, selecionado por prova de conceito (ADR-008). Não usar Qt/QML no produto final.

## Consequências

### Positivas
- **Stack unificada:** Core + UI + Platform = 100% Rust
- **Type safety:** UI components tipados, props validadas em compile time
- **Hot reload:** Frameworks modernos suportam dev iteration rápida
- **Acessibilidade nativa:** Slint/Iced/egui têm suporte a11y (ARIA, screen readers)
- **Cross-platform real:** Mesmo binário UI roda Linux + Android (via NDK)

### Negativas
- **Ecossistema menor:** Menos componentes prontos que Qt/QML
- **Framework risk:** Escolha errada = retrabalho massivo (mitigado por ADR-008 PoC)
- **Android integration:** Requer NDK, OpenGL ES/Vulkan, JNI para system integration

## Frameworks Candidatos

| Framework | Linux | Android | Licença | Maturidade | Rust-Native |
|-----------|-------|---------|---------|------------|-------------|
| **Slint** | ✅ OpenGL/Vulkan/Software | ✅ NDK (OpenGL ES) | MIT/Commercial | Alta | ✅ |
| **Iced** | ✅ wgpu | ⚠️ Experimental | MIT | Média | ✅ |
| **egui** | ✅ wgpu/glow | ⚠️ winit/ndk-glue | MIT | Média | ✅ |

## Critérios de Seleção (ADR-008)

1. **PoC Obrigatória:** Mesma tela (Home Grid + Nav + Theme) compila e executa em Linux + Android
2. **Acessibilidade:** WCAG 2.2 AA baseline (axe-core 0 violations)
3. **Performance:** 60fps sustained, <16ms frame, <100ms interaction feedback
4. **Component Library:** Buttons, Cards, Grids, Navigation, Search, BottomSheets
5. **Theming:** Dark/Light/High Contrast + Dynamic Color (Material You) + tokens JSON
5. **Gestures:** Touch, mouse, keyboard, edge swipes
6. **Licença:** MIT/Apache-2.0 preferido (compatible com Apache-2.0 projeto)

## Qt/QML — Papel no Projeto

| Uso | Permitido? | Justificativa |
|-----|------------|---------------|
| **Dependência runtime** | ❌ | Não é alvo; adiciona C++, moc, qml cache |
| **Referência visual/comportamental** | ✅ | `nemomobile/qt-components`, `glacier-home` como design reference |
| **Protótipo rápido (descartável)** | ✅ | Apenas para validar UX antes de reimplementar em Rust |
| **Componentes legados reutilizados** | ❌ | LGPL-2.1 + stack mismatch |

## Validação

- [ ] ADR-008 PoC completa: framework selecionado com evidência
- [ ] `conexus-ui` crate criada com component library
- [ ] Design tokens (`design-tokens.json`) integrados
- [ ] Storybook/documentação de componentes
- [ ] A11y audit: axe-core 0 violations

---

**Assinatura:** H0, H1, H5  
**Próxima revisão:** Após ADR-008 resolvido (Fase 1 completa)