# ADR-015: `driver-core` como base dos módulos por marca — **PROPOSTO**

**Status:** Proposto (decisão do Tiago em 2026-10-22: manter o `driver-core` e usá-lo para criar os módulos das marcas). Resolve o ponto aberto do ADR-009 sobre o destino do `driver-core/`. **Ajustado pelo ADR-016:** com Rust e Go como linguagens padrão, o `driver-core` em Java fica congelado (sem código novo) e serve de referência para reescrever cada HAL em Rust, marca a marca; os descritores `brands/` e a política de privilégio continuam valendo.
**Data:** 2026-10-22
**Decisores:** Tiago (decisão de manter), Tech Lead (A0), Security (A5)
**Assinatura:** pendente

---

## Contexto

O `driver-core/` (Java, ~13 mil linhas) traz HALs por marca (`vendors/google|nothing|oneplus|samsung|xiaomi`), abstrações por versão de kernel (4.19–6.6) e um `driver-manager`. Quando foi revisado:

- Tinha três caminhos de execução privilegiada sem controle: `executeCommand` (qualquer comando), `insmod`/`rmmod` com nome de módulo sem validação e sem assinatura (`Kernel61`), e hooks `sh` vindos do pacote (`DriverInstaller`). A extração do `.cdrv` ainda tinha zip-slip e nunca gravava os arquivos.
- **Não compila.** Não está no build raiz. Passa da fase de sintaxe do `javac` depois das correções deste PR, mas fica com ~2,5 mil erros semânticos: ~1,7 mil `cannot find symbol` (dependência do Android SDK, `android.hardware.camera2`, `android.media`, etc.), classes públicas em arquivo de outro nome (`BaseVendorClasses.java`), e tipos inexistentes.
- Moto G/Motorola e OPPO não têm HAL no `driver-core`.

## Decisão

1. **O `driver-core` fica** como a base de código dos HALs de userspace por marca (classe 1 do ADR-009).
2. **Privilégio fora do `driver-core`.** O `driver-core` não carrega módulo de kernel, não roda hook de pacote e não executa comando arbitrário. Carregar é do `conexus-driver-manager` (Rust), que verifica a assinatura sobre manifest, variante e payload (ADR-014). Já aplicado neste PR.
3. **Cada marca é um descritor de dados**, `brands/<marca>.json`, validado por `brand-module.schema.json` e pelo `conexus-sdk` (Rust), com testes de paridade. O descritor aponta para o `hal_source` no `driver-core` e guarda a evidência C1–C5 do ADR-010 por modelo.
4. **Nenhuma marca é `verified` sem evidência.** Hoje: `google`, `nothing`, `oneplus`, `samsung`, `xiaomi` (com `poco` e `redmi`) estão `unverified`; `oppo` e `motorola` estão `planned` (sem HAL). Nenhum modelo foi verificado.
5. **O `driver-core` precisa compilar antes de gerar qualquer bundle.** Um módulo por marca só entra no bundle depois de compilar com o Android SDK e passar nos testes dessa marca.

## Consequências

### Positivas
- Aproveita o que já existe, sem reescrever os HALs.
- O que roda com privilégio deixa de estar no `driver-core`.
- O estado de cada marca é auditável e testado (`cargo test` lê `brands/`).

### Negativas
- Fazer o `driver-core` compilar é trabalho grande, marca por marca. Não está feito.
- O kernel por versão (`Kernel419`…`Kernel66`) está duplicado em seis cópias; qualquer correção precisa ser aplicada em todas.

### Riscos
- **R-COMPILE:** o código pode não representar o hardware real (foi escrito sem compilar nem testar em aparelho). Mitigação: nenhum HAL é considerado funcional até compilar e passar em teste de aparelho com evidência C1–C5.
- **R-DUP:** seis cópias de kernel divergem. Mitigação: extrair o código comum para `kernel/common` no mesmo trabalho que fizer a árvore compilar.

## Plano

1. **A4:** criar o módulo Gradle de uma marca (começar pelo `google`, que tem o aparelho primário) com o Android SDK 36 e fazer só ele compilar, isolado do resto.
2. **A4:** repetir por marca, na ordem: `google`, `nothing`, `xiaomi` (com `poco`), `samsung`, `oneplus`; criar o HAL de `oppo` e de `motorola`.
3. **A2:** levantar C1–C5 por modelo a partir de fontes oficiais, e só então mudar o descritor para `verified`.
4. **A5:** implementar o verificador ed25519 e o domínio SELinux (pendentes do ADR-014).

## Referências

- ADR-009, ADR-010, ADR-011, ADR-014
- `brands/`, `framework/sdk/shared/brand-module.schema.json`, `framework/sdk/rust/src/brand.rs`
- `driver-core/kernel/common/.../KernelCommandPolicy.java`
