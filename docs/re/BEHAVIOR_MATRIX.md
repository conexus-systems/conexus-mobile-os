# Behavior Matrix — MeeGo Harmattan → Conexus Harmattan

**Projeto:** Conexus Harmattan  
**Data:** 22 de outubro de 2026  
**Fase:** 0 — Auditoria  
**Base:** Referências: `harmattan/*`, `nemomobile/glacier-home`, `nemomobile/qt-components`, `ajalkane/*`

---

## Objetivo

Documentar **um comportamento por linha** com **critérios de equivalência mensuráveis** para cada funcionalidade do MeeGo Harmattan que o Conexus Harmattan deve reproduzir, adaptar ou substituir.

**Formato:** Cada linha = 1 comportamento observável + critério de aceitação testeável.

---

## 1. Home Screen / App Grid

| ID | Comportamento Harmattan | Referência | Equivalência Conexus (Critério de Aceitação) | Prioridade |
|----|------------------------|------------|---------------------------------------------|------------|
| HS-001 | Grid de aplicações 4 colunas (portrait) / 6 (landscape) | `glacier-home`, `qt-components/GridView` | Grid responsivo: 360dp=4cols, 600dp=5cols, 900dp=6cols, 1200dp=8cols | P0 |
| HS-002 | Tiles redimensionáveis: Small (1×1), Medium (2×2), Wide (4×2), Large (4×4) | `glacier-home` widgets | 4 tamanhos implementados; drag-handle para resize; snap to grid | P0 |
| HS-003 | Grupos nomeados (drag para criar, drag para mover) | `glacier-home` | Drag app sobre app → cria pasta; drag pasta → move; tap pasta → expande | P0 |
| HS-004 | Pastas: tap expande inline; drag out → remove da pasta | `glacier-home` | Animação 200ms; max 20 apps/pasta; nome editável | P1 |
| HS-005 | Scroll vertical contínuo com momentum | `glacier-home` | Scroll suave 60fps; overscroll glow; snap-to-page opcional | P0 |
| HS-006 | Busca integrada (tap topo ou swipe down centro) | `glacier-home`, `qt-components/SearchBar` | SearchBar persistente ou modal; filtra apps + settings + web; histórico | P0 |
| HS-007 | Jump list A-Z (scroll rápido lateral) | `qt-components/ListView` | Scrollbar interativa A-Z; tap letra → salta para seção | P1 |
| HS-008 | Favoritos (estrela) → pivô "Favoritos" no app drawer | `glacier-home` | Long-press app → toggle favorito; pivô "Favoritos" atualiza em tempo real | P1 |
| HS-008 | Recentes (MRU) → pivô "Recentess" no app drawer | `glacier-home` | Auto-populado por launch; max 20; limpa ao reiniciar | P1 |
| HS-010 | Wallpaper: cor sólida + parallax opcional | `glacier-home` | Cor accent dinâmica (Material You) + imagem parallax opcional | P1 |

---

## 2. Live Tiles / Widgets

| ID | Comportamento Harmattan | Referência | Equivalência Conexus | Prioridade |
|----|------------------------|------------|----------------------|------------|
| LT-001 | 4 tamanhos: S(1×1), M(2×2), W(4×2), L(4×4) | `glacier-home` widgets | 4 tamanhos; flip 3D para M/W/L | P0 |
| LT-002 | Flip animation 3D (Y-axis, 600ms ciclo) | `glacier-home`, N9 | Flip Y-axis 600ms; front/back content; pause on hover/focus | P0 |
| LT-003 | Badge numérico (1-99, "99+") + glyph | N9 tiles | Badge no canto; cor accent; acessível (aria-label) | P1 |
| LT-004 | Update mechanisms: push, periodic, scheduled, local | N9 tile templates | `TileUpdateMode`: Push, Periodic (min 15min), Manual, Scheduled | P0 |
| LT-005 | Content templates: icon, text, image, media, flip | N9 XML templates | 5 templates built-in; custom via manifest | P1 |
| LT-006 | Tile groups (folders de tiles) | `glacier-home` | Drag tile sobre tile → grupo; nomeável; max 9 tiles/grupo | P1 |

---

## 3. Navigation / Pivots / Gestures

