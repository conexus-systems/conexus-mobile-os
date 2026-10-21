# ADR-006: Fluent Design System como Visual Language — APROVADO

**Status:** Aceito
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Shell (A3), Framework (A4)
**Assinatura:** A0

---

## Contexto

O Windows 10 Mobile tinha identidade visual distinta: tipografia Segoe, motion system, Live Tiles, acrilico/blur, navegacao baseada em pivot/panorama.

Em 2026, o Fluent Design System (Microsoft) evoluiu para:
- Fluent 2 (Windows 11): Mica/Acrylic materials, rounded corners, updated typography, new iconography
- Cross-platform: Fluent UI React, Fluent UI Apple, Fluent UI Web Components
- Design tokens: Cores, tipografia, spacing, motion, elevation, z-index
- Acessibilidade integrada: High contrast, reduced motion, screen reader

Objetivo: Evoluir a experiencia Windows Mobile para 2026, nao replicar 2015 pixel-perfect.

---

## Decisao

Adotar Fluent Design System (Fluent 2) como visual language base, com adaptacoes Conexus.

### Tokens Obrigatorios (M1)
| Categoria | Tokens | Fonte |
|-----------|--------|-------|
| Color | Light/Dark/High Contrast palettes, semantic aliases (bg, fg, border, accent) | Fluent 2 + Conexus brand |
| Typography | Inter (OFL) primary, Noto Sans fallback/CJK, type ramp (display, title, body, caption) | Fluent 2 + open source |
| Spacing | 4px base unit, scale (0, 1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64) | Fluent 2 |
| Motion | Duration tokens (fast: 100ms, normal: 200ms, slow: 300ms), easing (standard, accelerate, decelerate) | Fluent 2 |
| Elevation | 0-4 levels, shadow tokens, Mica/Acrylic background treatments | Fluent 2 |
| Border Radius | 2px, 4px, 8px, 12px, 16px, 28px (circular) | Fluent 2 |
| Iconography | Fluent System Icons (MIT licensed), 16/20/24/28/32/48 | Fluent 2 |

### Adaptacoes Conexus (nao-Fluent)
- Live Tiles: Grid system proprio (4col base, resize handles, content templates)
- Start Screen: Pivot/panorama navigation pattern (Windows Mobile heritage)
- Action Center: Sheet-based, not flyout (mobile optimized)
- Keyboard: Layout proprio inspirado no Windows 10 Mobile (shape writing, cursor control)
- Brand Colors: Conexus accent color override em Fluent palette

### Implementacao
- Design Tokens: JSON + TypeScript definitions em shell/fluent/tokens/
- Kotlin/Jetpack Compose: shell/fluent/compose/ — Compose theme + components
- TypeScript/React Native: shell/fluent/rn/ — React Native components + theme
- Shared: framework/sdk/*/theming/ consome tokens

---

## Consequencias

### Positivas
- Design system maduro, acessivel, cross-platform
- Tokens versionados = consistencia garantida
- Fluent 2 = visual 2026 nativo, nao datado
- Open source (MIT) — sem licenciamento issues
- Ecossistema de componentes existentes para reference

### Negativas
- Fluent e "Microsoft look" — pode parecer derivado
- Mica/Acrylic em mobile = GPU/battery cost (precisa medicao)
- Customizacoes (Live Tiles, Start) exigem componentes fora do Fluent padrao
- Segoe UI Variable nao e open source (fallback: Inter)

### Riscos
- Performance: Acrylic/Mica em device mid-range — Mitigacao: M3/M4 battery profiling, fallback solids
- Legal: Segoe font licensing — Mitigacao: Usar Inter (OFL) ou Noto Sans como primary, Segoe so se licenciado
- Differentiation: Parece "Windows 11 mobile" — Mitigacao: Live Tiles, Start, Keyboard como differentiators unicos

---

## Alternativas Consideradas

| Alternativa | Pros | Contras | Por que nao |
|-------------|------|---------|-------------|
| Material Design 3 | Mature, Android native, tokens | "Android look", nao diferencia Windows Mobile heritage | Perde identidade unica |
| Custom design system do zero | Total control, unico | Enorme effort, acessibilidade, tokens, motion, icons do zero | Reinventar roda |
| iOS Human Interface | Consistent, polished | Nao e Android/Linux, licensing | Plataforma errada |
| Windows 10 Mobile visual (2015) | Autentico | Dated, nao escala para 2026 screen sizes, no dark mode proper | Nao evolui |

---

## Plano de Implementacao

1. M1: Design tokens JSON + TS definitions (shell/fluent/tokens/)
2. M1: Compose theme + base components (Button, Card, Tile, Typography)
3. M1: RN theme + base components (parity com Compose)
4. M2: Live Tile components (grid, resize, content templates) — Conexus-specific
5. M3: Start Screen, Action Center, Keyboard — heritage components
6. M4: Accessibility audit (TalkBack, scaling, high contrast, reduced motion)

---

## Referencias

- Fluent 2 Design Tokens (https://github.com/microsoft/fluentui/tree/master/packages/design-tokens)
- Fluent UI React Native (https://github.com/microsoft/fluentui-react-native)
- Fluent System Icons (https://github.com/microsoft/fluentui-system-icons)
- SDD v0.1 4.2
- CONEXUS brand identity (../brand-identity/)