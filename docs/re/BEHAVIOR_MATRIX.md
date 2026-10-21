# Behavior Matrix — Windows 10 Mobile → Conexus Mobile OS

**Versão:** v0.1  
**Data:** 19 de outubro de 2026  
**Status:** Template — preencher M0/M1

---

## Como Usar

Cada linha = um recurso comportamental observado no Windows 10 Mobile.
Colunas definem critérios objetivos para considerar implementação **equivalente**.

| Coluna | Descrição |
|--------|-----------|
| **ID** | Identificador único (ex: `START-001`, `TILE-003`) |
| **Categoria** | Start / Tile / Nav / ActionCenter / Settings / Keyboard / Lock / Other |
| **Recurso** | Nome descritivo curto |
| **Comportamento Observado** | O que acontece (estado inicial → interação → resposta visual → estado final) |
| **Transições/Animações** | Duração, easing, stagger, direção |
| **Acessibilidade** | Screen reader, high contrast, text scaling, touch target size |
| **Estados de Erro** | O que acontece em falha (rede, permissão, recurso indisponível) |
| **Critério de Equivalência** | **Teste objetivo** que passa se implementação for equivalente |
| **Prioridade** | P0 (MVP) / P1 (Alpha) / P2 (Beta) / P3 (Future) |
| **Status** | Not Started / In Progress / Implemented / Tested / Approved |
| **Notas** | Edge cases, variações, decisões |

---

## Matriz (Template — duplicar para cada recurso)

### START — Tela Iniciar

| ID | Categoria | Recurso | Comportamento Observado | Transições/Animações | Acessibilidade | Estados de Erro | Critério de Equivalência | Pri | Status | Notas |
|----|-----------|---------|------------------------|---------------------|----------------|-----------------|-------------------------|-----|--------|-------|
| START-001 | Start | Grid de Tiles | Tela mostra grid 4-col (portrait) / 6-col (landscape), tiles redimensionáveis | Entrada: fade-in 200ms + slide-up 300ms stagger | TalkBack lê tile name + size + content; high contrast mode | Se não há tiles: mostra "Add tiles" placeholder | Grid renderiza em < 100ms; touch scroll 60fps; reflow em rotação < 200ms | P0 | | |
| START-002 | Start | Grupo de Tiles | Tiles podem ser agrupados com label; drag para criar grupo | Drag: tile flutua 400ms; grupo forma com spring | Group name anunciado; drag acessível via teclado | Se drag falha: tile retorna com shake | Drag-drop cria grupo persistente; label editável | P0 | | |
| START-003 | Start | Lista de Apps (All Apps) | Swipe right ou tap "All apps" → lista alfabética com jump list | Slide horizontal 300ms; jump list fade 150ms | Lista navegável por teclado; primeiro letter annunciation | Lista vazia: "No apps installed" | Lista carrega < 500ms; scroll 60fps; search filtra em tempo real | P0 | | |
| START-004 | Start | Busca Integrada | Tap search box ou tecla Search → overlay com resultados apps/settings/web | Fade-in 150ms; resultados aparecem incrementalmente | Search box focado; resultados anunciados | Sem resultados: "Nothing found" + sugestões | Primeiros resultados < 200ms; digitação debounced 150ms | P1 | | |

### TILE — Live Tiles

