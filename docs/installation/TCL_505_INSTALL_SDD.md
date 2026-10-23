# SDD — Instalação do Conexus Mobile OS no TCL 505

**Versão:** v0.1  
**Data:** 22 de outubro de 2026  
**Status:** **PESQUISA (M0)** — Pré-requisitos não atendidos, nenhuma imagem existe  
**Documento:** Projeto de especificação para instalação futura, condicionado a viabilidade

---

## 1. Resumo Executivo

O **TCL 505 (modelos T509K1/T509K3)** é um smartphone de entrada com SoC **MediaTek Helio G36** (ou G35/MT6765G, conflitante em fichas técnicas), Android 14 de fábrica, tela 6.75" 720p 90 Hz IPS, bateria 5.010 mAh.

**Estado atual (2026-10-22):**

| Critério (ADR-010) | Estado | Ação Necessária |
|--------------------|--------|-----------------|
| **C1 — Desbloqueio oficial** | ❓ Desconhecido | Testar `fastboot flashing unlock` / `fastboot oem unlock` |
| **C2 — Fonte GPL do kernel** | ❓ Desconhecido | Consultar `opensource.tcl.com`, solicitar à TCL |
| **C3 — Device tree público** | ❌ Não encontrado | Criar do zero se C1+C2 viáveis |
| **C4 — Kernel compatível (≥5.10)** | ❓ Desconhecido | Verificar versão via `uname -r` no aparelho |
| **C5 — Recovery testado** | ❌ Não testado | Requer C1–C4 primeiro |

**Conclusão:** **Não é possível instalar o Conexus Mobile OS no TCL 505 hoje.** Não existe device tree, kernel source, nem confirmação de bootloader desbloqueável. Qualquer tentativa de flash de imagem de outro aparelho **irá brickar o dispositivo**.

Este SDD define o **plano de viabilidade** e, condicionalmente, o **procedimento de instalação futuro** uma vez que os pré-requisitos sejam atendidos.

---

## 2. Identificação do Dispositivo

### 2.1 Especificações Conhecidas (Fontes Públicas)

| Atributo | Valor | Fonte | Confiança |
|----------|-------|-------|-----------|
| **Modelo Comercial** | TCL 505 | GSMArena, PhoneArena, TuttoAndroid | Alta |
| **Modelo Interno** | T509K1 (4GB/128GB), T509K3 (6GB/256GB) | Varejistas | Média (varia por região) |
| **SoC** | MediaTek Helio G36 **ou** Helio G35 (MT6765G) | Fichas técnicas conflitantes | **Conflitante — ler do aparelho** |
| **CPU** | 8× Cortex-A53 @ 2.2 GHz (G36) ou 2.3 GHz (G35) | MediaTek specs | Média |
| **GPU** | PowerVR GE8320 | MediaTek specs | Média |
| **RAM** | 4 GB ou 6 GB LPDDR4X | Varejistas | Média |
| **Storage** | 128 GB ou 256 GB eMMC 5.1 | Varejistas | Média |
| **Display** | 6.75" IPS LCD, 720×1600 (HD+), 90 Hz, 20:9 | GSMArena | Alta |
| **Bateria** | 5.010 mAh, 10 W USB-C | GSMArena | Alta |
| **Câmera Traseira** | 50 MP (wide) + 2 MP (depth) | GSMArena | Alta |
| **Câmera Frontal** | 5 MP | GSMArena | Alta |
| **Android de Fábrica** | Android 14 (não-Go) | GSMArena | Alta |
| **Conectividade** | 4G, Wi-Fi 5, BT 5.0, GPS, USB-C 2.0 | GSMArena | Alta |
| **Biometria** | Sensor lateral (power button) | GSMArena | Alta |
| **Dimensões** | 165.7 × 76.2 × 8.9 mm, 195 g | GSMArena | Alta |

### 2.2 Propriedades a Coletar do Aparelho (Inventário Obrigatório)

