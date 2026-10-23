# Guia Windows — Unlock Bootloader Lumia 730/735 via WPInternals

**Dispositivo:** Nokia Lumia 730 (RM-914) / Lumia 735 (RM-915) — codename `nokia-superman`  
**Ferramenta:** WPInternals (Windows only)  
**Objetivo:** Desbloquear bootloader para permitir flash de Linux (postmarketOS) via fastboot

---

## ⚠️ AVISOS CRÍTICOS

```
┌────────────────────────────────────────────────────────────────────┐
│  1. ESTE PROCESSO APAGA TODOS OS DADOS DO TELEFONE                 │
│     → Backup COMPLETO antes de começar (fotos, contatos, 2FA)      │
│                                                                    │
│  2. RISCO DE BRICK PERMANENTE                                      │
│     → Se falhar no meio, pode precisar de test points (solda)      │
│     → Não há JTAP fácil no Lumia 730/735                           │
│                                                                    │
│  3. SÓ FUNCIONA NO WINDOWS NATIVO                                  │
│     → WPInternals NÃO roda no Wine/WSL2/VM (precisa drivers USB)   │
│                                                                    │
│  4. PRECISA DE CABO USB ORIGINAL MICROSOFT                         │
│     → Cabo genérico falha no modo recovery (engrenagens)           │
│                                                                    │
│  5. BATERIA ≥ 80% OBRIGATÓRIO                                      │
│     → Processo demora 15-30 min; se descarregar = brick            │
└────────────────────────────────────────────────────────────────────┘
```

---

## 📥 Downloads Necessários (Faça AGORA no Windows)

| Arquivo | Link Direto | Onde Salvar |
|---------|-------------|-------------|
| **WPInternals (latest)** | https://github.com/WPInternals/WPInternals/releases/latest → `WPInternals.zip` | `tools/lumia-linux-port/downloads/WPInternals/` |
| **WDRT (recuperação Windows)** | https://go.microsoft.com/fwlink/?LinkId=708425 | Qualquer pasta |
| **FFU Stock (firmware original)** | https://lumiadb.com/ → Search "RM-914" ou "RM-915" → Latest `.ffu` | `tools/lumia-linux-port/downloads/FFU/` |

### Como achar o FFU correto no lumiadb.com:
1. Acesse https://lumiadb.com/
2. Search: `RM-914` (se Lumia 730) ou `RM-915` (se Lumia 735)
3. Clique no modelo exato (ex: `RM-914_103416_2020...`)
4. Baixe o **Latest .ffu** (maior arquivo, ~2-3 GB)
5. **Não extraia** — WPInternals usa o .ffu direto

---

## 🔍 Identifique Seu Modelo Exato

No Lumia ligado:
```
Configurações → Sistema → Sobre → Mais informações
Modelo: RM-914  (Lumia 730 single/dual SIM global)
     OU
Modelo: RM-915  (Lumia 735 US/CA + dual SIM global)
```

**Anote: RM-____** — você vai precisar selecionar no WPInternals.

---

## 🛠 Passo a Passo Completo

### **PASSO 1: Prepare o Telefone**

```
1. Configurações → Atualização e segurança → Backup → "Fazer backup agora"
   OU conecte no PC → Explorador → Copie DCIM/, Documents/, etc.

2. Configurações → Atualização e segurança → Para desenvolvedores
   → "Modo de desenvolvedor" = ATIVADO
   → "Desbloqueio de OEM" = ATIVADO (se aparecer — no WP8.1/WM10 pode não ter)

3. Carregue bateria até ≥ 80% (deixe conectado no carregador durante todo processo)
```

---

### **PASSO 2: Entre no Modo Recovery (Engrenagens ⚙️)**

```
1. Power OFF completo:
   → Segure Power 10-15s até desligar totalmente
   → Se não desliga: Power + Vol Down 15s (force restart)

2. Modo Recovery:
   → Power + Volume Down (SIMULTÂNEO)
   → Segure 10-15s ATÉ VIBRAR
   → SOLTA Power, MANTÉM Volume Down
   → AGUARDA ENGRENAGENS ⚙️ GIRANDO (pode demorar 30-60s)
   → SOLTA Volume Down quando engrenagens aparecerem

3. Conecte USB no PC (cabo original Microsoft)
   → PC deve reconhecer "Nokia Lumia 730" ou "Nokia Lumia 735"
```

**⚠️ Se engrenagens NÃO aparecem:**
- Carregue 30 min → tente de novo
- Cabo/porta USB diferentes
- Power + Vol Down timing diferente (Vol Down primeiro, depois Power)
- Tente em outro PC

---

### **PASSO 3: WPInternals — Unlock Bootloader**

