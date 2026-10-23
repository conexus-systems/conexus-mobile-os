# Lumia Linux Port Helper — `tools/lumia-linux-port/`

**Versão:** 0.1  
**Data:** 2026-10-22  
**Dispositivo:** Nokia Lumia 730 / 735 (codename `nokia-superman`, SoC MSM8226/Snapdragon 400)  
**Objetivo:** Automatizar build/flash de **postmarketOS** (Linux) para desenvolvedores de kernel/drivers.

---

## ⚠️ AVISO CRÍTICO — LEIA ANTES DE CONTINUAR

```
┌────────────────────────────────────────────────────────────────────┐
│  ESTE NÃO É UM INSTALADOR DE "ANDROID" NEM DE "TELEFONE FUNCIONAL" │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  O que VOCÊ TERÁ após rodar tudo:                                  │
│  ✅ Kernel Linux bootando (mainline ou downstream)                 │
│  ✅ Shell serial/USB (headless, sem interface gráfica)             │
│  ✅ Acesso a /proc, /sys, LEDs, CPU info                           │
│                                                                    │
│  O que NÃO FUNCIONA (e não vai funcionar tão cedo):               │
│  ❌ Display / GPU (Adreno 305 — sem driver DRM/KMS mainline)      │
│  ❌ Touchscreen (sem driver I2C mainline)                          │
│  ❌ WiFi / Bluetooth (sem firmware proprietário QCA)               │
│  ❌ Modem / Chamadas / SMS / Dados móveis (sem driver QMI/RPC)    │
│  ❌ Áudio (sem driver ASoC/ALSA para codecs WCD9320)              │
│  ❌ Câmera (sem driver V4L2 para OV5670/IMX179)                   │
│  ❌ GPS / NFC / Sensores                                           │
│  ❌ Android apps (arquitetura ARMv7 32-bit, API level antigo)      │
│                                                                    │
│  É um **Linux para desenvolvimento de kernel/drivers**,           │
│  NÃO um sistema operacional para uso diário.                      │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

---

## 📋 Pré-requisitos Obrigatórios

| Item | Obrigatório? | Detalhes |
|------|--------------|----------|
| **PC Windows 10/11** (nativo, não VM/WSL) | ✅ SIM | WPInternals só roda no Windows |
| **Cabo USB original Microsoft** | ✅ SIM | Cabo genérico falha no modo recovery |
| **Bateria ≥ 80%** | ✅ SIM | Processo de unlock apaga tudo e pode brickar |
| **Backup completo dos dados** | ✅ SIM | **VAI APAGAR TUDO** — fotos, contatos, 2FA |
| **Aceitar risco de BRICK PERMANENTE** | ✅ SIM | Sem JTAG fácil; test points exigem solda na placa |
| **Linux real para build/flash** | ✅ SIM | Dual boot, VM com USB passthrough, ou live USB |
| **Saber seu RM Code** | ✅ SIM | RM-914 (730 global) ou RM-915 (735 US/CA) |

---

## 🗂 Estrutura do Repositório

```
tools/lumia-linux-port/
├── README.md                    # Este arquivo
├── lumia-port.sh               # Script principal (Linux) — automatiza pmbootstrap
├── wpinternals-guide.md        # Guia Windows (manual — não automatizável)
├── pmbootstrap-config.yaml     # Config pré-definida nokia-superman
├── flash.sh                    # Flash kernel + rootfs via fastboot
├── status-check.sh             # Verifica hardware funcionando pós-flash
├── known-issues.md             # Lista honesta do que NÃO funciona
├── downloads/                  # Binários/FFU baixados (gitignored)
│   ├── WPInternals/            # Extraído do release GitHub
│   └── FFU/                    # Firmware stock .ffu (você coloca)
└── .gitignore                  # Ignora downloads/, builds/, chaves
```

---

## 🚀 Fluxo Completo (Resumo)

### **Fase 1 — Windows (VOCÊ FAZ MANUAL — não automatizável)**

```cmd
# 1. Baixe ferramentas (links no wpinternals-guide.md)
#    → WPInternals.zip
#    → WDRT (recuperação)
#    → FFU stock para SEU RM code (RM-914 ou RM-915)