Executar no dispositivo com USB debugging ativo:

```bash
# Via ADB
adb shell getprop ro.product.device          # Codinome interno (ex: tcl_t509k1)
adb shell getprop ro.product.model           # Modelo comercial
adb shell getprop ro.product.manufacturer    # Fabricante
adb shell getprop ro.build.version.release   # Android version
adb shell getprop ro.build.version.sdk       # API level
adb shell getprop ro.build.fingerprint       # Build fingerprint completo
adb shell getprop ro.bootloader              # Bootloader version
adb shell getprop ro.boot.flash.locked       # 0 = unlocked, 1 = locked
adb shell uname -r                           # Kernel version
adb shell cat /proc/partitions               # Partition layout
adb shell ls -la /vendor/lib/modules/        # Kernel modules (MediaTek)
adb shell ls -la /vendor/lib/hw/             # HALs
adb shell ls -la /vendor/lib64/hw/           # HALs 64-bit
```

**Ferramenta do projeto:** `conexus-installer inventory --out tcl505.json` (lê apenas, não grava)

---

## 3. Análise de Viabilidade (ADR-010)

### 3.1 Critério C1 — Bootloader Desbloqueável Oficialmente

**Status:** ❓ **DESCONHECIDO**

**Investigação necessária:**
1. Ativar "Desbloqueio de OEM" em **Configurações > Sistema > Opções do desenvolvedor**
2. `adb reboot bootloader`
3. `fastboot flashing unlock` (Android 10+) ou `fastboot oem unlock` (legado)
4. Observar se aparece tela de confirmação no dispositivo

**Cenários:**
- ✅ **Sucesso:** Bootloader desbloqueado, device wipe automático → prosseguir
- ⚠️ **Opção não aparece:** Bootloader travado permanentemente → **FIM DO CAMINHO**
- ⚠️ **Erro "Command not allowed":** Requer autorização de conta/servidor → investigar

### 3.2 Critério C2 — Fonte GPL do Kernel

**Status:** ❓ **DESCONHECIDO**

**Fontes a consultar:**
- `https://opensource.tcl.com/` (portal oficial — estava fora do ar em 2026-10-22)
- `https://github.com/TCLCommunication` (organização GitHub)
- Solicitação formal via e-mail GPL compliance: `opensource@tcl.com`

**Requisito mínimo:** Kernel source tarball correspondente **exato** à versão no aparelho (`uname -r`).

### 3.3 Critério C3 — Device Tree

**Status:** ❌ **NÃO EXISTE PUBLICAMENTE**

**Buscas realizadas (2026-10-22):**
- GitHub: `tcl 505`, `t509k`, `tcl_t509`, `mediatek helio g36 lineageos` → 0 resultados relevantes
- LineageOS Wiki: nenhum device TCL com codinome similar
- XDA: nenhum thread de desenvolvimento para TCL 505

**Se C1+C2 viáveis:** Device tree deve ser criado do zero baseado em:
- `device/mediatek/common` (common MTK tree)
- `device/mediatek/sepolicy` (SELinux policies MTK)
- Kernel config extraído do aparelho (`/proc/config.gz` ou `ikconfig`)

### 3.4 Critério C4 — Kernel Compatível

**Status:** ❓ **DESCONHECIDO — LER DO APARELHO**

**Requisitos (ADR-010):**
- **Mínimo:** Linux 5.10 LTS (suporte GKI)
- **Desejável:** 5.15 / 6.1 / 6.6 LTS
- **MediaTek Helio G36/G35:** Tipicamente shipa com 4.19 ou 5.4 (legado)

**Se kernel < 5.10:** Classificação **"Aparelho Legado"** (ADR-010)
- Apenas driver em userspace (HAL Rust) — **sem módulo de kernel**
- Nenhuma modificação de kernel permitida
- Limitações: sem GKI, sem mainline, suporte limitado