```cmd
# 1. Extraia WPInternals.zip
#    → Pasta WPInternals/ → WPInternals.exe

# 2. Execute COMO ADMINISTRADOR
#    (Botão direito → "Executar como administrador")

# 3. Na interface principal:
#    → Clique em "Unlock Bootloader" (ícone de cadeado)

# 4. Wizard "Unlock Bootloader":
#    a) "Select your device model:"
#       → Nokia Lumia 730 (RM-914)  OU  Nokia Lumia 735 (RM-915)
#       → Next
#
#    b) "Select the FFU file to use as donor:"
#       → Browse → Selecione o .ffu que baixou (em downloads/FFU/)
#       → Next
#
#    c) "Backup your device" (OBRIGATÓRIO):
#       → Marca "Create a backup of your device"
#       → Choose location → pasta com espaço (5+ GB)
#       → Next → Aguarda backup completar
#
#    d) "Patch the bootloader":
#       → Next → Aguarda patch (pode demorar alguns minutos)
#
#    e) "Flash the patched bootloader":
#       → Next → Aguarda flash (telefone reinicia várias vezes, engrenagens)
#       → NÃO DESCONECTE!
#
#    f) "Unlock the bootloader":
#       → Next → Telefone entra em modo especial
#       → Aguarda "Bootloader unlocked successfully"
#
#    g) "Restore your backup" (OPCIONAL — restaura Windows):
#       → Se quiser manter Windows: Next → Restaura backup
#       → Se vai por Linux: Skip / Cancel
```

---

### **PASSO 4: Verifique Unlock**

Após o telefone reiniciar (pode ir para Windows ou ficar em tela preta):

```cmd
# 1. Se bootou no Windows:
#    Configurações → Desenvolvedor → Depuração USB = ON
#    adb reboot bootloader

# 2. Se tela preta / não bootou:
#    Power + Vol Down → fastboot mode

# 3. No PC (terminal):
fastboot devices
# → Deve mostrar serial (ex: 12345678	fastboot)

fastboot getvar unlocked
# → unlocked: yes   ← SUCESSO!

fastboot getvar product
# → product: eagle   ← Codename interno (Lumia 730/735 = eagle)
```

---

### **PASSO 5: (Opcional) Recuperar Windows Se Brickar**

Se der errado e telefone não liga nem engrenagens:

```cmd
# 1. Instale WDRT (Windows Device Recovery Tool)
#    https://go.microsoft.com/fwlink/?LinkId=708425

# 2. Modo Recovery (engrenagens) — mesmo procedimento do PASSO 2

# 3. WDRT detecta "Nokia Lumia 730/735 (Recovery Mode)"
#    → Install → Baixa firmware ~2-3 GB → Flash
#    → NÃO DESCONECTE durante 15-30 min

# 4. Telefone volta para Windows 10 Mobile 15254.603 stock
```

---

## ✅ Checklist Pós-Unlock

- [ ] `fastboot devices` → mostra serial
- [ ] `fastboot getvar unlocked` → `unlocked: yes`
- [ ] `fastboot getvar product` → `product: eagle`
- [ ] Telefone entra em fastboot mode: Power + Vol Down (ligado)

---

## 🆘 Problemas Comuns

| Problema | Solução |
|----------|---------|
| **"Device not found" no WPInternals** | Cabo original Microsoft? Porta USB 2.0? Drivers Nokia/Qualcomm instalados? Modo recovery (engrenagens) ativo? |
| **Engrenagens não aparecem** | Carregue 30 min. Power+VolDown timing. Cabo diferente. PC diferente. |
| **WPInternals trava em "Getting device info"** | Espere 5 min. Reinicia WPInternals. Porta USB 2.0 direta na motherboard. |
| **Erro ao flashar FFU** | FFU errado (RM-914 vs RM-915). Verifique modelo exato. Baixe FFU mais recente. |
| **"Bootloader unlock failed"** | Backup feito? FFU correto? Tente "Unlock" sem restaurar backup. |
| **Telefone não liga após unlock** | Force restart: Power + Vol Down 15s. Se não: WDRT recovery. |

---

## 🔗 Links de Referência

| Recurso | Link |
|---------|------|
| WPInternals GitHub | https://github.com/WPInternals/WPInternals |
| WPInternals Releases | https://github.com/WPInternals/WPInternals/releases |
| WDRT Download | https://go.microsoft.com/fwlink/?LinkId=708425 |
| Lumia Firmware DB | https://lumiadb.com/ |
| XDA Lumia 730 Forum | https://xdaforums.com/f/nokia-lumia-730.4529/ |
| postmarketOS Device | https://wiki.postmarketos.org/wiki/Nokia_Lumia_730_(nokia-superman) |
| Unlock Guide (WOA) | https://woa-project.github.io/LumiaWOA/guides/unlock/ |

---

## 📞 Próximo Passo Após Unlock

Volte para o Linux e rode:

```bash
cd tools/lumia-linux-port
./lumia-port.sh init
./lumia-port.sh build
./lumia-port.sh flash
./lumia-port.sh status
```

**Boa sorte! Lembre-se: resultado = Linux headless para dev kernel, não telefone usável.**