| ID | Comportamento Harmattan | Referência | Equivalência Conexus | Prioridade |
|----|------------------------|------------|----------------------|------------|
| NV-001 | Three panes horizontais: Events · Apps · Settings | N9, `glacier-home` | Pivô horizontal 3 tabs: Eventos · Apps · Configurações; swipe lateral | P0 |
| NV-002 | Swipe edge esquerda → Back | N9, Android 10+ | Edge swipe 20px threshold; anim 200ms; preditivo (Android 14+) | P0 |
| NV-003 | Swipe edge direita → Action Center / Multitask | N9, `glacier-home` | Edge swipe direita → Action Center; long-swipe → Overview | P0 |
| NV-004 | Swipe up (home) → Home screen | Android 10+ | Gesture navigation padrão; fallback 3-botões | P0 |
| NV-005 | Long press home → Multitask / Overview | N9, Android | Overview card stack horizontal; swipe dismiss; tap resume | P0 |
| NV-006 | Double tap home → Quick switch (último app) | Android 12+ | Double-tap gesture navegação → app anterior | P1 |
| NV-007 | Pull-down topo → Notificações / Search | `glacier-home`, Android | Pull-down centro → Search; pull-down bordas → Notifications/Quick Settings | P0 |
| NV-008 | Pull-down dentro app → Refresh / Search contextual | `qt-components` | Pull-to-refresh padrão; search contextual se app suporta | P1 |
| NV-009 | Pinch → Zoom (gallery, maps, browser) | N9 apps | Pinch gesture padrão; não no launcher home | P2 |

---

## 4. Action Center / Notifications / Quick Actions

| ID | Comportamento Harmattan | Referência | Equivalência Conexus | Prioridade |
|----|------------------------|------------|----------------------|------------|
| AC-001 | Swipe top-edge → Action Center (Quick Actions 4-col + Notifications) | N9, `glacier-home` | Swipe top → panel; Quick Actions 4-col CommandBar style; Notifications abaixo | P0 |
| AC-002 | Quick Actions: toggles (WiFi, BT, Rotation, Location, etc) | N9, `ajalkane/toggleavailability` | 4-col grid; tap toggle; long-press → Settings da feature | P0 |
| AC-003 | Notifications agrupadas por app, expansíveis | N9, Android | Group by app; expand/collapse; inline actions (Reply, Dismiss, Open) | P0 |
| AC-004 | Priority: High (peek toast), Normal, Low (silent) | Android | High: heads-up; Normal: banner; Low: sem som/banner | P1 |
| AC-005 | Inline Reply (resposta direta na notificação) | Android 7+ | `RemoteInput` action; text input inline; send sem abrir app | P1 |
| AC-005 | Dismiss swipe (left/right) + "Clear all" | Android | Swipe dismiss individual; botão "Limpar tudo" no topo | P0 |
| AC-006 | Notification dots no icon do app (badge count) | Android 8+ | Badge no tile/icon; count ou dot; long-press → preview | P1 |
| AC-007 | Canais de notificação (config por app) | Android 8+ | Settings → Notificações → por app: importância, som, vibração | P1 |

---

## 5. Settings / Configuração

| ID | Comportamento Harmattan | Referência | Equivalência Conexus | Prioridade |
|----|------------------------|------------|----------------------|------------|
| ST-001 | Categorias com pivô lateral (Personalização, Sistema, Apps, Contas) | N9, `harmattan/accounts-ui` | NavigationRail lateral; categorias: Personalização, Sistema, Apps, Rede, Privacidade, Sobre | P0 |
| ST-002 | Busca integrada nas configurações | N9, `glacier-home` | SearchBar no topo; filtra categorias + itens; destaca resultado | P0 |
| ST-003 | Personalização: Tema (Dark/Light/Auto), Accent (21 cores + custom), Animações | N9, `glacier-home`, Metro/W10M tokens | ThemeProvider: dark/light/high-contrast; 21 accents + custom hex; motion scale (off/low/normal/high) | P0 |
| ST-004 | Accent color: 21 system colors + custom picker (AA contrast auto) | N9, Metro/W10M | 21 presets + custom hex; validação contraste 4.5:1; dynamic color (Android 12+) | P0 |
| ST-005 | Reduzir animações (acessibilidade) | WCAG, Android | `prefers-reduced-motion` respeitado; motion scale: off/low/normal/high | P0 |
| ST-006 | Idiomas + teclado (múltiplos, quick switch spacebar long-press) | N9, `ajalkane/meego-keyboard` | IME selector; shape writing; cursor magnifier; quick switch | P1 |
| ST-007 | Perfis contextuais (localização, hora, wifi, bateria) | `harmattan/profilematic` | `conexus-profiles`: regras IF-THEN; perfis: Trabalho, Casa, Carro, Sono, Personalizado | P1 |

---

## 6. Lock Screen

