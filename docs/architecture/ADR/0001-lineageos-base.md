# ADR-001: LineageOS 23.2 como Base Principal

**Status:** Aceito  
**Data:** 21 de outubro de 2026  
**Substitui:** ADR-001 "AOSP como Base Principal" (2026-10-19), em [superseded/](superseded/0001-aosp-primary-base.md)  
**Decisores:** Tech Lead (A0), Architect, Platform (A2), Security (A5)  
**Assinatura:** A0 ✅

---

## Contexto

O Conexus Mobile OS precisa de uma base de sistema operacional que forneça:
- Kernel Linux com drivers para hardware mobile moderno (mainline upstream)
- Android Runtime (ART) para compatibilidade de apps Android
- HAL (Hardware Abstraction Layer) padronizada
- Sistema de build reproduzível (Soong/Blueprint)
- Ecossistema de ferramentas: fastboot, adb, OTA, recovery
- Suporte a verified boot (AVB/dm-verity)
- SELinux policies maduras
- Atualizações de segurança upstream regulares
- Device trees mantidos pela comunidade para hardware suportado

**Alternativas avaliadas:** AOSP puro, Linux mobile (postmarketOS), Windows on ARM, kernel próprio.

---

## Decisão

**Usar LineageOS 23.2 (Android 15 QPR) como base principal para o produto instalável em dispositivos Android modernos.**

- Target: LineageOS 23.2 (branch `lineage-23.2`) + Android 15 (`android-15.0.0_r1`)
- Build: LineageOS upstream + device trees + vendor blobs + Conexus customizations (launcher, services, SDK)
- Distribuição: Imagens assinadas (AVB v2), OTA via GitHub Releases + Pages, release cadence (nightly/weekly/monthly/stable)
- Certificação: Não buscar GMS/Play certification no Alpha; documentar gap; avaliar microG futuramente

---

## Consequências

### Positivas
- Device trees e vendor blobs mantidos pela LineageOS para Pixel 8a (akita) e outros devices
- Build system herdado (Soong/Blueprint + LineageOS customizations)
- Weekly security patches via LineageOS upstream merge
- Community testing e device bring-up já feitos
- Ferramentas maduras: `breakfast`/`brunch`/`mka`, `extract-files.sh`, `lineage-ota-server`
- Compatibilidade Android nativa (apps rodam sem camada de tradução)

### Negativas
- Diferenciação exige trabalho extra (não ser "mais um LineageOS fork")
- Vendor blobs proprietários limitam liberdade e mainline kernel
- Build system complexo (Soong/Blueprint, ~100GB source)
- LineageOS controla upstream; mudanças breaking possíveis
- Certificação GMS/Play não automática

### Riscos
- **R01:** Device sem bootloader unlock → Mitigação: Pixel 8a unlock trivial oficial ✅
- **R02:** Drivers proprietários indisponíveis → Mitigação: Pixel 8a GKI 6.1 mainline ✅
- **R03:** LineageOS upstream breakage → Mitigação: Pin tag `lineage-23.2`, vendor isolation, nightly CI

---

## Alternativas Consideradas

| Alternativa | Prós | Contras | Por que não |
|-------------|------|---------|-------------|
| AOSP puro | Controle total upstream | Device trees/vendor blobs manuais, sem OTA/recovery herdado | Mais trabalho, LineageOS já faz isso |
| Linux mobile (pmOS) | Controle total, mainline kernel | Drivers mobile imaturos, telefonia/câmera/GPU gaps, sem ART | Muitos riscos para produto |
| Windows on ARM | Estuda ecossistema Windows | Dependência firmware Microsoft, drivers limitados | Apenas referência Lumia |
| Kernel próprio | Controle máximo | Complexidade enorme; anos para produto | Descartado inicialmente |

---

## Plano de Implementação

1. **M0:** Device selection define LineageOS tag exata (`lineage-23.2` para Pixel 8a)
2. **M0:** Setup manifest repo `platforms-android-manifest` (LineageOS + Conexus customizations)
3. **M1:** Setup build environment, extract vendor blobs, kernel GKI build
4. **M2:** First reproducible build para Pixel 8a + ConexusLauncher default
5. **M3:** Integração Conexus Services + Shell no system image
6. **M4:** OTA update mechanism, verified boot, security hardening, GitHub Releases + OTA metadata
7. **Contínuo:** Weekly merge LineageOS upstream → Conexus branch

---

## Referências

- [LineageOS 23.2 akita wiki](https://wiki.lineageos.org/devices/akita)
- [LineageOS akita device tree](https://github.com/LineageOS/android_device_google_akita)
- [LineageOS akita vendor](https://github.com/LineageOS/android_vendor_google_akita)
- [LineageOS Build Guide](https://wiki.lineageos.org/devices/akita/build)
- [Android Verified Boot](https://source.android.com/security/verifiedboot)
- [SDD v0.2 §5](./../../SDD.md)