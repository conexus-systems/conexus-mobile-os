# Matriz de Repositórios de Referência — MeeGo Harmattan

**Projeto:** Conexus Harmattan  
**Data:** 22 de outubro de 2026  
**Fase:** 0 — Auditoria  
**Base:** Repositórios GitHub públicos da organização `harmattan` e relacionados

---

## 1. Organização `harmattan` (GitHub)

| Repositório | Descrição | Linguagem | Licença | Último Commit | Status | Reutilização |
|-------------|-----------|-----------|---------|---------------|--------|--------------|
| `harmattan/kernel-plus-harmattan` | Kernel patches para N9 (OMAP3) | C, Assembly | GPL-2.0 | 2013 | Archived | **Referência apenas** — kernel histórico, não compatível com Android moderno |
| `harmattan/profilematic` | Sistema de perfis contextuais (localização, hora, wifi) | Python, QML | BSD-3-Clause | 2012 | Archived | **Referência comportamental** — lógica de perfis inspirada |
| `harmattan/syncui` | UI de sincronização de contas | QML, C++ | LGPL-2.1 | 2012 | Archived | **Referência UI** — patterns de sync |
| `harmattan/accounts-ui` | UI de gerenciamento de contas | QML, C++ | LGPL-2.1 | 2012 | Archived | **Referência UI** — account management patterns |

---

## 2. Projetos Comunitários Relacionados

### 2.1 `ajalkane` (Jussi A.)

| Repositório | Descrição | Linguagem | Licença | Último Commit | Status | Reutilização |
|-------------|-----------|-----------|---------|---------------|--------|--------------|
| `ajalkane/profilematic` | Fork/continuação do profilematic | Python, QML | BSD-3-Clause | 2014 | Archived | **Referência** — regras de perfil |
| `ajalkane/toggleavailability` | Quick toggles (wifi, bt, rotation, etc) | QML, C++ | BSD-3-Clause | 2013 | Archived | **Referência UI** — toggles, quick actions |
| `ajalkane/meego-keyboard` | Teclado virtual MeeGo | C++, QML | BSD-3-Clause | 2013 | Archived | **Referência** — keyboard layout/gestures |

### 2.2 `rschiang`

| Repositório | Descrição | Linguagem | Licença | Último Commit | Status | Reutilização |
|-------------|-----------|-----------|---------|---------------|--------|--------------|
| `rschiang/moedict-meego` | Dicionário MOE para MeeGo | QML, JavaScript | GPL-3.0 | 2012 | Archived | **Referência app** — app structure |

---

## 3. `nemomobile` — Ativo e Relevante

| Repositório | Descrição | Linguagem | Licença | Último Commit | Status | Reutilização |
|-------------|-----------|-----------|---------|---------------|--------|--------------|
| `nemomobile/qt-components` | Qt Components (Buttons, Layouts, Navigation, etc) | C++, QML | **LGPL-2.1** | 2024 | **Active** | **Referência visual/funcional** — component API design |
| `nemomobile/lipstick` | Home screen / compositor (Wayland) | C++, QML | LGPL-2.1 | 2024 | Active | **Referência arquitetural** — shell composition |
| `nemomobile/qtquickcontrols-nemo` | Qt Quick Controls com tema Nemo | QML, C++ | LGPL-2.1 | 2023 | Active | **Referência componentes** |
| `nemomobile/glacier-home` | Glacier UX home screen | QML, C++ | LGPL-2.1 | 2024 | Active | **Referência UX** — home screen patterns |

---

## 4. Análise Detalhada por Projeto

### 4.1 `nemomobile/qt-components` — MAIOR RELEVÂNCIA

**Descrição:** Implementação de componentes UI reutilizáveis para Qt/QML, baseados nos componentes originais do MeeGo/Harmattan.

**Estrutura principal:**
```
qt-components/
├── src/
│   ├── components/
│   │   ├── Button.qml
│   │   ├── ToolButton.qml
│   │   ├── Switch.qml
│   │   ├── Slider.qml
│   │   ├── ProgressBar.qml
│   │   ├── Page.qml
│   │   ├── PageStack.qml
│   │   ├── ListView.qml
│   │   ├── GridView.qml
│   │   ├── TextField.qml
│   │   ├── TextArea.qml
│   │   ├── Label.qml
│   │   ├── Icon.qml
│   │   ├── Divider.qml
│   │   ├── ScrollDecorator.qml
│   │   └── ...
│   ├── layout/
│   │   ├── GridLayout.qml
│   │   └── FlowLayout.qml
│   ├── navigation/
│   │   ├── TabBar.qml
│   │   ├── ToolBar.qml
│   │   └── NavigationBar.qml
│   └── styling/
│       └── Theme.qml
```