| ID | Categoria | Recurso | Comportamento Observado | Transições/Animações | Acessibilidade | Estados de Erro | Critério de Equivalência | Pri | Status | Notas |
|----|-----------|---------|------------------------|---------------------|----------------|-----------------|-------------------------|-----|--------|-------|
| TILE-001 | Tile | Tile Pequeno (1x1) | Mostra ícone + contador/badge; tap abre app | Press: scale 0.95 50ms; release: scale 1.0 100ms | Anuncia: "App name, 5 notifications" | Sem badge: não mostra | Render < 50ms; badge update < 100ms | P0 | | |
| TILE-002 | Tile | Tile Médio (2x2) | Conteúdo rico: texto, imagem, flip animation | Flip: 3D rotate 600ms; cycle auto 5s | Conteúdo lido sequencialmente | Falha load: mostra último cache | Flip suave 60fps; memory < 10MB/tile | P0 | | |
| TILE-003 | Tile | Tile Largo (4x2) | Conteúdo estendido: múltiplas linhas, ações rápidas | Igual médio + layout horizontal | Igual | Igual | Igual + ações rápidas funcionais | P1 | | |
| TILE-004 | Tile | Tile Grande (4x4) | Full content: map, photo album, music player | Parallax scroll dentro do tile | Navegação interna acessível | Placeholder se app suspenso | Scroll interno 60fps; não bloqueia Start | P1 | | |
| TILE-005 | Tile | Live Update (Background) | Background task atualiza tile via push/periodic | Sem animação forçada; conteúdo troca no próximo ciclo | Anuncia mudança se screen reader ativo | Falha rede: mantém último; retry exponential | Update processado < 2s; não bloqueia UI thread | P1 | | |
| TILE-006 | Tile | Redimensionamento | Long press → handles → drag para resize → snap to grid | Handles appear 200ms; resize preview live; snap 150ms | Handles acessíveis; anúncio novo tamanho | Tamanho inválido: volta ao anterior | Resize persiste; layout reflow < 200ms | P0 | | |

### NAV — Navegação

| ID | Categoria | Recurso | Comportamento Observado | Transições/Animações | Acessibilidade | Estados de Erro | Critério de Equivalência | Pri | Status | Notas |
|----|-----------|---------|------------------------|---------------------|----------------|-----------------|-------------------------|-----|--------|-------|
| NAV-001 | Nav | Botão Voltar (Hardware/Software) | Volta na pilha de navegação; long press = task switcher | Slide right 200ms (page) / 300ms (task switcher) | Anuncia "Back" ou "Recent apps" | Pilha vazia: minimiza app / vai para Start | Back consistent em todo SO; task switcher < 300ms | P0 | | |
| NAV-002 | Nav | Gestos de Borda (Edge Swipes) | Swipe from left = back; right = action center; top = notifications | Follow finger; commit at 50% threshold | Gestos alternativos (botões) | Conflito com app: app priority | Latência < 16ms; threshold configurável | P1 | | |
| NAV-003 | Nav | Task Switcher (Card Stack) | Long press back ou gesture → cards horizontais com preview | Cards fan out 300ms; swipe dismiss 200ms | Cards anunciados; dismiss por teclado | Sem apps: "No recent apps" | Preview renderizado; swipe 60fps | P0 | | |
| NAV-004 | Nav | Pivot/Panorama (App Pattern) | Horizontal swipe entre seções; header fixo com tabs | Swipe: follow finger; snap to section 200ms | Tab announcement; swipe accessible | Uma seção: desabilita swipe | Snap preciso; header parallax | P1 | | |

### ACTIONCENTER — Central de Ações

| ID | Categoria | Recurso | Comportamento Observado | Transições/Animações | Acessibilidade | Estados de Erro | Critério de Equivalência | Pri | Status | Notas |
|----|-----------|---------|------------------------|---------------------|----------------|-----------------|-------------------------|-----|--------|-------|
| ACT-001 | ActionCenter | Painel de Notificações | Swipe from top → sheet com notificações agrupadas por app/tempo | Slide down 250ms; overscroll bounce | Notificações anunciadas; ações acessíveis | Sem notif: "No notifications" | Abre < 200ms; scroll 60fps; group expand/collapse | P0 | | |
| ACT-002 | ActionCenter | Quick Actions (Toggles) | Grid 4-col de toggles (WiFi, BT, Rotation, etc.); long press = settings | Tap: toggle instant + ripple; long press: navigate 300ms | Toggle state anunciado; high contrast icons | Toggle falha: toast "Couldn't change" | Toggle state sync < 500ms; navega para settings correto | P0 | | |
| ACT-003 | ActionCenter | Notificação Expandida | Tap notificação → expande com ações (Reply, Dismiss, Open) | Expand 200ms; collapse 150ms | Ações anunciadas; focus management | Ação falha: toast + mantém expandida | Expand suave; ações funcionais | P1 | | |

### SETTINGS — Configurações