### 3.5 Critério C5 — Recovery Testado

**Status:** ❌ **NÃO TESTADO**

**Requer:** C1–C4 concluídos com sucesso.

---

## 4. Arquitetura de Instalação Projetada (Condicional)

> **Nota:** Esta seção descreve o design **se e quando** todos os pré-requisitos forem atendidos.

### 4.1 Estratégia de Base

| Opção | Viabilidade | Justificativa |
|-------|-------------|---------------|
| **LineageOS 23.2 (Android 15/16) como upstream** | ⚠️ Condicional | Requer device tree + kernel source; MediaTek support em LineageOS é limitado |
| **GSI (Generic System Image)** | ✅ Fallback | Se bootloader unlock funcionar mas sem device tree, GSI + phhusson treble pode bootar |
| **Port direto AOSP** | ❌ Não recomendado | Muito esforço para device único, sem upstream |

**Recomendação:** Tentar **LineageOS 23.2** primeiro (mesma base do Pixel 8a/Nothing Phone 2). Se falhar, **GSI + phhusson** como fallback.

### 4.2 Particionamento Esperado (MediaTek Típico)

```text
/boot                 → boot.img (kernel + ramdisk)
/dtbo                 → dtbo.img (device tree overlay)
/vbmeta               → vbmeta.img (AVB)
/vbmeta_system        → vbmeta_system.img
/vbmeta_vendor        → vbmeta_vendor.img
/super                → super.img (dynamic partitions: system, vendor, product, odm)
    ├─ /system
    ├─ /vendor
    ├─ /product
    └─ /odm
/recovery             → recovery.img
```

**Verificar no aparelho:** `cat /proc/partitions` + `ls -la /dev/block/by-name/`

### 4.3 Componentes Conexus para TCL 505

| Componente | Adaptação Necessária |
|------------|---------------------|
| **Conexus Launcher** | Funciona out-of-the-box (Kotlin + Views, sem Compose) |
| **Conexus Services** | AIDL/Binder — compatível com qualquer Android 15+ |
| **Fluent/Metro UI** | Tokens independentes de hardware |
| **Driver Module (ADR-014/016)** | **Obrigatório** — HAL Rust para MediaTek (sensors, audio, camera, power, display) |
| **Vendor Blobs** | Extraídos do aparelho do usuário (**nunca redistribuídos**) |
| **Installer** | Adicionar `tcl_t509k1` / `tcl_t509k3` ao `SUPPORTED_DEVICES` |

### 4.4 Manifest Repo (platforms/android/manifest)

```xml
<!-- Adicionar ao default.xml quando device tree existir -->
<project path="device/tcl/t509k1" name="conexus-device-tcl-t509k1" remote="conexus" revision="main" />
<project path="kernel/mediatek/mt6765" name="conexus-kernel-mt6765" remote="conexus" revision="main" />
<project path="vendor/tcl/t509k1" name="conexus-vendor-tcl-t509k1" remote="conexus" revision="main" />
```

### 4.5 Build Target

```bash
# Quando device tree existir
lunch lineage_t509k1-userdebug
m -j$(nproc) bacon
```

**Output esperado:** `out/target/product/t509k1/lineage_t509k1-target_files-*.zip` → OTA package

---

## 5. Procedimento de Instalação (Design Futuro)

> **Este procedimento só é executável APÓS:** build reproduzível existir + release publicada no GitHub Releases.

### 5.1 Pré-requisitos do Usuário

- [ ] TCL 505 com bootloader **desbloqueado** (C1 confirmado)
- [ ] Cabo USB-C de dados (não apenas carregamento)
- [ ] Computador com Linux/macOS/Windows + Android Platform Tools (`adb`, `fastboot`)
- [ ] Bateria ≥ 30%
- [ ] **Backup completo dos dados** (processo apaga tudo)
- [ ] Imagem de fábrica TCL baixada (para restore se necessário)