**Componentes-chave mapeados para Harmattan:**

| Componente Nemo | Equivalente Harmattan | Status para Conexus |
|-----------------|----------------------|---------------------|
| `Button` / `ToolButton` | Botões de ação, toolbar | ✅ Referência API |
| `Switch` | Toggles (wifi, bt, etc) | ✅ Referência comportamento |
| `Page` / `PageStack` | Navegação baseada em pilha | ✅ Referência navegação |
| `ListView` / `GridView` | Lists, grids de apps | ✅ Referência layout |
| `TabBar` | Pivôs (Events/Apps/Settings) | ✅ Referência pivôs |
| `Theme` | Tema centralizado | ✅ Referência theming |

**Licença LGPL-2.1 — Implicações:**
- ✅ Pode linkar dinamicamente (não é o caso — alvo é Rust)
- ✅ Pode usar como **referência de design/API** sem obrigações
- ❌ **Não pode** copiar código QML/C++ diretamente para distribuição sem cumprir LGPL
- ✅ **Estratégia:** Usar como referência visual e comportamental; reimplementar em Rust (Slint/Iced/egui)

### 4.2 `nemomobile/glacier-home` — Home Screen Reference

**Descrição:** Implementação moderna de home screen para Glacier UX (baseado em Nemo Mobile).

**Features relevantes:**
- Grid de aplicações com drag-drop para pastas
- Widgets redimensionáveis
- Quick settings panel (swipe from top)
- App drawer com busca
- Gestos de borda (swipe up → home, swipe down → notifications)
- Tema dinâmico (dark/light/high contrast)

**Mapeamento para Conexus Harmattan:**

| Feature Glacier | Equivalente Conexus | Notes |
|-----------------|---------------------|-------|
| App Grid | Home grid (tiles S/M/W/L) | ✅ Core |
| Folders (drag-drop) | Pastas de tiles | ✅ Core |
| Widgets | Live Tiles redimensionáveis | ✅ Core |
| Quick Settings | Action Center (CommandBar style) | ✅ Core |
| Edge Gestures | Navegação gestual | ✅ Core |
| Search | Busca unificada | ✅ Core |
| Dynamic Theme | Accent dinâmico + dark/light | ✅ 2026 |

### 4.3 `nemomobile/lipstick` — Compositor/Shell

**Descrição:** Compositor Wayland + shell para Nemo Mobile. Gerencia superfícies, input, compositor.

**Relevância:** Arquitetura de shell separada do launcher — útil para entender separação de concerns.

---

## 5. Matriz de Compatibilidade e Reutilização

| Projeto | Licença | Pode Copiar Código? | Pode Referenciar API? | Pode Inspirar Design? | Recomendação |
|---------|---------|---------------------|----------------------|----------------------|--------------|
| `harmattan/*` | GPL-2.0 / LGPL-2.1 / BSD-3 | ❌ (GPL) / ⚠️ (LGPL) | ✅ | ✅ | Referência histórica apenas |
| `ajalkane/*` | BSD-3-Clause | ✅ | ✅ | ✅ | Código livre, mas legacy Qt/QML |
| `rschiang/moedict-meego` | GPL-3.0 | ❌ | ✅ | ✅ | Referência app structure |
| `nemomobile/qt-components` | **LGPL-2.1** | ⚠️ (dynamic link only) | ✅ | ✅ | **Referência API/Design — NÃO copiar** |
| `nemomobile/lipstick` | LGPL-2.1 | ⚠️ | ✅ | ✅ | Referência arquitetural |
| `nemomobile/glacier-home` | LGPL-2.1 | ⚠️ | ✅ | ✅ | **Referência UX principal** |

---

## 6. Estratégia de Migração por Componente

### Componentes de UI (Alta Prioridade)

