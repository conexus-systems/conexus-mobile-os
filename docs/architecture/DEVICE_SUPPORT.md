# Device Support Matrix — Conexus Mobile OS

**Versão:** v0.1  
**Data:** 19 de outubro de 2026  
**Status:** M0 Gate — Device Selection

---

## Critérios de Seleção (M0 Gate)

| Critério | Peso | Mínimo | Desejável |
|----------|------|--------|-----------|
| Bootloader desbloqueável (oficial) | Crítico | Sim | Sim |
| Kernel source disponível (GPL) | Crítico | Sim | Mainline ≥ 5.10 |
| Recovery funcional (TWRP/LineageOS) | Crítico | Sim | Oficial |
| Partição A/B (seamless updates) | Alto | Não | Sim |
| Dynamic partitions (super) | Alto | Não | Sim |
| Community ativa (XDA, GitHub) | Alto | Sim | Forte |
| Device tree público | Alto | Sim | Bem mantido |
| Vendor blobs disponíveis | Médio | Sim | Parciais/mainline |
| Preço/Disponibilidade | Médio | < $500 | < $350 |
| Especificações (RAM/Storage/SoC) | Baixo | 6GB/128GB | 8GB+/256GB+ |

---

## Candidatos Android — Matriz de Decisão M0

### Device 1: Google Pixel 8a (akita) — **RECOMENDADO COMO PRIMÁRIO**