### 5.2 Modo Automático (Instalador Conexus)

```bash
# 1. Baixar instalador
curl -L https://github.com/conexus-mobile-os/conexus-installer/releases/latest/download/conexus-installer-linux -o conexus-installer
chmod +x conexus-installer

# 2. Conectar TCL 505 em fastboot mode (Power + Vol Down)
# 3. Executar
./conexus-installer install --device t509k1 --channel stable

# Ou dry-run primeiro
./conexus-installer install --device t509k1 --channel stable --dry-run
```

**O instalador fará:**
1. Detectar dispositivo via `fastboot getvar product`
2. Baixar release correspondente do GitHub Releases
3. Verificar SHA-256 + assinatura minisign
4. Flash partitions: `boot`, `dtbo`, `vbmeta*`, `super`, `recovery`
5. Reboot + verificação de boot

### 5.3 Modo Manual (Sideload / Fastboot Direto)

```bash
# 1. Baixar OTA package
wget https://github.com/conexus-mobile-os/conexus-mobile-os/releases/download/vX.Y.Z-stable-t509k1/conexus-t509k1-vX.Y.Z-stable.zip

# 2. Verificar SHA-256
sha256sum conexus-t509k1-vX.Y.Z-stable.zip
# Comparar com SHA256SUMS do release

# 3. Reboot to recovery
adb reboot recovery
# Ou: fastboot boot recovery.img (se tiver recovery.img separado)

# 4. Sideload (LineageOS Recovery)
adb sideload conexus-t509k1-vX.Y.Z-stable.zip

# 5. Factory reset (wipe data/cache)
# 6. Reboot to system
```

### 5.4 Modo Fastboot Direto (Partition Images)

```bash
# Extrair OTA package
unzip conexus-t509k1-vX.Y.Z-stable.zip -d t509k1-images/

# Flash cada partição
fastboot flash boot       t509k1-images/boot.img
fastboot flash dtbo       t509k1-images/dtbo.img
fastboot flash vbmeta     t509k1-images/vbmeta.img
fastboot flash vbmeta_system t509k1-images/vbmeta_system.img
fastboot flash vbmeta_vendor t509k1-images/vbmeta_vendor.img
fastboot flash super      t509k1-images/super.img
fastboot flash recovery   t509k1-images/recovery.img

# Reboot
fastboot reboot
```

---

## 6. Driver Module — Especificação (ADR-014/016)

O TCL 505 **exige** módulo de driver específico por ser MediaTek.

### 6.1 Estrutura do Pacote (userspace HAL em Rust)

```
conexus-drivers-tcl/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Entry point: register_hal()
│   ├── audio.rs            # Audio HAL (media.audiopolicy, primary, r_submix)
│   ├── camera.rs           # Camera HAL (media.camera.provider)
│   ├── display.rs          # Display/HWC HAL (graphics.composer, mapper)
│   ├── gnss.rs             # GNSS HAL (gnss)
│   ├── power.rs            # Power HAL (power.stats, power.hint)
│   ├── sensors.rs          # Sensors HAL (sensors)
│   ├── thermal.rs          # Thermal HAL (thermal)
│   ├── vibrator.rs         # Vibrator HAL (vibrator)
│   ├── wifi.rs             # WiFi HAL (wifi.supplicant, wifi.chip)
│   └── bluetooth.rs        # BT HAL (bluetooth)
├── vendor-blobs/           # VAZIO NO REPO — extraído localmente
│   ├── README.md           # Instruções: ./conexus-installer extract
│   ├── AVISO.txt           # "Arquivos da TCL/MediaTek — NÃO REDISTRIBUIR"
│   ├── extract-manifest.json
│   └── .gitignore          # *
└── tests/
```

### 6.2 Blobs de Vendor Necessários (Extraídos do Aparelho do Usuário)

