# Conexus Mobile OS — Security Policy

**Versão:** v0.1  
**Data:** 19 de outubro de 2026  
**Status:** Baseline — atualizar a cada release

---

## Princípios de Segurança

1. **Defense in Depth:** Múltiplas camadas (verified boot, SELinux, sandboxing, permissions, encryption)
2. **Least Privilege:** Services rodam com mínimas capabilities necessárias
3. **Supply Chain Integrity:** Signed builds, SBOM, dependency scanning, reproducible builds
4. **Transparency:** Threat model público, vulns divulgadas responsavelmente, no security by obscurity
5. **User Control:** Permissões granulares, telemetria opt-in, chaves do usuário

---

## Threat Model (Baseline M0)

| Ator de Ameaça | Capacidade | Mitigação Primária |
|----------------|------------|-------------------|
| **Malicious App** | Instala app, solicita perms | Permission model, sandbox, SELinux, app signing |
| **Compromised Network** | MITM, DNS spoofing | TLS 1.3 everywhere, certificate pinning, HSTS |
| **Physical Access** | Device roubado, bootloader locked | FBE (File-Based Encryption), verified boot, lockscreen |
| **Supply Chain** | Dependency injection, build tamper | Reproducible builds, SBOM, sigstore, signed artifacts |
| **Kernel Exploit** | Local privilege escalation | Hardened kernel config, kptr_restrict, modules_disabled |
| **Bootloader Unlock** | Flash custom images | Verified boot (AVB), rollback protection, anti-rollback version |

---

## Camadas de Proteção

### 1. Hardware / Boot
- **Verified Boot (AVB v2):** Todos partitions assinados, vbmeta verificado no boot
- **Rollback Protection:** Anti-rollback version em vbmeta, bootloader enforce
- **Bootloader Lock:** Re-lock após flash (onde suportado), unlock = data wipe
- **Hardware Keystore:** StrongBox/TEE para chaves de criptografia, attestation

### 2. Kernel
- **Config Hardening:** `CONFIG_DEBUG_RODATA=y`, `CONFIG_DEBUG_SET_MODULE_RONX=y`, `CONFIG_HARDENED_USERCOPY=y`, `CONFIG_FORTIFY_SOURCE=y`, `CONFIG_KPTR_RESTRICT=2`, `CONFIG_MODULES_DISABLE=1` (production)
- **SELinux:** Enforcing mode, policy custom para Conexus services, `neverallow` rules
- **Memory Protection:** KASLR, KPTI, PAN, PXN, MTE (onde HW suporta)

### 3. System Services
- **Process Isolation:** Cada service = processo separado (ADR-007)
- **IPC Security:** Binder/D-Bus com interface validation, permission checks
- **Capability Bounding:** systemd `CapabilityBoundingSet=` / Android `isolatedProcess=true`
- **Namespace Sandbox:** User namespaces, mount namespaces, network namespaces

### 4. Application Sandbox
- **Android Apps:** UID per app, SELinux `untrusted_app` domain, scoped storage
- **Conexus Apps:** SDK permission model, tile/notification contracts validated
- **Native Code:** NDK r26+, hardened malloc (scudo), CFI, HWASAN (dev builds)

### 5. Data Protection
- **FBE (File-Based Encryption):** Per-user, per-profile, hardware-backed keys
- **Credential Encryption:** Lockscreen credentials derivam KEK para DE keys
- **Keystore:** Android Keystore / Linux kernel keyring para app secrets
- **Backup Encryption:** Backup only com user consent, encrypted

### 6. Network
- **TLS 1.3 Only:** `minVersion = TLSv1.3` em todas conexões
- **Certificate Pinning:** Para API BFFs, update server, telemetry
- **Private DNS:** DoT/DoH por default, user configurable
- **VPN Integration:** System VPN API, per-app routing

### 7. Updates (ADR-008)
- **Signed Payloads:** AVB v2, RSA-2048/ECDSA-P256
- **Integrity Verification:** Client-side antes de write, boot-time via vbmeta
- **Anti-Rollback:** Version em vbmeta, bootloader enforce
- **Staged Rollout:** 1% → 10% → 50% → 100% com health metrics

---

## Gestão de Vulnerabilidades

### Divulgação Responsável
- **Email:** security@conexus.systems (PGP key no repo)
- **Prazo:** 90 dias para fix + coordinated disclosure
- **Bounty:** Programa futuro (pós-Alpha)

### Processo Interno
1. **Detecção:** CI scanning (Grype/Trivy), fuzzing, pentest, relatórios externos
2. **Triagem:** A5 (Security) classifica: Critical/High/Medium/Low (CVSS)
3. **Fix:** Patch em branch `security/<cve>` → test → merge → release
4. **Disclosure:** Advisory no GitHub Security Advisories + mailing list

