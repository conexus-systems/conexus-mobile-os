# ADR-008: OTA Updates com Partições A/B ou Recovery Fallback — APROVADO

**Status:** Aceito
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Platform (A2), Security (A5), Release (A7)
**Assinatura:** A0

---

## Contexto

Atualizacoes over-the-air (OTA) sao criticas para:
- Security patches mensais
- Bug fixes
- Feature delivery
- Rollback em caso de regressao

Android suporta dois modelos:
1. A/B (Seamless) System Updates: Duas particoes (boot, dtbo, vbmeta, system, vendor, product, etc.), update na inativa, swap no reboot. Suportado desde Android 7.1.
2. Non-A/B (Traditional): Single partition set, update via recovery mode, mais arriscado.

Device selection (M0) determinara qual o device suporta. A/B e fortemente preferivel.

Requisitos Conexus:
- Imagens assinadas (AVB v2)
- Verificacao de integridade antes de boot
- Rollback protection (anti-rollback version)
- Recovery automatico se boot falhar (A/B) ou recovery mode (non-A/B)
- Delta updates (opcional, bandwidth)

---

## Decisao

Implementar OTA updates usando particoes A/B onde device suportar; fallback para non-A/B com recovery mode onde nao suportar.

### Estrategia por Device

| Device Type | Partition Scheme | Update Mechanism | Rollback |
|-------------|------------------|------------------|----------|
| A/B capable (Pixel, OnePlus 11+, Xiaomi 13+, etc.) | A/B (slot_a/slot_b) | update_engine + bootctrl | Auto: boot failed -> swap slot |
| Non-A/B (older devices, some Xiaomis) | Single + recovery | Recovery mode + update_package | Manual: recovery -> flash previous |

### Implementacao Conexus

**Comum (ambos):**
- Payload: super partition image (dynamic partitions) ou individual partition images
- Assinatura: AVB v2 (RSA-2048/SHA-256 ou ECDSA-P256)
- Metadata: payload_properties.txt com version, security patch level, device, build fingerprint
- Verificacao: avbtool verify_image no client + vbmeta verification no boot
- Server: HTTPS endpoint com JSON manifest (version, url, hash, size, signature)

**A/B (preferido):**
- Usar update_engine (AOSP) + custom ConexusUpdateService (IPC)
- Stream download -> write to inactive slot -> bootctrl mark-boot-successful no proximo boot
- Health check: service verifica boot success, reporta ao server
- Rollback: bootctrl set-active-boot-slot previous slot

**Non-A/B (fallback):**
- Download -> recovery partition -> update_package script
- Requer user action (reboot to recovery) ou auto-reboot se misc partition command
- Rollback: flash previous images via recovery (manual ou scriptado)

### Chaves de Assinatura

| Ambiente | Chave | Armazenamento | Rotacao |
|----------|-------|---------------|---------|
| Dev | dev-key.pem | Repo (git-crypt ou age-encrypted) | Sem rotacao |
| CI/Nightly | ci-key.pem | CI secrets (GitHub Environments) | 90 dias |
| Alpha/Beta | alpha-key.pem | Offline HSM / YubiKey / age-encrypted | 180 dias |
| Production | prod-key.pem | Apenas Tiago — offline, air-gapped | 365 dias |

Regra: Chaves de producao NUNCA em CI, repo, agentes, servers. Assinatura de release = processo manual documentado.

---

## Consequencias

### Positivas
- A/B = seamless, safe, auto-rollback (padrao Android moderno)
- AVB v2 = verified boot chain, anti-rollback
- Chaves segregadas por ambiente = supply chain security
- Fallback non-A/B = suporta mais devices

### Negativas
- A/B requer device support (particao scheme, bootloader)
- Non-A/B = user friction, maior risco de brick
- Dois caminhos de implementacao = mais codigo
- Dynamic partitions (super) adiciona complexidade

### Riscos
- Device nao tem A/B: Mitigacao: M0 device selection prioriza A/B ✅ RESOLVIDO (Pixel 8a = A/B)
- Update_engine bugs: Mitigacao: Usar AOSP upstream, testar exaustivamente M4
- Key compromise: Mitigacao: Prod key offline, rotacao, revocation list no client

---

## Alternativas Consideradas

| Alternativa | Pros | Contras | Por que nao |
|-------------|------|---------|-------------|
| So A/B, nao suportar non-A/B | Codigo simples | Limita devices drasticamente | Exclui muitos devices viaveis |
| Custom update engine (nao AOSP) | Controle total | Reimplementar verified boot, rollback, delta = enorme effort | Reinventar roda |
| FOTA via app (nao system) | Simples | Nao atualiza kernel, vendor, boot, system partitions | Nao e SO update |
| Balena/Resin.io style (container updates) | Atomic, rollback easy | Requer container runtime, nao atualiza kernel/boot | Nao adequado para base OS |

---

## Plano de Implementacao

1. M0: Device selection confirma A/B support (ou nao) ✅ Pixel 8a = A/B
2. M2: Verified boot (AVB) configurado e testado
3. M3: ConexusUpdateService skeleton + payload format definido
4. M4:
   - A/B: update_engine integration + auto-rollback test (5 ciclos)
   - Non-A/B: Recovery update script + manual rollback test
5. M4 Gate: Security audit (A5) aprova: assinaturas, rollback, anti-rollback
6. Post-M4: Delta updates, staged rollout, telemetry opt-in

---

## Referencias

- Android A/B Updates (https://source.android.com/docs/core/ota/ab_updates)
- Android Verified Boot (AVB) (https://source.android.com/security/verifiedboot)
- update_engine (https://source.android.com/docs/core/ota/update_engine)
- Dynamic Partitions (https://source.android.com/docs/core/ota/dynamic_partitions)
- SDD v0.1 4.3, 10