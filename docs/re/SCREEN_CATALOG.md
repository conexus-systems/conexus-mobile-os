# Screen Catalog — Windows 10 Mobile Visual Reference

**Versão:** v0.1  
**Data:** 19 de outubro de 2026  
**Status:** Template — capturas de tela a adicionar M0/M1

---

## Metodologia

- Device: Lumia 950 XL (1440x2560, 564 ppi)
- Orientação: Portrait (primary) + Landscape
- Tema: Light + Dark + High Contrast
- Escala: 100%, 125%, 150%, 200% (text scaling)
- Idioma: pt-BR (primary) + en-US

Cada screen deve ter:
1. **Screenshot** (referência visual)
2. **Wireframe anotado** (layout, grids, spacing)
3. **Specs** (cores, tipografia, spacing, tokens Fluent mapeados)
4. **Estados** (default, focused, pressed, disabled, loading, error)
5. **Variações** (orientação, tema, escala, idioma RTL)

---

## Telas do Start

### START-01: Tela Iniciar — Default (Portrait, Light, 100%)
- **Layout:** Grid 4 colunas, margem 24px lateral, 16px topo/baixo
- **Tiles:** Mix de tamanhos (small 1x1, medium 2x2, wide 4x2, large 4x4)
- **Grupos:** Headers com label, espaçamento 32px entre grupos
- **Scroll:** Vertical, snap-to-tile opcional
- **Tokens Fluent:** `spacing-24`, `spacing-16`, `spacing-32`, `type-ramp-body1`, `color-bg-primary`, `color-fg-primary`

### START-02: Tela Iniciar — Landscape
- **Layout:** Grid 6 colunas, mesma margem
- **Reflow:** Tiles rearranjam mantendo grupos

### START-03: Tela Iniciar — Dark Theme
- **Cores:** `color-bg-primary` → `color-bg-dark`, `color-fg-primary` → `color-fg-dark`
- **Tiles:** Acrílico/Background adaptado

### START-04: Tela Iniciar — High Contrast
- **Bordas:** 2px em todos tiles
- **Cores:** Sistema high contrast palette

### START-05: All Apps List
- **Layout:** Lista alfabética, jump list lateral (A-Z), search no topo
- **Item:** Ícone 24px + label, height 48px, divider 1px
- **Groups:** Letra header sticky

### START-06: Search Overlay
- **Layout:** Full screen overlay, search box centralizado topo
- **Results:** Cards com ícone, nome, categoria, ação
- **Animation:** Fade-in 150ms, stagger results 50ms

---

## Telas de Tiles (Estados por Tamanho)

### TILE-SM: Small (1x1) — 72x72px (100%)
| Estado | Specs |
|--------|-------|
| Default | Ícone 32px centrado, badge top-right 16px |
| Pressed | Scale 0.92, background `color-bg-pressed` |
| Badge | Count 1-99, "99+" se >99, cor `color-accent` |
| Disabled | Opacity 0.4, no interaction |

### TILE-MD: Medium (2x2) — 168x168px (100%)
| Estado | Specs |
|--------|-------|
| Default | Conteúdo: ícone 48px + texto 14px ou imagem fill |
| Flip Front | App name + live content (text/image) |
| Flip Back | Extended info (description, status) |
| Flip Animation | 3D rotate Y 600ms, easing `cubic-bezier(0.4, 0, 0.2, 1)` |
| Pressed | Scale 0.96, subtle shadow elevation+1 |

### TILE-WD: Wide (4x2) — 360x168px (100%)
| Estado | Specs |
|--------|-------|
| Default | Layout horizontal: imagem 50% + texto 50% ou 2x medium side-by-side |
| Flip | Mesmo princípio, conteúdo estendido |

### TILE-LG: Large (4x4) — 360x360px (100%)
| Estado | Specs |
|--------|-------|
| Default | Full content: mapa, álbum, player controls |
| Internal Scroll | Vertical scroll dentro do tile, parallax header |
| Pressed | Subtle scale + elevation |

---

## Telas de Navegação

### NAV-01: Task Switcher
- **Layout:** Cards horizontais, 70% viewport width, gap 16px
- **Card:** Screenshot preview (aspect ratio device), app name bottom
- **Gesture:** Swipe horizontal, snap to card, dismiss swipe up
- **Animation:** Fan-out 300ms, dismiss 200ms + fade

### NAV-02: Back Transition (Page)
- **Animation:** Slide right 200ms, incoming page slide from left 200ms
- **Easing:** `cubic-bezier(0.4, 0, 0.2, 1)`

### NAV-03: Pivot Header (App Pattern)
- **Layout:** Header fixo com tabs, conteúdo swipe horizontal
- **Tab Indicator:** Line 3px, `color-accent`, slide 200ms
- **Content:** Lazy load adjacent pivots

