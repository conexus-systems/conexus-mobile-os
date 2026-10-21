# ADR-012: Rust para o núcleo nativo novo — **PROPOSTO**

**Status:** Proposto (complementa ADR-004; não substitui o SDK Kotlin de UI)
**Data:** 2026-10-21
**Decisores:** Tech Lead (A0), Platform (A2), Security (A5)
**Assinatura:** pendente

---

## Contexto

O pedido foi construir o sistema em Rust. O SDD e o ADR-004 previam Kotlin para o SDK Android e TypeScript para o cross-platform. Um sistema operacional não é só código de app: há serviços de sistema, gerenciador de drivers, validação de pacotes e, no futuro, possivelmente módulos de kernel.

Limites reais:

- O framework Android (Java/Kotlin/C++) e a UI do launcher não passam para Rust sem reescrever o framework. Isso fica fora do escopo.
- Rust está presente no AOSP para componentes de sistema, e o kernel 6.1 pode compilar módulos em Rust, mas isso depende de configuração (`CONFIG_RUST`) e de cada kernel. Precisa ser confirmado no Pixel 8a (leitura, sem flash).
- Lumia (UEFI/Linux) suporta Rust no userspace sem restrição.

---

## Decisão proposta

**Rust para todo código nativo novo de sistema e de SDK**, com escopo:

| Camada | Linguagem | Motivo |
|---|---|---|
| Serviços `conexus-*` (tile-engine, notifications, app-manager, update, security) | Rust | Segurança de memória em processos com privilégio |
| `conexus-driver-manager` e validação de pacotes/manifests (ADR-009, ADR-011) | Rust | Parsing de entrada não confiável |
| Tipos e validação de manifests (`framework/sdk/rust`) | Rust | Mesmas regras dos schemas em `framework/sdk/shared/` |
| Módulos de kernel (quando houver, classe 2 do ADR-009) | Rust, se `CONFIG_RUST` estiver ativo; senão C | Só depois de confirmar o kernel do device |
| UI do shell (Android Views, launcher) | Kotlin | Framework Android; fora do escopo de reescrita |
| SDK de apps Android | Kotlin (com API FFI para Rust quando fizer sentido) | ADR-004 |
| Cross-platform (apps próprios) | TypeScript | ADR-004 |

Primeiro corte entregue neste branch: `framework/sdk/rust` (workspace Cargo na raiz), com os tipos e as validações dos três manifests e 10 testes, todos passando em Rust 1.75.

---

## Consequências

### Positivas
- Parsing de manifests e pacotes em linguagem sem corrupção de memória, o que reduz o risco de SEC-001 e SEC-002.
- Um único conjunto de regras de validação para todos os serviços e para o driver manager.
- O workspace já compila e testa localmente.

### Negativas
- Duas linguagens nativas (Rust e Kotlin) e uma terceira de app (TypeScript). O custo de manutenção sobe.
- Binding Rust para Android (JNI/FFI) precisa de build próprio, com NDK e ferramentas de cross-compilation.
- Kernel em Rust depende de versão e de configuração; pode não estar disponível no Pixel 8a.

### Riscos
- **R-FFI:** a fronteira Kotlin/Rust é fonte de bug. Mitigação: API pequena, versionada, com testes de contrato.
- **R-TOOL:** toolchain Rust precisa ser pinada e auditada (A5). Mitigação: `rust-toolchain` fixado e `cargo-deny` no CI, a criar.

---

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Só Kotlin (ADR-004 como está) | Serviços de sistema sem garantia de memória; menos alinhado com o pedido |
| Só C/C++ | Mais risco de memória nos componentes que processam entrada não confiável |
| Reescrever o framework Android em Rust | Escopo impraticável; fora do produto |

---

## Plano

1. **A4:** manter `framework/sdk/rust` e adicionar a camada FFI para Kotlin, com um teste de contrato.
2. **A5:** fixar a toolchain (`rust-toolchain`), adicionar `cargo-deny` e revisar licenças das dependências (`serde`, `serde_json` são MIT/Apache-2.0).
3. **A2:** confirmar `CONFIG_RUST` no kernel do Pixel 8a (leitura de config, sem flash).
4. **A0:** aprovar este ADR e atualizar o ADR-004 com a referência.

---

## Referências

- ADR-004 (SDK próprio), ADR-009 (drivers), ADR-011 (matriz por marca)
- `framework/sdk/shared/` (schemas de referência)
- `framework/sdk/rust/` (implementação)