| Caminho no Device | Conteúdo | Uso |
|-------------------|----------|-----|
| `/vendor/lib/modules/` | `.ko` kernel modules | Carregados pelo init se kernel compatível |
| `/vendor/lib/hw/` / `/vendor/lib64/hw/` | `*.so` HALs (audio, camera, sensors, etc.) | `dlopen` pelo HAL Rust via `libloading` |
| `/vendor/bin/hw/` | Binários HAL (ex: `android.hardware.camera.provider@2.4-service`) | Spawn como subprocesso |
| `/vendor/firmware/` | Firmware binários (DSP, modem, touch, etc.) | Carregados via `request_firmware` |
| `/vendor/etc/init/` | `.rc` init scripts | Referência para recriar serviços |
| `/vendor/etc/vintf/` | `manifest.xml`, `compatibility_matrix.xml` | Declaração de HALs suportados |

**Regra de ouro:** Blobs **nunca** entram no repositório. O instalador extrai localmente (`conexus-installer extract --out ./tcl505-vendor`) e o build system faz `dlopen` em tempo de compilação/link.

---

## 7. Plano de Ação (Roadmap TCL 505)

### Fase 0 — Inventário (Esta Semana) — **BLOQUEADOR**

| Tarefa | Responsável | Critério de Sucesso |
|--------|-------------|---------------------|
| Executar `conexus-installer inventory` no TCL 505 físico | Usuário / A2 | `tcl505.json` com: codinome, kernel version, bootloader status, partition scheme, A/B status |
| Verificar "Desbloqueio de OEM" nas opções do desenvolvedor | Usuário | Screenshot ou confirmação |
| Tentar `fastboot flashing unlock` | Usuário / A2 | Sucesso = bootloader unlocked; Falha = fim do caminho |
| Solicitar kernel source à TCL (GPL) | A0 / Legal | Resposta com link ou confirmação de não-disponibilidade |

### Fase 1 — Viabilidade Confirmada (Se C1+C2 OK)

| Tarefa | Responsável | Critério de Sucesso |
|--------|-------------|---------------------|
| Extrair vendor blobs: `conexus-installer extract --out ./tcl505-vendor` | Usuário / A2 | `extract-manifest.json` com SHA-256 de cada blob |
| Criar device tree inicial (`device/tcl/t509k1/`) | A2 | `lineage_t509k1-userdebug` compila (mesmo que não boot) |
| Build kernel MediaTek (config do aparelho) | A2 | `Image.gz` + `dtbo.img` gerados |
| Testar boot + recovery em hardware real | A2 | 5/5 boots sucessivos, recovery funcional |

### Fase 2 — Integração Conexus

| Tarefa | Responsável | Critério de Sucesso |
|--------|-------------|---------------------|
| Adicionar TCL 505 ao `SUPPORTED_DEVICES` no instalador | A2 / A4 | Instalador detecta `t509k1` / `t509k3` |
| Escrever HALs Rust para MediaTek (áudio, sensores, power, display) | A4 | `cargo build --release` passa, unit tests verdes |
| Build LineageOS 23.2 + Conexus para t509k1 | A2 | OTA package gerado, assinado dev key |
| Testes estendidos (7 dias): boot, UI, telefonia, WiFi, BT, câmera, bateria | A6 | Zero critical crashes, battery ≤ metas |

### Fase 3 — Release Alpha

| Tarefa | Responsável | Critério de Sucesso |
|--------|-------------|---------------------|
| Publicar release no GitHub Releases | A7 | Assets: OTA zip, partition images, SHA256SUMS, assinatura |
| Atualizar OTA metadata (`ota_metadata.json`) | A7 | Canal `stable` com `rollout_percentage: 5` |
| Documentar `ANDROID_INSTALL_TCL505.md` | A7 | Procedimento completo com screenshots |
| Site: adicionar TCL 505 à matriz `/devices` | A7 | Status "Alpha — Limited Support" |

---

## 8. Riscos e Mitigações

