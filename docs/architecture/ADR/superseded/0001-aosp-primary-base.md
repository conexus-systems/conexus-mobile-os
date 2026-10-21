# ADR-001: AOSP como Base Principal para Produto — **SUBSTITUÍDO**

**Status:** Substituído por ADR-001 "LineageOS 23.2 como Base Principal" (2026-10-21). Mantido só como histórico.
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Platform (A2), Security (A5)
**Assinatura:** (anulada pela substituição)

---

## Contexto

O Conexus Mobile OS precisa de uma base de sistema operacional que forneça:
- Kernel Linux com drivers para hardware mobile moderno
- Android Runtime (ART) para compatibilidade de apps Android
- HAL (Hardware Abstraction Layer) padronizada
- Sistema de build reproduzível (Soong/Blueprint)
- Ecossistema de ferramentas: fastboot, adb, OTA, recovery
- Suporte a verified boot (AVB/dm-verity)
- SELinux policies maduras
- Atualizações de segurança upstream regulares

Alternativas avaliadas: Linux mobile puro (postmarketOS, pmOS), Windows on ARM, kernel próprio.

---

## Decisão

**Usar Android Open Source Project (AOSP) como base principal para o produto instalável em dispositivos Android modernos.**

- Target inicial: **Android 15 (android-15.0.0_r1)** com kernel **6.1 GKI** para Pixel 8a (akita)
- Build: AOSP upstream + device trees + vendor blobs + Conexus customizations (shell, services, SDK)
- Distribuição: Imagens assinadas (AVB v2), OTA via update engine próprio
- Certificação: Não buscar GMS/Play certification no Alpha; documentar gap; avaliar microG futuramente

---

## Consequências

### Positivas
- Drivers e suporte de hardware "grátis" para dispositivos suportados
- Compatibilidade Android nativa (apps rodam sem camada de tradução)
- Ferramentas maduras: build, flash, debug, OTA, recovery
- Upstream security patches mensais
- Ecossistema de desenvolvedores e documentação vasta

### Negativas
- Diferenciação exige trabalho extra (não ser "mais um AOSP fork")
- Vendor blobs proprietários limitam liberdade e mainline kernel
- Build system complexo (Soong/Blueprint, ~100GB source)
- Google controla upstream; mudanças breaking possíveis
- Certificação GMS/Play não automática

### Riscos
- **R01:** Device sem bootloader unlock → Mitigação: M0 valida 3 candidatos antes de comprar ✅ **RESOLVIDO** (Pixel 8a unlock trivial)
- **R02:** Drivers proprietários indisponíveis → Mitigação: Priorizar devices com mainline kernel support ✅ **RESOLVIDO** (Pixel 8a GKI 6.1 mainline)
- **R03:** AOSP upstream breakage → Mitigação: Pin tag, vendor isolation, nightly CI

---

## Plano de Implementação

1. **M0:** Device selection define AOSP tag exata (`android-15.0.0_r1` para Pixel 8a)
2. **M1:** Setup build environment, manifest repo (`platforms/android/manifest/`)
3. **M2:** First reproducible build para device alvo
4. **M3:** Integração Conexus Shell + Services no system image
5. **M4:** OTA update mechanism, verified boot, security hardening

---

## Referências

- [Android Compatibility Definition](https://source.android.com/compatibility)
- [AOSP Build System](https://source.android.com/setup/build)
- [AVB (Android Verified Boot)](https://source.android.com/security/verifiedboot)
- [SDD v0.1 §3, §4.1, §10](./../../SDD.md)