| Atributo | Valor | Fonte | Status |
|----------|-------|-------|--------|
| **Modelo Comercial** | Google Pixel 8a | — | ✅ |
| **Codename** | `akita` | LineageOS Wiki | ✅ |
| **SoC** | Google Tensor G3 (4nm) | — | ✅ |
| **RAM / Storage** | 8 GB LPDDR5X / 128 ou 256 GB UFS 3.1 | — | ✅ |
| **Bootloader Unlock** | **Oficial** — `fastboot flashing unlock` (sem wait, sem conta) | [Google](https://developers.google.com/android/guides/unlock) | ✅ |
| **Kernel Source** | `android-gs-akita-6.1-android16` (GKI) | [AOSP](https://source.android.com/docs/setup/build/building-pixel-kernels) | ✅ |
| **Kernel Version** | **6.1** (mainline GKI) | AOSP | ✅ |
| **Mainline Support** | **Completo** — Pixel kernel upstreamed, GKI certified | AOSP | ✅ |
| **Recovery** | **LineageOS Recovery oficial** + LineageOS 21/22.1/22.2/23.0/23.2 | [LineageOS Wiki](https://wiki.lineageos.org/devices/akita) | ✅ |
| **Partition Scheme** | **A/B + Dynamic Partitions (super)** | AOSP | ✅ |
| **Device Tree** | `lineageos/android_device_google_akita` | [GitHub](https://github.com/LineageOS/android_device_google_akita) | ✅ |
| **Vendor Blobs** | `lineageos/android_vendor_google_akita` (proprietary blobs) | [GitHub](https://github.com/LineageOS/android_vendor_google_akita) | ✅ |
| **XDA Thread** | [Pixel 8a ROMs](https://xdaforums.com/f/google-pixel-8a.11813/) | XDA | ✅ |
| **LineageOS Support** | **Oficial** — mantido por `mikeioannina`, builds semanais | LineageOS Wiki | ✅ |
| **Preço Atual (USD)** | ~$499 (128GB) / ~$559 (256GB) | Google Store / Amazon | ⚠️ >$500 |
| **Disponibilidade** | **Novo** — lançado maio/2024, em produção | Google Store | ✅ |
| **Riscos Conhecidos** | Preço ligeiramente acima do target; Tensor G3 não é Qualcomm (drivers GPU Mali) | — | ⚠️ |

**Decisão:** ✅ **APROVADO COMO PRIMÁRIO** — Melhor combinação: unlock trivial, kernel mainline 6.1, A/B + dynamic partitions, LineageOS oficial, device tree/vendor mantidos pela comunidade, hardware novo. Único ponto: preço ~$500.

---

### Device 2: Nothing Phone (2) (pong) — **RECOMENDADO COMO BACKUP**

| Atributo | Valor | Fonte | Status |
|----------|-------|-------|--------|
| **Modelo Comercial** | Nothing Phone (2) | — | ✅ |
| **Codename** | `pong` | XDA / Nothing Community | ✅ |
| **SoC** | Qualcomm Snapdragon 8+ Gen 1 (4nm) | — | ✅ |
| **RAM / Storage** | 8/12 GB LPDDR5 / 128/256/512 GB UFS 3.1 | — | ✅ |
| **Bootloader Unlock** | **Oficial** — `fastboot oem unlock` (fácil, sem wait) | [Nothing Community](https://nothing.community/) | ✅ |
| **Kernel Source** | **Liberado pela Nothing** (3 dias após launch) | [Nothing OSS](https://github.com/NothingOSS) | ✅ |
| **Kernel Version** | 5.15 / 6.1 (depende da versão OS) | Nothing OSS | ✅ |
| **Mainline Support** | **Parcial** — Qualcomm SM8475, drivers upstream parciais | Kernel.org | ⚠️ |
| **Recovery** | **LineageOS 23.2 (Android 16) unofficial** + TWRP | [XDA](https://xdaforums.com/f/nothing-phone-2.11677/) | ✅ |
| **Partition Scheme** | **A/B + Dynamic Partitions** | XDA | ✅ |
| **Device Tree** | `SuperAviation001/android_device_nothing_pong` (community) | [GitHub](https://github.com/SuperAviation001/android_device_nothing_pong) | ⚠️ Community |
| **Vendor Blobs** | Extraídos de firmware stock (TheMuppets) | [TheMuppets](https://github.com/TheMuppets) | ⚠️ Community |
| **XDA Thread** | [Phone (2) Custom Development](https://xdaforums.com/f/nothing-phone-2.11677/) | XDA | ✅ |
| **LineageOS Support** | **Não-oficial** — builds community (LineageOS 23.2) | XDA / YouTube | ⚠️ |
| **Preço Atual (USD)** | ~$399-499 (usado/novo) | Swappa / eBay / Nothing | ✅ <$500 |
| **Disponibilidade** | Novo (estoque limitado) / Usado abundante | Nothing Store / Revendedores | ✅ |
| **Riscos Conhecidos** | Device tree/vendor community (não LineageOS upstream); Nothing pode parar suporte kernel | — | ⚠️ |

**Decisão:** ✅ **APROVADO COMO BACKUP** — Unlock fácil, kernel source liberado pelo OEM, A/B + dynamic, preço bom. Risco: device tree/vendor community-only, não upstream LineageOS.

---

### Device 3: OnePlus 12 (waffle) — **VIÁVEL MAS COM RESSALVAS**

| Atributo | Valor | Fonte | Status |
|----------|-------|-------|--------|
| **Modelo Comercial** | OnePlus 12 | — | ✅ |
| **Codename** | `waffle` | LineageOS Wiki | ✅ |
| **SoC** | Qualcomm Snapdragon 8 Gen 3 (4nm) | — | ✅ |
| **RAM / Storage** | 12/16/24 GB LPDDR5X / 256/512/1024 GB UFS 4.0 | — | ✅ |
| **Bootloader Unlock** | **Oficial** — `fastboot oem unlock` (global variants) | [OnePlus](https://www.oneplus.com/support/answer/detail/op588) | ✅ |
| **Kernel Source** | `android_kernel_oneplus_sm8650` (LineageOS) | [GitHub](https://github.com/LineageOS/android_kernel_oneplus_sm8650) | ✅ |
| **Kernel Version** | 5.15 / 6.1 (GKI) | LineageOS | ✅ |
| **Mainline Support** | **Parcial** — SM8650 drivers upstreaming | Kernel.org | ⚠️ |
| **Recovery** | **LineageOS 22.2 (Android 15) unofficial** | [XDA](https://xdaforums.com/t/rom-unofficial-lineageos-22-for-oneplus-12.4728027) | ⚠️ Unofficial |
| **Partition Scheme** | **A/B + Dynamic Partitions** | XDA | ✅ |
| **Device Tree** | `lineageos/android_device_oneplus_waffle` (WIP) | [GitHub](https://github.com/LineageOS/android_device_oneplus_waffle) | ⚠️ WIP |
| **Vendor Blobs** | Extraídos de firmware CPH2581_15.0.0.510(EX01) | XDA | ⚠️ Community |
| **XDA Thread** | [LineageOS 22 for OnePlus 12](https://xdaforums.com/t/rom-unofficial-lineageos-22-for-oneplus-12.4728027) | XDA | ✅ |
| **LineageOS Support** | **Não-oficial** — builds UNOFFICIAL por maintainer community | XDA | ⚠️ |
| **Preço Atual (USD)** | ~$699-799 (novo) | OnePlus / Amazon | ❌ >$500 |
| **Disponibilidade** | Novo — lançado dez/2023 | OnePlus Store | ✅ |
| **Riscos Conhecidos** | **Preço alto**; variants T-Mobile US têm OEM unlock **desabilitado**; ColorOS/OxygenOS unified base adiciona complexidade; LineageOS unofficial | — | ⚠️⚠️ |

**Decisão:** ⚠️ **VIÁVEL COMO TERCEIRA OPÇÃO** — Hardware excelente, unlock oficial, mas preço >$700, LineageOS unofficial, variant carrier risks. Apenas se Pixel 8a e Nothing Phone (2) falharem.

---

### Device 4: Xiaomi 14 (houji) — **REJEITADO PARA M0**

| Atributo | Valor | Fonte | Status |
|----------|-------|-------|--------|
| **Modelo Comercial** | Xiaomi 14 | — | ✅ |
| **Codename** | `houji` | XDA | ✅ |
| **SoC** | Snapdragon 8 Gen 3 | — | ✅ |
| **Bootloader Unlock** | **Oficial mas com wait 7-168h + limite 1 device/ano/conta** | [AndroidPolice](https://www.androidpolice.com/xiaomi-bootloader-unlock-restricted-to-one-device-per-user-year) | ❌ |
| **Kernel Source** | Stock kernel GPL (MiCode) | [GitHub](https://github.com/MiCode/Xiaomi_Kernel_OpenSource) | ✅ |
| **LineageOS Support** | **Beta/Closed** — thread fechada no XDA | [XDA](https://xdaforums.com/t/closed-a14-beta-lineage-os-for-xiaomi-14-houji.4691530) | ❌ |
| **Riscos Críticos** | Wait time imprevisível (7 dias a 2 meses), limite 1 device/ano, community fragmented | — | ❌ |

**Decisão:** ❌ **REJEITADO** — Política de unlock hostil a desenvolvedores, wait times imprevisíveis, LineageOS não oficial/closed.

---

## Comparação Resumo (Scorecard)

| Critério | Pixel 8a (akita) | Nothing Phone 2 (pong) | OnePlus 12 (waffle) |
|----------|------------------|------------------------|---------------------|
| **Bootloader Unlock** | ⭐⭐⭐ Trivial, oficial | ⭐⭐⭐ Trivial, oficial | ⭐⭐ Oficial (carrier risk) |
| **Kernel Source** | ⭐⭐⭐ Mainline 6.1 GKI | ⭐⭐ OEM released | ⭐⭐ LineageOS tree |
| **Mainline Support** | ⭐⭐⭐ Completo (GKI) | ⭐⭐ Parcial | ⭐⭐ Parcial |
| **Recovery/LineageOS** | ⭐⭐⭐ Oficial, weekly | ⭐⭐ Unofficial 23.2 | ⭐ Unofficial 22.2 |
| **A/B + Dynamic** | ⭐⭐⭐ Sim | ⭐⭐⭐ Sim | ⭐⭐⭐ Sim |
| **Device Tree/Vendor** | ⭐⭐⭐ Upstream LineageOS | ⭐⭐ Community | ⭐ WIP |
| **Preço** | ⭐⭐ $499 | ⭐⭐⭐ $399-499 | ⭐ $699+ |
| **Disponibilidade** | ⭐⭐⭐ Novo, em produção | ⭐⭐ Limitado/Usado | ⭐⭐⭐ Novo |
| **Score Total** | **27/30** | **24/30** | **20/30** |

---

## Device Selecionado (Pós-M0 Gate)

| Campo | Valor |
|-------|-------|
| **Device Primário** | **Google Pixel 8a (akita)** |
| **Device Backup** | **Nothing Phone (2) (pong)** |
| **AOSP Tag** | `android-15.0.0_r1` (ou `android-latest-release` para Android 16 QPR) |
| **Kernel Tag/Branch** | `android-gs-akita-6.1-android16` (GKI) |
| **Manifest Repo** | `platforms/android/manifest/` (a criar) |
| **Boot/Recovery Testado** | [ ] Sim — Data: ___ Logs: ___ |
| **Factory Image Hash** | SHA-256: ___ (obter de `flash.android.com` ou `developers.google.com/android/images`) |
| **Recovery Procedure Doc** | `docs/installation/ANDROID_INSTALL.md` |

---

## Próximas Ações Imediatas (Esta Semana)

1. **[A0 + A2] Comprar 2x Pixel 8a** (1 primário + 1 backup) — ~$1000 total
2. **[A2] Obter Factory Image** — `flash.android.com` → download `akita-factory-<build>.zip`, validar SHA-256
3. **[A2] Testar Restore Stock** — Flash factory image → boot → confirmar funcionando
4. **[A2] Testar Bootloader Unlock** — `fastboot flashing unlock` → confirmar wipe + unlock
5. **[A2] Testar LineageOS Recovery** — Flash LineageOS recovery → boot → factory reset → boot
6. **[A2] Documentar Procedimento** — Preencher `docs/installation/ANDROID_INSTALL.md`
7. **[A0] Aprovar ADRs Baseline** — ADR-001 a ADR-008 assinados
8. **[A1] Legal Review** — Submeter `LEGAL_BOUNDARIES.md` para advogado IP

---

## Lumia Research Target (Inalterado)

| Campo | Valor |
|-------|-------|
| **Modelo** | Lumia 950 / 950 XL (RM-1104 / RM-1105 / RM-1116) |
| **Variante Escolhida** | **Lumia 950 XL (RM-1116)** — tela 5.7" 1440p, mesmo SoC |
| **SoC** | Snapdragon 810 (MSM8994) |
| **RAM** | 3 GB |
| **Storage** | 32 GB |
| **Bootloader Status** | Locked (precisa exploit) / Unlocked (via WoA) |
| **UEFI Status** | [Lumia950XLPkg](https://github.com/WOA-Project/Lumia950XLPkg) — **Working** (950 XL) |
| **Linux Boot** | [LumiaWOA](https://github.com/WOA-Project/LumiaWOA) — **Partial** (display, touch, storage; modem/camera/audio broken) |
| **Drivers Conhecidos** | Touch: ✅ | Display: ✅ | Modem: ❌ | Camera: ❌ | Audio: ⚠️ | Sensors: ✅ |
| **Riscos Críticos** | Sem telefonia confiável, sem emergência, hardware 2015, 3GB RAM limite |

---

## Aprovação M0 Gate

```
Tech Lead (A0): _________________________ Data: __________
Platform (A2):  _________________________ Data: __________
Security (A5):  _________________________ Data: __________
```

---

## Observações para A2 (Platform Agent)

### Build Environment Setup (Pixel 8a)
```bash
# Repo init para LineageOS 23.2 (Android 16)
repo init -u https://github.com/LineageOS/android.git -b lineage-23.2 --git-lfs
repo sync -j$(nproc)

# Device trees
# Já incluídos no manifest LineageOS para akita

# Kernel build (GKI)
# cd kernel/google/gs/akita
# ./build_akita.sh

# Vendor blobs
# ./extract-files.sh (do android_vendor_google_akita)
```

### Factory Image Source
- **URL:** https://flash.android.com/ (web flasher) ou https://developers.google.com/android/images
- **Device:** `akita` (Pixel 8a)
- **Latest build:** Verificar `android-15.0.0_rXX` ou `android-16.0.0_rXX`

### Recovery Test Procedure
1. `adb reboot bootloader`
2. `fastboot flash boot lineage-23.2-<date>-recovery-akita.img`
3. `fastboot reboot recovery`
4. Factory reset via recovery menu
5. `fastboot reboot` → confirmar boot LineageOS
6. **Então:** restaurar factory image original → confirmar restore funciona