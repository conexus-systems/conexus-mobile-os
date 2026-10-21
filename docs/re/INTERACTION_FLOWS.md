# Interaction Flows — Windows 10 Mobile → Conexus Mobile OS

**Versão:** v0.1  
**Data:** 19 de outubro de 2026  
**Status:** Template — documentar fluxos M0/M1

---

## Formato de Fluxo

Cada fluxo segue notação simplificada:

```
FLUXO-ID: Nome do Fluxo
Pré-condição: Estado inicial necessário
Trigger: Ação do usuário ou evento
Passos:
  1. [Ação] → [Resposta do sistema] → [Novo estado]
  2. ...
Pós-condição: Estado final
Variantes: [Caminhos alternativos]
Erros: [Tratamento de falhas]
Acessibilidade: [Considerações]
Critério de Teste: [Como validar equivalência]
```

---

## Fluxos da Tela Iniciar (START)

### START-FLOW-01: Abrir App via Tile
**Pré-condição:** Start screen visível, tile do app presente
**Trigger:** Tap no tile
**Passos:**
1. Tap no tile → Tile pressed state (scale 0.95, 50ms)
2. Release → Tile scale 1.0 (100ms) + app launch animation (slide up 300ms)
3. App carrega → Splash screen (se cold start) → UI principal
**Pós-condição:** App em foreground, Start em background
**Variantes:**
- App já em memória (warm start): sem splash, resume < 200ms
- Tile wide/large: mesma animação, conteúdo do tile pode influenciar splash
**Erros:**
- App crash na inicialização: volta para Start + toast "App falhou ao abrir"
- App não instalado: tile mostra "Reinstall" badge
**Acessibilidade:** TalkBack anuncia "Opening [App Name]"; focus move para app
**Critério de Teste:** Launch latency cold < 2s, warm < 500ms; animação 60fps

---

### START-FLOW-02: Criar Grupo de Tiles
**Pré-condição:** Start screen, pelo menos 2 tiles não agrupados
**Trigger:** Long press tile A → drag sobre tile B → release
**Passos:**
1. Long press tile A (500ms) → Tile A entra em "drag mode" (elevation +2, scale 1.05)
2. Drag tile A → Visual feedback: tile B destaca (border accent), drop zone aparece
3. Drop sobre tile B → Grupo criado com label padrão "Group"
4. Keyboard abre para editar label (opcional)
**Pós-condição:** Grupo persistente com 2+ tiles, label editável
**Variantes:**
- Drag para área vazia: cria novo grupo com tile único
- Drag tile de grupo existente para outro: move entre grupos
- Cancel (drag para borda ou back): tile retorna com spring animation
**Erros:**
- Drag falha (timeout 10s): cancela, tile retorna
- Grupo > 20 tiles: avisa "Group full", não permite drop
**Acessibilidade:** Drag alternativo via "Move to group" menu no long press; anuncia grupo criado
**Critério de Teste:** Drag latency < 16ms; grupo persiste após reboot; label edit salva

---

### START-FLOW-03: Redimensionar Tile
**Pré-condição:** Start screen, tile selecionável
**Trigger:** Long press tile → drag handle de resize → release
**Passos:**
1. Long press (500ms) → Handles aparecem nos 4 cantos (fade 200ms)
2. Drag handle → Preview live do novo tamanho (snap to grid 1x1, 2x2, 4x2, 4x4)
3. Release → Tile anima para novo tamanho (200ms), layout reflow
**Pós-condição:** Tile no novo tamanho, posição ajustada, persistido
**Variantes:**
- Resize para menor: conteúdo adapta (corta flip, mostra só ícone)
- Resize para maior: conteúdo expande (ativa flip se medium+)
- Keyboard: Setas + Enter para resize acessível
**Erros:**
- Espaço insuficiente: auto-arrasta tiles adjacentes ou cancela
- Tamanho não suportado pelo app: fallback para tamanho padrão
**Acessibilidade:** Anuncia novo tamanho; handles navegáveis por teclado
**Critério de Teste:** Resize suave 60fps; persiste; conteúdo adapta corretamente

---

### START-FLOW-04: Buscar e Abrir App (All Apps + Search)
**Pré-condição:** Start screen
**Trigger:** Swipe right OU tap search box OU tecla Search
**Passos:**
1. Trigger → All Apps list slide-in (300ms) OU Search overlay fade-in (150ms)
2. Digita query (debounce 150ms) → Resultados filtrados em tempo real
3. Tap resultado → Search/List fecha (150ms) → App launch (como START-FLOW-01)
**Pós-condição:** App aberto
**Variantes:**
- Busca por settings: abre Settings page diretamente
- Busca web: abre browser com query
- Histórico: mostra recent apps no topo
**Erros:**
- Sem resultados: "Nothing found" + sugestões
- Index corrompido: reconstrói em background
**Acessibilidade:** Search box auto-focus; resultados anunciados; navegação por setas
**Critério de Teste:** Primeiros resultados < 200ms; filtro tempo real < 50ms/keystroke