| Risco | Probabilidade | Impacto | Mitigação |
|-------|---------------|---------|-----------|
| Bootloader **permanentemente travado** | Alta (TCL histórico) | **Bloqueio total** | Documentar como "unsupported", focar em dispositivos viáveis |
| Kernel source **não liberado** pela TCL | Alta | **Bloqueio total** | Tentar GSI + phhusson treble; se falhar, descartar device |
| Kernel **< 5.10** (legado) | Média | Limitações severas | Classificar como "Legado" — apenas userspace HAL, sem kernel mods |
| Device tree **muito complexo** (MediaTek) | Média | Atraso meses | Priorizar Pixel 8a / Nothing Phone 2; TCL 505 = best effort |
| Blobs vendor **incompatíveis** com Android 15/16 | Média | Quebra HALs | Manter Android 14 base se necessário; documentar limitação |
| Hardware **insuficiente** (4GB RAM, eMMC, Helio G36) | Baixa | UX degradada | Definir requisitos mínimos: 6GB RAM, UFS, SoC ≥ G99 |

---

## 9. Decisão de Go/No-Go (Gate TCL 505)

```text
┌─────────────────────────────────────────────────────────────┐
│                    TCL 505 GO/NO-GO GATE                      │
├─────────────────────────────────────────────────────────────┤
│ C1 Bootloader Unlock:    [ ] PASS   [ ] FAIL  → NO-GO       │
│ C2 Kernel Source GPL:    [ ] PASS   [ ] FAIL  → NO-GO       │
│ C3 Device Tree Criado:   [ ] PASS   [ ] FAIL  → NO-GO       │
│ C4 Kernel ≥ 5.10:        [ ] PASS   [ ] FAIL  → LEGACY ONLY │
│ C5 Recovery Funcional:   [ ] PASS   [ ] FAIL  → NO-GO       │
├─────────────────────────────────────────────────────────────┤
│ DECISÃO FINAL:          [ ] GO      [ ] NO-GO  [ ] LEGACY   │
│ Assinatura A0: ________________________ Data: __________    │
│ Assinatura A2: ________________________ Data: __________    │
│ Assinatura A5: ________________________ Data: __________    │
└─────────────────────────────────────────────────────────────┘
```

---

## 10. Referências

- **ADR-010:** Device Support Criteria — `docs/architecture/ADR/0010-device-support-criteria.md`
- **ADR-014:** Driver Bundles (APK Container) — `docs/architecture/ADR/0014-driver-bundles.md`
- **ADR-016:** Modular Platform (Android, Rust, Go) — `docs/architecture/ADR/0016-modular-platform.md`
- **SDD Principal:** `SDD.md` (seção 5, 11, 12)
- **Device Support Matrix:** `docs/architecture/DEVICE_SUPPORT.md`
- **Inventário TCL 505 (PR #11):** `docs/devices/tcl-505.md`
- **Instalador:** `scripts/install_conexus.py` + `tools/installer/`
- **Fontes Públicas:** GSMArena, PhoneArena, TuttoAndroid (links na seção 2.1)

---

## 11. Próximos Passos Imediatos (Para Esta Semana)

1. **[Usuário]** Executar inventário: `cd tools/installer && go build -o conexus-installer ./cmd/conexus-installer && ./conexus-installer inventory --out tcl505.json`
2. **[Usuário]** Verificar "Desbloqueio de OEM" nas Opções do Desenvolvedor
3. **[Usuário]** Tentar `fastboot flashing unlock` (com backup de dados feito!)
3. **[A0]** Enviar `tcl505.json` para análise da equipe
4. **[A0]** Solicitar kernel source à TCL (compliance GPL)
5. **[A2]** Se C1+C2 PASS: iniciar device tree + kernel build

---

**Este documento é vivo.** Será atualizado a cada fase com dados reais do aparelho. Não existe instalação sem passar pelo Gate acima.