| Componente Harmattan | Fonte Referência | Implementação Conexus |
|----------------------|------------------|----------------------|
| **Home Screen / App Grid** | `glacier-home` + `qt-components/GridView` | Rust (Slint/Iced/egui) — grid responsivo, drag-drop, pastas |
| **App Drawer / All Apps** | `qt-components/PageStack` + `ListView` | Rust — pivô "Todos · Favoritos · Recentes", busca integrada |
| **Action Center / Quick Settings** | `glacier-home` quick panel | Rust — CommandBar style, 4-col, notificações agrupadas |
| **Lock Screen** | `glacier-home` + `qt-components` | Rust — relógio tipográfico, notificações privacidade |
| **Switches / Toggles** | `qt-components/Switch` + `ajalkane/toggleavailability` | Rust — estado, animação, acessibilidade |
| **Navigation / Pivots** | `qt-components/TabBar` + `PageStack` | Rust — navegação horizontal, gesto swipe |
| **Text Input / Keyboard** | `ajalkane/meego-keyboard` | Rust — shape writing, cursor control, multi-lang |

### Serviços de Sistema (Média Prioridade)

| Serviço Harmattan | Fonte Referência | Implementação Conexus |
|-------------------|------------------|----------------------|
| **Profile Management** | `harmattan/profilematic` + `ajalkane/profilematic` | `conexus-profiles` crate — regras contextuais (localização, hora, wifi) |
| **Tile Engine / Live Tiles** | `glacier-home` widgets | `conexus-core::tiles` — flip 3D, badge, agendamento push/periodic |
| **Notification Broker** | `glacier-home` + `harmattan/syncui` | `conexus-core::notifications` — agrupamento, actions, canais |
| **App Manager / Lifecycle** | `glacier-home` app launching | `conexus-core::apps` — catálogo, launch, pivô recentes |
| **Settings / Config** | `harmattan/accounts-ui` | `conexus-config` + `ConfigStore` trait |

---

## 7. Riscos Legais e Técnicos

| Risco | Descrição | Mitigação |
|-------|-----------|-----------|
| **LGPL-2.1 viral** | Copiar código QML/C++ de nemomobile obriga LGPL no binário final | **Não copiar código** — apenas referenciar API/design; reimplementar em Rust |
| **GPL-3.0 (moedict)** | Contamina todo o projeto se linkado | Não usar — apenas referência conceitual |
| **Qt/QML dependency** | Harmattan original usa Qt — não é alvo | Alvo é Rust-native; Qt apenas referência |
| **Nomes protegidos** | "MeeGo", "Harmattan", "N9" podem ter marcas | Usar "Conexus", "Live Tiles" genérico, "Metro/W10M design system" |
| **Patentes Nokia/Microsoft** | Gestos, tiles, navegação podem ter patentes | Clean room: documentar comportamento observado, não código |

---

## 8. Próximos Passos (Fase 0 → 1)

### Para H1 (Architect)
1. Documentar `conexus-platform` traits baseado em capacidades reais do `glacier-home` e `qt-components`
2. Definir ADR-008 critérios de PoC UI Framework
3. Especificar `design-tokens.json` completo (21 accents, spacing, typography, motion)

### Para H4 (Platform Adapter)
1. Testar `AppDiscovery` no Linux: `desktop-entry` crate, `xdg` dirs, `gio` app-info
2. Testar `NotificationBackend` Linux: `notify-rust` + `zbus` (DBus)
3. Prototypar Android JNI: `PackageManager` → app list, `NotificationManager` → show/dismiss

### Para H5 (UX/UI)
1. Carregar skill `conexus-ux-2026`
2. Criar `design-tokens.json` baseado em `metro-w10m.tokens.json` + Material You dynamic color
3. Definir component specs para: HomeGrid, AppDrawer, ActionCenter, Navigation, Search, Settings
4. Preparar protótipo Figma/Storybook para PoC Fase 1

---

## 9. Resumo de Decisões

| Decisão | Justificativa |
|---------|---------------|
| **Não usar código Qt/QML diretamente** | LGPL-2.1 + stack diferente (Rust-native) |
| **Usar nemomobile como referência principal** | Ativo, LGPL-2.1 (compatível com referência), UX moderna Glacier |
| **harmattan/ org apenas para comportamento histórico** | Archived, kernel antigo, mas documenta UX original N9 |
| **ajalkane/ para componentes específicos** | BSD-3, código livre, mas legacy — apenas referência |
| **Reimplementar tudo em Rust** | Arquitetura Harmattan exige Rust core + multiplatform |

---

**Assinatura:** H0 (Orchestrator) + H1 (Architect)  
**Data:** 2026-10-22  
**Próxima atualização:** Após Fase 1 PoC — adicionar decisões de framework UI