---

## Fluxos de Navegação (NAV)

### NAV-FLOW-01: Navegação Voltar (Back)
**Pré-condição:** App com navigation stack > 1
**Trigger:** Botão back (hardware/software) OU gesture swipe from left
**Passos:**
1. Trigger → Página atual slide right (200ms) + página anterior slide from left (200ms)
2. Stack pop → Página anterior resume
**Pós-condição:** Página anterior ativa
**Variantes:**
- Root page: Back minimiza app (vai para Start)
- Modal/Dialog: Back fecha modal primeiro
- WebView: Back navega history do WebView primeiro
**Erros:**
- Stack corrompido: reset para root
- App não responde: ANR dialog após 5s
**Acessibilidade:** Anuncia "Back to [Page Title]"; gesture alternativo via botão
**Critério de Teste:** Transição 60fps; stack consistency; root behavior correto

---

### NAV-FLOW-02: Task Switcher (Recent Apps)
**Pré-condição:** Qualquer tela
**Trigger:** Long press back (500ms) OU gesture swipe up and hold
**Passos:**
1. Trigger → Task switcher fan-out (300ms)
2. Swipe horizontal → Cards navegam, snap to center
3. Tap card → Task switcher collapse (200ms) + App resume
4. Swipe up card → Dismiss (200ms) + remove de recentes
**Pós-condição:** App selecionado em foreground OU card removido
**Variantes:**
- "Clear all" button: remove todos cards
- Split screen (se suportado): Drag card para topo/base
**Erros:**
- App crashado: card mostra "App closed", tap = relaunch
- Memória baixa: SO pode matar apps em background silenciosamente
**Acessibilidade:** Cards anunciados; navegação por teclado; dismiss acessível
**Critério de Teste:** Fan-out 60fps; snapshot preview renderizado; resume < 500ms

---

## Fluxos da Action Center (ACT)

### ACT-FLOW-01: Ver e Dispensar Notificação
**Pré-condição:** Notificação(ões) recebida(s), tela qualquer
**Trigger:** Swipe down from top edge OU tap notification toast
**Passos:**
1. Trigger → Action Center slide down (250ms)
2. Scroll para notificação → Tap para expandir (200ms)
3. Ler conteúdo → Swipe right para dismiss (200ms) OU tap ação
**Pós-condição:** Notificação removida OU ação executada
**Variantes:**
- Agrupadas: Tap grupo expande lista; dismiss grupo = dismiss all
- Prioridade alta: Aparece como toast (peek) 5s antes de ir para Action Center
- Silent: Não faz som/vibração, só badge no Action Center handle
**Erros:**
- Notificação órfã (app uninstalled): "App não instalado" + dismiss
- Ação falha: Toast erro + mantém notificação
**Acessibilidade:** Anuncia contagem ao abrir; cada notificação lida; ações navegáveis
**Critério de Teste:** Abre < 200ms; scroll 60fps; dismiss instantâneo

---

### ACT-FLOW-02: Toggle Quick Action (Wi-Fi)
**Pré-condição:** Action Center aberto
**Trigger:** Tap no tile Wi-Fi
**Passos:**
1. Tap → Tile ripple + estado toggle (on↔off) instantâneo
2. Sistema processa → Wi-Fi radio on/off
3. Tile atualiza ícone + label (connected/disconnected)
**Pós-condição:** Wi-Fi estado alterado, tile reflete estado real
**Variantes:**
- Long press → Abre Settings > Network > Wi-Fi
- Toggle indisponível (modo avião): Tile disabled, tap = toast
**Erros:**
- Radio falha: Tile volta estado anterior + toast "Couldn't turn on"
- Timeout: Loading spinner 10s → erro
**Acessibilidade:** Anuncia novo estado; high contrast icon change
**Critério de Teste:** Toggle visual < 50ms; radio state sync < 2s; long press navega correto

---

## Fluxos de Configurações (SET)