| ID | Categoria | Recurso | Comportamento Observado | Transições/Animações | Acessibilidade | Estados de Erro | Critério de Equivalência | Pri | Status | Notas |
|----|-----------|---------|------------------------|---------------------|----------------|-----------------|-------------------------|-----|--------|-------|
| SET-001 | Settings | Categorias Principais | Sistema, Dispositivos, Rede, Personalização, Contas, Hora, Privacidade, Atualização | Navegação: slide horizontal 200ms | Categorias anunciadas; busca integrada | Categoria indisponível: hidden | Busca resultados < 300ms; navegação fluida | P0 | | |
| SET-002 | Settings | Página de Detalhe | Lista de toggles, sliders, entradas; breadcrumb no topo | Slide in 200ms; back slide out 200ms | Todos controles acessíveis | Setting falha: toast + log | Mudança persiste instantaneamente | P0 | | |
| SET-003 | Settings | Busca Global | Top search box filtra categorias + settings individuais | Debounce 150ms; resultados fade 100ms | Resultados anunciados | Sem resultados: "No matches" | Index completo; busca < 200ms | P1 | | |

### KEYBOARD — Teclado Virtual

| ID | Categoria | Recurso | Comportamento Observado | Transições/Animações | Acessibilidade | Estados de Erro | Critério de Equivalência | Pri | Status | Notas |
|----|-----------|---------|------------------------|---------------------|----------------|-----------------|-------------------------|-----|--------|-------|
| KEY-001 | Keyboard | Layout QWERTY (Português) | Teclas com press states; long press = accents/symbols | Key press: scale 0.9 30ms; popup 100ms | High contrast keys; key announcements | Layout não carregado: fallback EN | Latência tecla < 10ms; popup suave | P0 | | |
| KEY-002 | Keyboard | Shape Writing (Swipe) | Swipe contínuo entre letras → predição de palavra | Trail visual segue dedo; palavra aparece 100ms | Anuncia palavra predita; undo acessível | Falha predição: mostra alternatives | Precisão > 90% palavras comuns; trail 60fps | P1 | | |
| KEY-003 | Keyboard | Cursor Control | Tap hold no texto → lupa + cursor draggable | Lupa fade 100ms; cursor move suavemente | Anuncia posição; navegação por setas | Campo read-only: sem lupa | Posicionamento pixel-perfect; sem lag | P0 | | |

### LOCK — Tela de Bloqueio

| ID | Categoria | Recurso | Comportamento Observado | Transições/Animações | Acessibilidade | Estados de Erro | Critério de Equivalência | Pri | Status | Notas |
|----|-----------|---------|------------------------|---------------------|----------------|-----------------|-------------------------|-----|--------|-------|
| LOCK-001 | Lock | Relógio + Data | Centro superior; atualiza a cada minuto; estilo tipográfico | Fade in 300ms ao wake | Anuncia hora ao wake | Time sync falha: mostra "--:--" | Render < 100ms; atualização pontual | P0 | | |
| LOCK-002 | Lock | Notificações na Lock | Permitidas pelo usuário; tap → unlock + open app | Slide up com notificações; dismiss swipe | Anuncia contagem; ações acessíveis | Sem permissão: não mostra | Privacy respected; unlock < 500ms | P1 | | |
| LOCK-003 | Lock | Autenticação | PIN, Password, Windows Hello (face/fingerprint — se HW) | PIN: ripple por dígito; biometric: animation | Feedback háptico/sonoro; acessível | Falha 5x: lockout 30s | Biometric < 2s; PIN < 500ms | P1 | | |

---

## Legenda de Prioridade

- **P0** — Bloqueia MVP (M3 gate). Deve estar implementado e testado.
- **P1** — Bloqueia Alpha (M4 gate). Deve estar implementado.
- **P2** — Beta. Desejável mas não bloqueante.
- **P3** — Futuro. Documentado para roadmap.

---

## Rastreabilidade

Cada ID deve ter:
- [ ] Test case em `tests/device/` ou `tests/integration/`
- [ ] Reference implementation em `shell/` ou `services/`
- [ ] Design spec em `shell/fluent/` ou `framework/sdk/`