---

## Telas da Action Center

### ACT-01: Notification Panel (Closed → Open)
- **Closed:** Handle top-center 4px, swipe down threshold 20%
- **Open:** Sheet 90% height, rounded top 16px, backdrop blur
- **Sections:** Quick Actions (top) + Notifications (scrollable)

### ACT-02: Quick Actions Grid
- **Layout:** 4 colunas, 4 linhas max, scroll horizontal se >4
- **Tile:** Ícone 28px + label 12px, toggle state color
- **Long Press:** Navigate para settings page correspondente

### ACT-03: Notification Card
- **States:** Collapsed (1 line) → Expanded (full content + actions)
- **Actions:** Buttons com ícone + label, max 3
- **Dismiss:** Swipe right → dismiss animation 200ms

---

## Telas de Configurações

### SET-01: Settings Home
- **Layout:** Grid 2 colunas de categorias, cards com ícone + label
- **Search:** Top persistent, 48px height
- **Categories:** Sistema, Dispositivos, Rede, Personalização, Contas, Hora, Privacidade, Atualização

### SET-02: Category Page (ex: Personalização)
- **Layout:** List sections com dividers
- **Controls:** Toggles, sliders, navigation links, pickers
- **Breadcrumb:** Top-left "Settings > Personalization"

### SET-03: Settings Search Results
- **Layout:** List grouped by category, highlight match
- **Action:** Tap → navega para setting + highlight temporário

---

## Telas do Teclado

### KEY-01: QWERTY Layout (pt-BR)
- **Rows:** 4 rows + space row
- **Keys:** 10% gap, min touch target 48x48dp
- **Long Press:** Popup 200ms com accents/symbols
- **Shift States:** Lower → Upper → Caps Lock (double tap)

### KEY-02: Symbols/Numbers Layout
- **Switch:** `?123` key toggles layout
- **Long Press:** Extended symbols (currency, math, etc.)

### KEY-03: Shape Writing Trail
- **Visual:** Curved path following finger, color `color-accent`, width 4px
- **Prediction:** Word appears above keyboard, tap to accept

### KEY-04: Cursor Control (Magnifier)
- **Trigger:** Long press on text field (500ms)
- **Magnifier:** Circle 80dp, 2x zoom, follows finger
- **Cursor:** Vertical line 2px, `color-accent`

---

## Telas de Bloqueio

### LOCK-01: Lock Screen — Default
- **Clock:** Top center, `type-ramp-display` (48pt), time + date
- **Background:** User image ou Windows Spotlight
- **Notifications:** Bottom sheet, max 3 visible, "Show more" link

### LOCK-02: Lock Screen — PIN Entry
- **Layout:** Numpad 3x4 + backspace, center bottom
- **Feedback:** Ripple per digit, dot indicator top
- **Error:** Shake + "Incorrect PIN" toast, haptic

### LOCK-03: Lock Screen — Camera Quick Launch
- **Gesture:** Swipe up from bottom-right camera icon
- **Animation:** Camera UI slides up 300ms

---

## Mapeamento para Fluent Tokens

| Windows 10 Mobile | Fluent 2 Token | Conexus Override |
|-------------------|----------------|------------------|
| System Accent | `color-brand-foreground` | `color-conexus-accent` |
| Tile Background | `color-bg-secondary` | — |
| Text Primary | `color-fg-primary` | — |
| Text Secondary | `color-fg-secondary` | — |
| Divider | `color-border-default` | — |
| Focus Border | `color-border-focus` | 2px solid |
| Hover/Pressed | `color-bg-hover` / `color-bg-pressed` | — |
| Elevation 1 | `shadow-4` | Tile default |
| Elevation 2 | `shadow-8` | Pressed/Active |
| Elevation 3 | `shadow-16` | Modal/Sheet |
| Corner Radius 2px | `border-radius-2` | Small tiles |
| Corner Radius 4px | `border-radius-4` | Medium tiles, buttons |
| Corner Radius 8px | `border-radius-8` | Cards, sheets |
| Corner Radius 12px | `border-radius-12` | Large tiles, modals |

---

## Checklist de Captura (M0/M1)

- [ ] START-01 a START-06 (all themes, orientations, scales)
- [ ] TILE-SM/MD/WD/LG (all states)
- [ ] NAV-01 a NAV-04
- [ ] ACT-01 a ACT-03
- [ ] SET-01 a SET-03
- [ ] KEY-01 a KEY-04
- [ ] LOCK-01 a LOCK-03

**Armazenamento:** `docs/re/screenshots/` (git LFS) + Figma/Zeplin link para specs