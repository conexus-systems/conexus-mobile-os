# Skill: conexus-ux-2026
## UX/UI Design Intelligence para Conexus Harmattan 2026

**Versão:** 0.1  
**Compatibilidade:** OpenCode, agents `ux-ui-2026`, `validator`, `architect-rust`  
**Foco:** MeeGo Harmattan inspirado, atualizado para padrões 2026 (Material 3 Expressive, Fluent 2, iOS 17, Android 14+, WCAG 2.2 AA)

---

## 1. Heurísticas Expandidas (Nielsen + Mobile 2026 + Launcher)

### H1 — Visibilidade do Status do Sistema
- **Launcher:** Indicador visual de apps recentes, notificações pendentes, modo (dark/light), conectividade
- **Feedback:** ≤100ms para toque, ≤16ms frame, loading states skeleton
- **Offline/Online:** Banner sutil, não bloqueante

### H2 — Correspondência Mundo Real
- **Termos:** "Apps" não "Applications", "Pasta" não "Directory", "Widget" não "AppWidget"
- **Ícones:** Material Symbols / Fluent System Icons (reconhecíveis)
- **Metáforas:** Drag=pasta, swipe=navegar, long-press=ações, pinch=zoom

### H3 — Controle e Liberdade do Usuário
- **Undo global:** Toast com "Desfazer" (5s) para delete, move, uninstall
- **Dismiss:** Swipe down/back em sheets, dialogs, notifications
- **Cancel:** Botão visível em fluxos multi-step
- **Exit:** Home gesture/btn sempre acessível

### H4 — Consistência e Padrões
- **Tokens:** Single source of truth (`design-tokens.json`)
- **Component Library:** Storybook documentado
- **Plataforma:** Respeita Material 3 (Android) + Fluent (Linux) + Human Interface (iOS se futuro)

### H5 — Prevenção de Erros
- **Confirmação:** Destructive actions (uninstall, clear data, reset)
- **Validação inline:** Search, forms, config
- **Constraints:** Input masks, type-specific keyboards

### H6 — Reconhecimento em vez de Memorização
- **Affordances:** Ripple, elevation, hover states, tooltips
- **Search:** História, sugestões, categorias
- **Onboarding:** Progressive disclosure, não tutorial upfront

### H7 — Flexibilidade e Eficiência
- **Aceleradores:** Long-press=context menu, double-tap=top, shortcuts
- **Gestos:** Edge swipe, pull-to-refresh, drag-drop, pinch
- **Teclado:** Tab order, shortcuts (Ctrl+K=search, Esc=dismiss)

### H8 — Estética Minimalista
- **Whitespace:** 8dp base grid, densidade adaptável
- **Hierarquia:** Tipografia + cor + espaço (não apenas cor)
- **Clutter:** Max 3 níveis de navegação, 1 ação primária por tela

### H9 — Erros Recuperáveis
- **Mensagem:** O que + Por que + Como corrigir
- **Ação:** Botão "Tentar novamente", "Configurar", "Ajuda"
- **Logging:** Erro técnico oculto, acessível via "Detalhes"

### H10 — Ajuda e Documentação
- **Contextual:** Tooltip, hint text, empty state illustration
- **Buscável:** Help search integrado
- **Offline:** Cache local de help articles

### H11 — Gestos Mobile 2026 (Extensão)
- **Back gesture:** Preditivo (Android 14+), visual feedback
- **Home swipe:** Borda inferior → home
- **Multitask swipe:** Borda inferior + hold → overview
- **Quick switch:** Swipe horizontal na barra de navegação
- **Pull-down:** Notificações (top) / Search (home center)
- **Drag-drop:** Reorder, folders, widgets, cross-app (Android 12+)

### H12 — Large Screens / Desktop (Extensão)
- **Navigation rail:** ≥600dp width
- **Two-pane:** List + detail (settings, search results)
- **Drag-drop:** Entre janelas, apps
- **Keyboard shortcuts:** Completos (Ctrl+/ = search, etc)
- **Windowing:** Redimensionável, snap, maximize

---

## 2. Paletas de Cores 2026 (Dynamic + Static Fallback)