| ID | Comportamento Harmattan | Referência | Equivalência Conexus | Prioridade |
|----|------------------------|------------|----------------------|------------|
| LS-001 | Relógio tipográfico grande (Segoe UI Light ~48pt), data abaixo | N9 | `font-family: Inter Variable`; tamanho responsivo; data localizada | P0 |
| LS-002 | Background: user image / slideshow / dynamic | N9 | Imagem usuário; slideshow; dynamic color background (Material You) | P1 |
| LS-003 | Notificações com privacidade (hide content, show count only) | N9, Android | Config: mostrar/ocultar conteúdo; apenas contador; app name visível | P0 |
| LS-004 | Quick actions: Camera (swipe up bottom-right), Flashlight | N9, Android | Swipe up corners: Camera / Flashlight; haptic feedback | P1 |
| LS-005 | Auth: PIN (4+ dígitos), Password, Biometric (fingerprint/face) | N9, Android | `BiometricPrompt` API; fallback PIN; lockout progressivo | P0 |
| LS-006 | Emergency call button (sempre visível) | Regulatory | Botão "Emergência" sempre acessível; discador direto | P0 |

---

## 7. Input / Keyboard / Text

| ID | Comportamento Harmattan | Referência | Equivalência Conexus | Prioridade |
|----|------------------------|------------|----------------------|------------|
| IN-001 | Layouts QWERTY por idioma + símbolos/números (shift) | N9, `ajalkane/meego-keyboard` | IME multi-idioma; long-press spacebar → switch; símbolos via shift/alt | P1 |
| IN-002 | Shape Writing (swipe contínuo → predição palavra) | N9, `ajalkane/meego-keyboard` | Gesture typing; dicionário local; aprendizagem vocabulário usuário | P2 |
| IN-003 | Cursor control: long press texto → lupa + cursor arrastável | N9 | Long-press → magnifier + draggable handle; mouse/touch | P1 |
| IN-004 | Text prediction acima do teclado, tap para inserir | N9 | Suggestion bar; tap insere; aprende vocabulário; multilíngue | P1 |
| IN-005 | Emoji panel (categorias, busca, recentes) | Android, iOS | Emoji picker; categorias; skin tones; recentes; search | P1 |

---

## 8. Profiles / Context Awareness

| ID | Comportamento Harmattan | Referência | Equivalência Conexus | Prioridade |
|----|------------------------|------------|----------------------|------------|
| PF-001 | Perfis baseados em: localização, hora, WiFi, Bluetooth, bateria | `harmattan/profilematic` | Rules engine: IF (location=work) AND (time=09-18) THEN profile=Work | P1 |
| PF-002 | Transições automáticas + manual override | `harmattan/profilematic` | Auto-switch com notificação; toggle manual no Action Center | P1 |
| PF-003 | Ações por perfil: volume, brilho, WiFi, BT, DND, wallpaper | `harmattan/profilematic` | Ações declarativas no rule; revert ao sair do contexto | P1 |
| PF-004 | Perfis pré-definidos: Trabalho, Casa, Carro, Sono, Personalizado | `glacier-home` | 5 presets + custom; export/import JSON | P1 |

---

## 9. Accessibility (A11y) — WCAG 2.2 AA Baseline

| ID | Critério WCAG | Comportamento Esperado | Teste |
|----|---------------|------------------------|-------|
| A11y-001 | 1.1.1 Non-text Content | Icons têm labels; imagens alt text | axe-core scan |
| A11y-002 | 1.3.1 Info & Relationships | Heading order, landmarks, regions | axe-core + manual |
| A11y-003 | 1.4.3 Contrast (AA) | 4.5:1 texto, 3:1 large text/UI | Stark, axe |
| A11y-004 | 1.4.4 Resize Text | 200% zoom sem loss function | Browser zoom test |
| A11y-005 | 1.4.11 Non-text Contrast | 3:1 UI components, focus indicators | axe |
| A11y-006 | 1.4.13 Content on Hover/Focus | Tooltips, menus acessíveis | manual |
| A11y-007 | 2.1.1 Keyboard | All functionality via keyboard | Tab/Shift+Tab test |
| A11y-008 | 2.1.2 No Keyboard Trap | Focus movable, Esc dismiss | manual |
| A11y-009 | 2.4.3 Focus Order | Logical, visual=tab order | Tab test |
| A11y-010 | 2.4.7 Focus Visible | 2px+ outline, 3:1 contrast | axe, manual |
| A11y-011 | 2.5.1 Pointer Gestures | Single-pointer alternative | manual |
| A11y-012 | 2.5.2 Pointer Cancellation | Up-event cancel, no down-event action | manual |
| A11y-013 | 2.5.3 Label in Name | Accessible name = visual label | axe |
| A11y-014 | 2.5.5 Target Size | 44×44dp minimum (24×24 + 8dp spacing) | manual |
| A11y-015 | 3.2.1 On Focus | No unexpected context change | manual |
| A11y-016 | 3.2.2 On Input | No auto-submit on change | manual |
| A11y-017 | 4.1.2 Name/Role/Value | Custom components ARIA compliant | axe, screen reader |