### SLA de Patch
| Severidade | SLA (dias) | Release Type |
|------------|------------|--------------|
| Critical (CVSS ≥ 9.0) | 7 | Hotfix / OTA emergency |
| High (7.0–8.9) | 14 | Patch release |
| Medium (4.0–6.9) | 30 | Next minor |
| Low (0.1–3.9) | 90 | Next minor |

---

## Supply Chain Security

### Dependências
- **Allowlist Licenças:** Apache-2.0, MIT, BSD-3-Clause, ISC, Unicode-3.0, OFL-1.1
- **Blocklist Licenças:** GPLv3, AGPLv3, LGPLv3 (no runtime), SSPL, Commons Clause
- **SBOM:** SPDX JSON gerado em todo build (Syft)
- **Scanning:** Grype (vulns), Trivy (misconfig), OWASP Dependency Check (CI)

### Build Reprodutível
- **AOSP:** `BUILD_NUMBER` fixo, `BUILD_USERNAME`/`BUILD_HOST` sanitizados
- **Tools:** Bazel/Soong hermeticidade, pinned toolchains
- **Verification:** `diffoscope` entre builds idênticos

### Artefatos Assinados
- **Images:** AVB v2 (boot, dtbo, vbmeta, system, vendor, product, super)
- **SDK Artifacts:** Maven (JAR + POM + sources + javadoc assinados), npm (provenance + attestation)
- **Release:** `cosign` keyless signing ou `age` + `minisign` para artifacts soltos

---

## Políticas Operacionais

### Chaves de Produção
- **Geradas offline** por Tiago (air-gapped machine)
- **Armazenadas:** YubiKey HSM / paper backup em cofre físico
- **Uso:** Apenas para `release/` tags assinadas manualmente
- **Rotação:** Anual + em caso de suspeita de compromisso
- **Revocation:** Publicar revocation list no update server + client check

### Segredos em CI
- **Zero secrets no repo** (pre-commit: `gitleaks`, `trufflehog`)
- **CI Secrets:** GitHub Environments com protection rules + required reviewers
- **Rotação:** 90 dias para CI keys, 30 dias para tokens de acesso

### Auditoria
- **Mensal:** Dependency scan, SBOM review, key inventory
- **Por Release:** Full SBOM, vulnerability scan, license compliance
- **Anual:** Penetration test externo, threat model review

---

## Privacidade

- **Telemetria:** Opt-in apenas, granular (crash, performance, usage), anonimizada
- **Dados Locais:** FBE, sem cloud sync automático
- **Analytics:** Sem Google Analytics, Firebase, Crashlytics — stack próprio via API BFF
- **Retention:** Crash logs 30 dias, metrics agregados 1 ano, raw data não armazenado

---

## Incident Response

| Fase | Ação | Responsável | Prazo |
|------|------|-------------|-------|
| **Detect** | Alert (CI, monitoring, report) | A5 / A6 | Imediato |
| **Triage** | Classificar, isolar se crítico | A0 + A5 | < 4h |
| **Contain** | Mitigação temporária (config, block) | A2 / A5 | < 24h |
| **Eradicate** | Root cause, patch, test | A2 / A4 | Per SLA |
| **Recover** | Deploy fix, verify, monitor | A7 / A6 | < 24h post-fix |
| **Lessons** | Postmortem, update threat model | A0 + A5 | < 7 dias |

---

## Checklist de Release (Security Gate)

Antes de qualquer release (Nightly, Alpha, Beta, Stable):

- [ ] SBOM gerado e validado (SPDX)
- [ ] Vulnerability scan: 0 critical, 0 high não-mitigados
- [ ] License compliance: 0 blocklist violations
- [ ] Imagens assinadas (AVB v2) + hashes SHA-256 publicados
- [ ] Anti-rollback version incrementado
- [ ] SELinux enforcing + no denials em logs de teste
- [ ] FBE verificado (userdata encrypted)
- [ ] Update payload verificado (assinatura + hash)
- [ ] Rollback testado (A/B swap ou recovery flash)
- [ ] Chaves de produção usadas (se Stable) — apenas Tiago
- [ ] Advisory preparado se houver vulns conhecidas mitigadas

---

## Referências

- [ADR-001: AOSP Primary Base](../docs/architecture/ADR/0001-aosp-primary-base.md)
- [ADR-005: Recovery Before Flash](../docs/architecture/ADR/0005-recovery-before-flash.md)
- [ADR-007: System Services Isolated](../docs/architecture/ADR/0007-system-services-isolated.md)
- [ADR-008: OTA Updates](../docs/architecture/ADR/0008-ota-updates-ab-partitions.md)
- [Android Security Best Practices](https://source.android.com/security)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)