### Material You / Dynamic Color (Android 12+)
```json
{
  "scheme": "dynamic",
  "source": "wallpaper|user_picked|content",
  "tones": {
    "primary": [10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99],
    "secondary": [10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99],
    "tertiary": [10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99],
    "neutral": [10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99],
    "neutralVariant": [10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99],
    "error": [10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99]
  }
}
```

### Fallback Static (Conexus Brand)
```json
{
  "light": {
    "primary": "#0066CC",
    "onPrimary": "#FFFFFF",
    "primaryContainer": "#D6E4FF",
    "onPrimaryContainer": "#001D45",
    "secondary": "#4A637A",
    "onSecondary": "#FFFFFF",
    "surface": "#FFFBFE",
    "onSurface": "#1C1B1F",
    "outline": "#79747E"
  },
  "dark": {
    "primary": "#A6C8FF",
    "onPrimary": "#003366",
    "primaryContainer": "#004C91",
    "onPrimaryContainer": "#D6E4FF",
    "secondary": "#CCDDEE",
    "onSecondary": "#1D3347",
    "surface": "#1C1B1F",
    "onSurface": "#E6E1E5",
    "outline": "#938F99"
  }
}
```

### High Contrast (WCAG 2.2 AAA target)
```json
{
  "light": { "primary": "#003399", "surface": "#FFFFFF", "onSurface": "#000000" },
  "dark":  { "primary": "#CCDDEE", "surface": "#000000", "onSurface": "#FFFFFF" }
}
```

---

## 3. Pares de Fontes 2026 (Variable Fonts First)

| Uso | Primary | Fallback | Variable? |
|-----|---------|----------|-----------|
| **Display/Headline** | Inter Variable | Roboto Flex | ✅ wght 100-900 |
| **UI/Body** | Roboto Flex | Noto Sans Variable | ✅ wght+wdth+opsz |
| **Monospace** | JetBrains Mono Variable | Source Code Variable | ✅ wght |
| **Numeric** | Tabular Figures (Roboto Flex `tnum`) | — | ✅ |

**CSS/Slint usage:**
```css
font-family: "Inter Variable", "Roboto Flex", "Noto Sans Variable", system-ui, sans-serif;
font-variant-numeric: tabular-nums; /* para números */
font-optical-sizing: auto; /* responsive */
```

---

## 4. Guidelines de Charts & Data Viz (se aplicável)

| Tipo | Quando usar | A11y requerido |
|------|-------------|----------------|
| **Bar/Column** | Comparação categórica | Pattern fill + labels |
| **Line/Area** | Tendência temporal | Markers + tooltip |
| **Donut/Pie** | Partes do todo (max 5) | Labels + legend |
| **Scatter** | Correlação 2 vars | Axis titles + trendline |
| **Heatmap** | Densidade/matriz | Colorblind-safe palette |
| **Sparkline** | Inline trend | No axis, tooltip only |

**Paleta colorblind-safe (Okabe-Ito):**
`#000000, #E69F00, #56B4E9, #009E73, #F0E442, #0072B2, #D55E00, #CC79A7`

---

## 5. Presets GSAP / Motion (para protótipos Web/Slint)

```javascript
// Motion tokens
const motion = {
  easing: {
    standard: "cubic-bezier(0.2, 0, 0, 1)",
    emphasized: "cubic-bezier(0.2, 0, 0, 1)",
    decelerated: "cubic-bezier(0, 0, 0.2, 1)",
    accelerated: "cubic-bezier(0.4, 0, 1, 1)"
  },
  duration: {
    instant: 0,
    fast: 100,
    medium: 200,
    slow: 300,
    page: 400
  },
  presets: {
    "fade-in": { opacity: 0, duration: 100, ease: "standard" },
    "slide-up": { y: 20, opacity: 0, duration: 200, ease: "standard" },
    "scale-in": { scale: 0.95, opacity: 0, duration: 150, ease: "standard" },
    "slide-from-left": { x: -40, opacity: 0, duration: 250, ease: "decelerated" },
    "stagger-children": { stagger: 0.05, ease: "standard" }
  }
};
```