---

## 10. Performance & Quality Targets

| Métrica | Target | Medição |
|---------|--------|---------|
| **Frame rate** | 60fps sustained | `perfetto` / `flutter` devtools |
| **Frame time** | ≤16ms (p99) | `tracing` + `criterion` |
| **Interaction latency** | ≤100ms (tap→feedback) | `criterion` bench |
| **App launch (cold)** | ≤1.5s | `adb shell am start -W` |
| **Search results** | ≤200ms (local) | `criterion` |
| **Memory (launcher process)** | <150MB RSS | `adb shell dumpsys meminfo` |
| **Battery impact** | ≤1.5%/h idle, ≤8%/h active | `BatteryManager` / `upower` |
| **Binary size (release)** | <50MB (Android APK) | `cargo bloat` |
| **Compile time (incremental)** | <30s | `cargo build --timings` |
| **A11y score (axe-core)** | 0 violations WCAG 2.2 AA | `axe-cli` CI |

---

## 11. Cross-Platform Parity Matrix

| Feature | Linux (Wayland/X11) | Android (API 24+) | Paridade |
|---------|---------------------|-------------------|----------|
| Home Grid | ✅ | ✅ | ✅ Visual + behavior |
| Tiles (S/M/W/L) | ✅ | ✅ | ✅ |
| Flip Animation | ✅ | ✅ | ✅ |
| App Drawer + Search | ✅ | ✅ | ✅ |
| Action Center | ✅ (DBus) | ✅ (System) | ✅ Contract |
| Quick Actions | ✅ | ✅ | ✅ |
| Notifications | ✅ (Freedesktop) | ✅ (NotificationManager) | ✅ Contract |
| Runtime Permissions | ⚠️ (Portals/Polkit) | ✅ (Native) | ⚠️ Gap documentado |
| Dynamic Color | ❌ (Static fallback) | ✅ (Material You) | ADR-010 |
| Gestures (edge swipe) | ⚠️ (libinput) | ✅ (System) | ⚠️ Gap documentado |
| Widgets | Custom | ✅ (AppWidget) | Divergente |
| Launcher Registration | .desktop + DBus | `CATEGORY_HOME` | Diferente |
| System Services | ✅ (systemd/D-Bus) | ❌ | Linux-only |

---

## 12. Critérios de Aceitação por Fase

### Fase 1 (PoC UI)
- [ ] Framework compila+executa Linux+Android
- [ ] Home Grid + 3 tiles + navegação tabs + theme toggle
- [ ] A11y: axe-core 0 violations

### Fase 2 (Workspace)
- [ ] `cargo check/test/clippy/fmt/audit/deny` passam 100%
- [ ] Crates base criadas: error, config, events, diagnostics, platform

### Fase 3 (Core Mínimo)
- [ ] Config persistente + migrações
- [ ] Event bus funcional
- [ ] Diagnostics collector
- [ ] Platform traits definidos + stubs Linux/Android compilam

### Fase 4 (Launcher Funcional)
- [ ] Home Grid + App Drawer + Action Center + Settings
- [ ] Navegação pivôs + gestos + search
- [ ] Tema dark/light + accent picker
- [ ] Funcional nos 2 targets

### Fase 5 (Serviços)
- [ ] Profiles + rules engine
- [ ] App Catalog + launch via platform
- [ ] Permissions policy
- [ ] Matrix plataforma documentada

### Fase 6 (Migração Harmattan)
- [ ] Behavior matrix 100% mapeada (implementado/parcial/não-suportado/planejado)
- [ ] Componentes legados removidos/isolados

### Fase 7 (Distribuição)
- [ ] APK assinado + Linux package (Flatpak/AppImage)
- [ ] Install/onboarding flow
- [ ] SBOM + signatures reproduzíveis

---

**Assinatura:** H1 (Architect), H5 (UX/UI)  
**Data:** 2026-10-22  
**Próxima atualização:** Após cada fase — marcar itens concluídos