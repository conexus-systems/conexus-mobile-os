# ADR-016: Plataforma compatível com Android, modular, Conexus Launcher padrão, sem GApps, Rust e Go — **PROPOSTO**

**Status:** Proposto (diretrizes do Tiago em 2026-10-22)
**Data:** 2026-10-22
**Decisores:** Tiago (diretrizes), Tech Lead (A0), Security (A5)
**Assinatura:** pendente

---

## Contexto

O Tiago definiu cinco diretrizes: (1) manter a compatibilidade com o Android; (2) o sistema é modular; (3) o launcher padrão é o Conexus Launcher; (4) a Play Store não vem instalada, e quem quiser instala GApps; (5) Rust e Go são as linguagens padrão. Elas organizam decisões anteriores (ADR-001, 004, 007, 009, 012, 014, 015) e mudam a política de linguagem.

## Decisão

### 1. Compatibilidade com Android
A base continua LineageOS/AOSP (ADR-001). Aplicativos Android rodam sem modificação, e as APIs públicas do framework não são removidas nem alteradas. Não se declara certificação Android nem aprovação no CTS: executar Android não é ser compatível no sentido oficial (SDD). **Critério de aceite:** um conjunto de apps de teste instala, abre e funciona com os módulos Conexus ligados e também com todos desligados.

### 2. Sistema modular
Cada recurso do Conexus é um módulo que pode ser incluído, removido e atualizado separadamente, com interface versionada entre eles (AIDL no Android, D-Bus no Linux, ADR-007):

| Módulo | Função | Remover? |
|---|---|---|
| `conexus-launcher` | Início, tiles, gaveta, Action Center | Sim, volta o launcher padrão do LineageOS |
| `conexus-tile-engine`, `conexus-notifications`, `conexus-app-manager` | Serviços do core (Rust) | Sim, o launcher perde os recursos que dependem deles |
| `conexus-keyboard` | Teclado | Sim |
| `conexus-driver-manager` | Instala pacotes de driver assinados (ADR-014) | Sim, sem drivers extras |
| `conexus-update` | OTA próprio | Sim, usa o updater do LineageOS |
| Módulos por marca (`brands/`, ADR-015) | HALs por marca | Só o da marca do aparelho entra |

O sistema-base precisa **inicializar e funcionar sem nenhum módulo Conexus**. Isso é teste de aceite (SYS-001).

### 3. Launcher padrão
O Conexus Launcher (`conexus-systems/conexus-launcher`, hoje na 0.32.0) é o launcher padrão da imagem e o papel HOME é atribuído a ele na construção da imagem. Continua um módulo separado, com repositório e ciclo de versões próprios (ADR-002 do export), e o usuário pode trocar de launcher como em qualquer Android. Como atribuir o papel HOME no build do LineageOS (overlay de recursos do framework) precisa ser confirmado no código-fonte da versão fixada; não está verificado.

### 4. Play Store e GApps
A imagem **não inclui** Play Store, Google Play Services nem apps Google. O LineageOS já se comporta assim. Regras:
- O Conexus não redistribui GApps (são proprietários). O instalador explica a opção e deixa o usuário fornecer o pacote de GApps de sua escolha, compatível com a versão do Android da imagem.
- Alternativa livre opcional: microG, como módulo à parte. Exige suporte a *signature spoofing* na imagem, decisão de segurança que fica para um ADR próprio.
- Nenhum módulo Conexus depende de serviços Google. O catálogo de apps Conexus funciona sem Play Store.

### 5. Linguagens padrão: Rust e Go
| Camada | Linguagem | Observação |
|---|---|---|
| Serviços de sistema, driver manager, validação de pacotes e manifests, HALs novos | **Rust** | Suportado no build do AOSP (Soong); backend AIDL em Rust existe. Verificar na versão fixada |
| Ferramentas de host: instalador, servidor OTA, pipeline de release, ferramentas do lab de dispositivos | **Go** | Binário único estático, multiplataforma (Linux, macOS, Windows). Substitui o instalador em Python |
| Daemons nativos no aparelho sem dependência de API Android | Go (estático, `CGO_ENABLED=0`) | Só se for realmente necessário. Não faz parte da árvore de build da plataforma |
| UI Android (Conexus Launcher, teclado) | **Kotlin** (exceção) | O SDK Android é JVM. Regra de negócio testável vai para o núcleo em Rust via JNI, como já feito no teclado (PR #43 do launcher) |
| Apps cross-platform do SDK | TypeScript | ADR-004 mantido para apps; não é linguagem de sistema |
| `driver-core` em Java | **Congelado** | Nenhum código novo em Java. Cada HAL é reescrito em Rust, marca a marca (ADR-015, ajustado abaixo) |
| Scripts | Shell mínimo | Sem lógica de negócio |

## Consequências

### Positivas
- Menos superfície de ataque em código com privilégio (Rust).
- Ferramentas de usuário em um único binário, sem Python nem dependências.
- A modularidade é testável: ligar e desligar módulos sem quebrar o sistema-base.

### Negativas
- Três linguagens ativas (Rust, Go, Kotlin) e uma congelada (Java).
- O instalador em Python precisa ser portado para Go. A parte de segurança (hash, extração segura, frase de confirmação) já tem testes e serve de contrato.
- Reescrever os HALs do `driver-core` em Rust é trabalho maior do que fazer o Java compilar. **Isto ajusta o ADR-015:** o `driver-core` Java passa a ser referência de leitura e não a base de código final.

### Riscos
- **R-RUST-AOSP:** o suporte a Rust na plataforma depende da versão do AOSP/LineageOS. Mitigação: confirmar na versão fixada antes de escrever o primeiro serviço de plataforma.
- **R-GAPPS:** apps que exigem Google Play Services não funcionam sem GApps ou microG. Mitigação: documentar e deixar o instalador explicar.
- **R-HOME:** a atribuição do papel HOME no build não está confirmada. Mitigação: verificar no código da versão fixada.

## Plano

1. **A4:** portar a camada de segurança do instalador para Go (`tools/installer`), com os mesmos casos de teste do Python. Feito neste PR: pacote `safety` e testes.
2. **A4:** portar o restante do instalador (detecção, download, fastboot) para Go e aposentar o Python.
3. **A2:** confirmar suporte a Rust no build da versão fixada do LineageOS e o mecanismo para definir o HOME padrão.
4. **A4:** escrever o primeiro HAL de uma marca em Rust (Google), usando o `driver-core` Java como referência.
5. **A5:** ADR de microG e *signature spoofing*, se for adotado.
6. **A6:** teste de aceite "apps Android com módulos ligados e desligados".

## Referências

- ADR-001, 002, 004, 007, 009, 012, 014, 015
- `docs/re/NOKIA_PLATFORMS_REUSE.md`
- SDD: "executar Android não equivale a possuir certificação Android"
