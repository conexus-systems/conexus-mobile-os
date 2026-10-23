# ADR-001: Kernel Linux Mantido

**Status:** Aceito  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect)

---

## Contexto

O projeto Conexus Harmattan visa criar uma plataforma de interface e serviços inspirada no MeeGo Harmattan, rodando sobre Linux existente (Android e Linux desktop). A questão arquitetural fundamental é se devemos reescrever o kernel em Rust ou manter o kernel Linux existente.

## Decisão

**Manter o kernel Linux existente.** Desenvolver todo o core Conexus (serviços, UI, adaptadores) em Rust no espaço de usuário (userspace). Não reescrever o kernel.

## Consequências

### Positivas
- **Viabilidade:** Reescrever kernel é esforço de anos com equipe dedicada; fora do escopo
- **Compatibilidade:** Drivers, hardware, ecossistema Linux/Android preservados
- **Foco:** Equipe foca em diferenciação real (UX, serviços, launcher)
- **Risco reduzido:** Kernel Linux é battle-tested, certificado, suportado por vendors

### Negativas
- **Limitações de kernel:** Não podemos mudar APIs de kernel, scheduling, memory management
- **Dependência de vendors:** Drivers proprietários, blobs, firmware continuam necessários
- **Android constraints:** Sandbox, SELinux, permissions limitam integração profunda

## Alternativas Consideradas

| Alternativa | Avaliação |
|-------------|-----------|
| **Rust kernel (rust-for-linux)** | Imaturo para produção, não suporta Android, drivers incompletos |
| **Redox OS / novo kernel** | Incompatível com Android, hardware support zero |
| **Microkernel (seL4, Fuchsia)** | Mudança radical, ecossistema zero, não roda apps Android |

## Implementação

- Core Rust roda 100% em userspace
- IPC via D-Bus (Linux) / Android Framework (Intents, Binder, Services)
- Kernel modules apenas se estritamente necessário (ADR-009/014) — isolados em `driver-core`
- Boot, init, drivers, power management = responsabilidade do SO base

## Validação

- ✅ Conexus Launcher roda como app/launcher padrão
- ✅ Serviços core comunicam via platform traits (D-Bus/Intents)
- ✅ Zero modificações de kernel necessárias para MVP

---

**Assinatura:** H0, H1  
**Próxima revisão:** Se requisitos mudarem para "sistema operacional próprio" (Fase 8)