# 2. Modo Recovery (engrenagens ⚙️):
#    Power OFF → Power + Vol Down (10s) → Solta Power, mantém Vol Down
#    Espera engrenagens → Conecta USB

# 3. WPInternals.exe (Admin) → "Unlock Bootloader"
#    → Seleciona modelo → Aponta FFU → Segue wizard
#    → Sucesso: "Bootloader unlocked"

# 4. Verifica:
#    adb reboot bootloader
#    fastboot getvar unlocked  →  unlocked: yes
```

### **Fase 2 — Linux (ESTE SCRIPT AUTOMATIZA)**

```bash
cd tools/lumia-linux-port

# 1. Setup pmbootstrap (uma vez)
./lumia-port.sh init

# 2. Build kernel + rootfs (30-60 min primeira vez)
./lumia-port.sh build

# 3. Flash no device (precisa fastboot mode)
./lumia-port.sh flash

# 4. Verifica o que funciona
./lumia-port.sh status
```

---

## 📥 O Que Este Repositório **NÃO** Inclui (Você Baixa)

| Arquivo | Onde Baixar | Por que não no repo |
|---------|-------------|---------------------|
| `WPInternals.zip` | GitHub Releases WPInternals | Binário Windows, copyright Microsoft |
| `FFU stock .ffu` | lumiadb.com / firmware.nokia.com | Proprietário Microsoft, específico por RM |
| `WDRT` | Microsoft (link no guide) | Instalador Windows, copyright |

**Coloque em `downloads/` antes de começar:**
```
downloads/
├── WPInternals/
│   └── WPInternals.exe  (extraído do zip)
└── FFU/
    └── RM-914_... .ffu  (OU RM-915_... .ffu)
```

---

## 🎯 Comandos Principais

| Comando | O que faz |
|---------|-----------|
| `./lumia-port.sh init` | Configura pmbootstrap para nokia-superman (downstream kernel, UI=none) |
| `./lumia-port.sh build` | Compila kernel + rootfs (baixa toolchains, compila) |
| `./lumia-port.sh flash` | Flasha boot.img + rootfs.img via fastboot |
| `./lumia-port.sh status` | Verifica hardware detectado no device (CPU, RAM, LEDs, USB) |
| `./lumia-port.sh clean` | Limpa build dir (mantém config) |
| `./lumia-port.sh help` | Mostra ajuda |

---

## 🔗 Links Úteis (Já no `wpinternals-guide.md`)

| Recurso | Link |
|---------|------|
| WPInternals Releases | https://github.com/WPInternals/WPInternals/releases |
| WDRT (Windows Recovery) | https://go.microsoft.com/fwlink/?LinkId=708425 |
| FFU Stock (lumiadb) | https://lumiadb.com/ |
| postmarketOS Device Page | https://wiki.postmarketos.org/wiki/Nokia_Lumia_730_(nokia-superman) |
| Dev Mainline (RayyanAnsari) | https://gitlab.postmarketos.org/RayyanAnsari |
| XDA Lumia 730 Forum | https://xdaforums.com/f/nokia-lumia-730.4529/ |

---

## 📞 Suporte

**Este é um helper para desenvolvedores.** Se você não tem experiência com:
- Kernel Linux / device trees / DTB
- Fastboot / bootloaders / partitions
- pmbootstrap / Alpine Linux / cross-compilation
- Solda / test points / hardware bring-up

**Recomendação forte:** Use este dispositivo como **GPS offline / câmera / MP3** no Windows 10 Mobile (recuperável via WDRT) e foque em dispositivos suportados pelo Conexus Mobile OS (Pixel 8a, Nothing Phone 2).

---

## 📝 Licença

Este helper é **Apache-2.0** (mesmo do Conexus Mobile OS).  
O que ele baixa/instala (postmarketOS, WPInternals, FFU) tem suas próprias licenças.