**Reduced Motion:** Respeita `prefers-reduced-motion: reduce` → durations = 0, easing = linear.

---

## 6. Metodologia de Auditoria UX (Passo a Passo)

### Fase 1: Preparação
1. Coletar: build atual, design tokens, component library, specs
2. Definir: personas, tarefas críticas, dispositivos alvo
3. Preparar: checklists, ferramentas (axe, storybook, device farm)

### Fase 2: Auditoria Automatizada
```bash
# A11y
npx axe-cli http://localhost:3000 --save report.json

# Visual regression (se baseline existe)
npx playwright test --update-snapshots

# Token compliance
node scripts/check-tokens.js src/
```

### Fase 3: Auditoria Manual (Checklist)
| Categoria | Itens | Tempo estimado |
|-----------|-------|----------------|
| Visual/Interaction | 25 checks | 45 min |
| Acessibilidade (WCAG 2.2 AA) | 35 checks | 60 min |
| Conteúdo/Copy | 15 checks | 30 min |
| Performance UX | 10 checks | 20 min |
| Cross-platform | 15 checks | 30 min |

### Fase 4: Testes com Usuários (se build funcional)
- **Recrutamento:** 5 usuários (mistos)
- **Cenários:** 8 tarefas críticas
- **Métricas:** Taxa sucesso, tempo, erros, SUS score
- **Pass:** ≥80% sucesso nas tarefas críticas

### Fase 5: Relatório
```markdown
# UX Audit Report — Conexus Launcher vX.Y.Z

## Resumo Executivo
- Score geral: X/100
- Bloqueadores P0: N
- Recomendações P1: N
- Melhorias P2: N

## Por Categoria
### Visual/Interaction: X/25 ✅
### Acessibilidade: X/35 ⚠️
### Conteúdo: X/15 ✅
### Performance: X/10 ✅
### Cross-platform: X/15 ⚠️

## Bloqueadores (P0)
1. [Descrição + arquivo:linha + screenshot]
2. ...

## Recomendações Priorizadas
| Prioridade | Item | Esforço | Impacto |
|------------|------|---------|---------|
| P0 | ... | S | Alto |
| P1 | ... | M | Médio |
| P2 | ... | L | Baixo |

## Próximos Passos
1. Corrigir P0 antes de merge
2. P1 na próxima sprint
3. P2 backlog
```

---

## 7. Referências Rápidas (Links)

| Referência | Link |
|------------|------|
| WCAG 2.2 | https://www.w3.org/TR/WCAG22/ |
| Material 3 Expressive | https://m3.material.io/ |
| Fluent 2 Design | https://fluent2.microsoft.design/ |
| Android 14 UX | https://developer.android.com/design |
| iOS 17 HIG | https://developer.apple.com/design/human-interface-guidelines |
| WCAG Quick Reference | https://www.w3.org/WAI/WCAG22/quickref/ |
| axe-core | https://github.com/dequelabs/axe-core |
| Material Symbols | https://fonts.google.com/icons |
| Fluent System Icons | https://github.com/microsoft/fluentui-system-icons |
| Inter Variable Font | https://github.com/rsms/inter |
| Roboto Flex | https://github.com/googlefonts/roboto-flex |
| Okabe-Ito Palette | https://jfly.uni-koeln.de/color/ |

---

## 8. Uso no Agente

```markdown
# Antes de validar PR/build:
skill conexus-ux-2026

# Então execute checklist da skill:
- Visual/Interaction: 25 items
- Acessibilidade WCAG 2.2 AA: 35 items  
- Conteúdo: 15 items
- Performance UX: 10 items
- Cross-platform: 15 items

# Gere relatório padrão
# Anexe screenshots anotados
# Classifique: P0 (bloqueia), P1 (próxima sprint), P2 (backlog)
```

---

## 9. Versionamento da Skill

| Versão | Data | Mudanças |
|--------|------|----------|
| 0.1 | 2026-10-22 | Versão inicial — Harmattan 2026 baseline |

**Próximas atualizações:** Adicionar testes automatizados a11y, visual regression baseline, device farm integration.