### SET-FLOW-01: Alterar Tema (Light/Dark)
**Pré-condição:** Settings > Personalization > Colors
**Trigger:** Tap em "Choose your mode" → Select Dark
**Passos:**
1. Tap radio "Dark" → Radio selected state imediato
2. Sistema aplica tema → UI reconstrói com dark tokens (fade 200ms)
3. Persiste preferência
**Pós-condição:** Todo SO em dark mode
**Variantes:**
- Auto (sunset/sunrise): Usa location/time
- Per-app override: Apps podem opt-out
**Erros:**
- Tema não aplica: Fallback para light + log erro
**Acessibilidade:** High contrast override vence; anúncio "Dark mode enabled"
**Critério de Teste:** Aplicação < 500ms; sem flicker; persiste reboot

---

## Fluxos de Teclado (KEY)

### KEY-FLOW-01: Digitação com Shape Writing
**Pré-condição:** Text field focado, teclado visível
**Trigger:** Swipe contínuo sobre teclas formando palavra
**Passos:**
1. Finger down na primeira letra → Trail inicia
2. Swipe através das letras → Trail segue dedo, predição atualiza tempo real
3. Finger up → Palavra confirmada, inserida no campo + espaço
4. Teclado reset para próxima palavra
**Pós-condição:** Palavra no text field
**Variantes:**
- Palavra errada: Tap sugestão alternativa OU backspace apaga palavra inteira
- Gesture curto (< 3 letras): Fallback para tap individual
**Erros:**
- Trail muito rápido: "Slow down" toast
- Nenhuma predição: Mostra "No match" + teclas individuais
**Acessibilidade:** Anuncia palavra predita; undo acessível; shape writing desativável
**Critério de Teste:** Precisão > 90% top 1000 palavras; trail 60fps; latência < 50ms

---

## Fluxos de Bloqueio (LOCK)

### LOCK-FLOW-01: Unlock com PIN
**Pré-condição:** Tela bloqueada, PIN configurado
**Trigger:** Wake (power button, double tap, lift) → Swipe up ou tap
**Passos:**
1. Wake → Lock screen fade in (300ms), relógio + notificações
2. Swipe up → PIN entry slide up (250ms)
3. Digita PIN → Dot indicators preenchem, ripple per key
4. Último dígito → Validação (async < 500ms)
5. Sucesso → Lock screen slide down (300ms) → Home/Last app
**Pós-condição:** Device unlocked
**Variantes:**
- Biometric: Face/fingerprint → Skip PIN se sucesso
- Emergency call: Bottom-left button sempre acessível
- Forgot PIN: "Forgot PIN" → Microsoft account recovery (se conectado)
**Erros:**
- PIN errado: Shake + erro + delay progressivo (1s, 2s, 4s, 30s lockout)
- Biometric falha 3x: Força PIN
**Acessibilidade:** PIN anunciado se "Speak passwords" on; emergency call sempre visível
**Critério de Teste:** Unlock success < 1s (PIN) / < 2s (biometric); lockout funciona

---

## Rastreabilidade

| Fluxo ID | Behavior Matrix IDs | Screen Catalog | Test Case |
|----------|---------------------|----------------|-----------|
| START-FLOW-01 | START-001, TILE-001..004 | START-01, TILE-* | tests/integration/start_launch.test |
| START-FLOW-02 | START-002, TILE-006 | START-01 | tests/integration/start_groups.test |
| START-FLOW-03 | TILE-006 | TILE-* | tests/integration/tile_resize.test |
| START-FLOW-04 | START-003, START-004 | START-05, START-06 | tests/integration/start_search.test |
| NAV-FLOW-01 | NAV-001 | NAV-02 | tests/device/nav_back.test |
| NAV-FLOW-02 | NAV-003 | NAV-01 | tests/device/nav_switcher.test |
| ACT-FLOW-01 | ACT-001, ACT-003 | ACT-01, ACT-03 | tests/integration/act_notifications.test |
| ACT-FLOW-02 | ACT-002 | ACT-02 | tests/device/act_toggles.test |
| SET-FLOW-01 | SET-001, SET-002 | SET-01, SET-02 | tests/integration/settings_theme.test |
| KEY-FLOW-01 | KEY-001, KEY-002 | KEY-01, KEY-02 | tests/device/keyboard_shape.test |
| LOCK-FLOW-01 | LOCK-001, LOCK-003 | LOCK-01, LOCK-02 | tests/device/lock_unlock.test |

---

## Próximos Passos

1. [ ] Validar cada fluxo no device de referência (Lumia 950 XL)
2. [ ] Capturar vídeos de referência (60fps) para comparação visual
3. [ ] Definir métricas quantitativas por passo (latência, fps, memory)
4. [ ] Mapear para test cases automatizados (Espresso/UIAutomator/Playwright)