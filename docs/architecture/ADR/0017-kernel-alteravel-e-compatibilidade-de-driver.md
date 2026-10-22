# ADR-017: Kernel alterável, regra kernel↔driver e fontes externas — **PROPOSTO**

**Status:** Proposto (diretrizes do Tiago em 2026-10-22)
**Data:** 2026-10-22
**Decisores:** Tiago (diretrizes), Tech Lead (A0), Security (A5), Platform (A2)
**Assinatura:** pendente

---

## Contexto

O Tiago pediu três coisas: (1) aproveitar drivers de outros ROMs (XDA e similares) e portá-los; (2) a **única** regra é que o kernel do core bata com o kernel do driver; (3) o kernel do sistema deve ser **alterável**.

## O que as fontes confirmam

- **O kernel recusa módulo que não bate.** A causa típica de `insmod: Invalid module format` é o `vermagic` do módulo ser diferente do kernel em execução; o conserto normal é compilar o módulo contra os headers e a configuração exatos desse kernel. ([LKMPG, "módulos para um kernel pré-compilado"](https://ftp.kaist.ac.kr/tldp/LDP/lkmpg/2.6/html/x380.html); [relato de vermagic em NXP](https://community.nxp.com/t5/i-MX-Processors/Linux-kernel-vermagic-not-corresponding/m-p/242921/highlight/true)). Não achei fonte que explique a interação com `CONFIG_MODVERSIONS`; essa parte **não** foi verificada.
- **No Android GKI a estabilidade é limitada.** O KMI só vale dentro do mesmo par LTS+Android (`android14-6.1`, `android15-6.6`, `android16-6.12`); só os símbolos da lista contam; o kernel só pode usar `gki_defconfig`; e o toolchain do kernel e dos módulos de vendor precisa ser compatível. Módulo que exige símbolo fora do KMI não carrega. ([AOSP: Maintain a stable KMI](https://source.android.com/docs/core/architecture/kernel/stable-kmi)).
- **Como os ROMs do XDA costumam ser feitos:** device tree em `device/<marca>/<aparelho>`, kernel em `kernel/<marca>/<aparelho>`, blobs em `vendor/<marca>`; blobs vêm do TheMuppets ou são extraídos do aparelho, e são presos à versão do Android. ([exemplo de guia](https://ubports-docs-dev.readthedocs.io/en/latest/_sources/porting/build_and_boot/H7_setup_sources.rst.txt); [fio no XDA](https://forum.xda-developers.com/t/porting-lineageos-to-an-unsupported-device-with-no-similar-devices.3890429/post-87503309)). Não encontrei um guia único e atual; a prática varia por aparelho.

## Decisão proposta

### 1. A regra "o kernel tem de bater", em três camadas (já implementada no `conexus-core`)

Um módulo de kernel só serve se **todas** estas valem; qualquer falha recusa a instalação inteira do bundle:
1. **Variante certa:** `device` e `kernel_release` (`uname -r`) exatos.
2. **`vermagic` idêntico** ao do kernel em execução, `modinfo -F vermagic` do módulo contra o do kernel. Obrigatório em todo módulo de kernel.
3. **KMI igual**, se o módulo declara um (só kernel GKI), e **perfil de kernel igual**, se declara um. O KMI do aparelho é **derivado** do `uname -r` (`kmi_from_release`), nunca aceito como declarado pelo aparelho.

Kernel sem GKI (4.14, 4.19, 5.4…) não tem KMI: vale só o `vermagic` e o perfil. A assinatura cobre também `vermagic` e perfil.

### 2. Kernel alterável = perfil de kernel versionado

O kernel do sistema é descrito por um **perfil**: fonte (repositório e commit), defconfig e fragmentos, toolchain, e o resultado (`kernel_release`, `vermagic`, KMI se houver). O Conexus pode ter um perfil por aparelho e trocá-lo. Consequências:
- Trocar o perfil (ou até recompilar com outra configuração) muda o `vermagic`. Todo módulo de kernel antigo passa a "não servir". O `DriverManager::incompatible` lista esses módulos; eles são desativados e recompilados contra o novo perfil. Drivers de userspace não são afetados.
- A chave de assinatura de módulo é embutida **por perfil**, no build do kernel, para `CONFIG_MODULE_SIG_FORCE` (ADR-009).
- **Em kernel GKI, alterar é restrito:** sair do `gki_defconfig` ou trocar o toolchain quebra a garantia do KMI e pode impedir que módulos de terceiros carreguem. Kernel GKI alterado é tratado como kernel **sem GKI**: valem vermagic e perfil, e o KMI deixa de ser aceito como prova.
- Um kernel próprio muda a imagem de boot, então a verificação de boot (AVB) precisa ser refeita com chave do projeto, e isso depende de o bootloader permitir chave customizada. **Não verificado** em nenhum aparelho.

### 3. Fontes externas (XDA, outros ROMs): o que entra e o que não

| Fonte | Pode? | Condição |
|---|---|---|
| Device tree (Apache-2.0 na prática) | Sim | Referenciar por URL e commit; manter atribuição |
| Código de kernel e de módulo (GPL-2.0) | Sim | Compilar **da fonte**, publicar a fonte de tudo que for distribuído, GPL preservada |
| Scripts e patches de posts | Só com licença clara | Sem licença declarada, não copiar |
| **Blobs de vendor** | **Não redistribuir** | Extrair do aparelho de quem instala (`conexus-installer extract`); não usar espelhos como o TheMuppets |
| **ZIPs, kernels e imagens prontas de fórum** | **Nunca** | Binário sem fonte é risco de cadeia de suprimento; só se aceita o que se compila da fonte, com commit fixado |

Todo item aproveitado tem `fonte`, `commit` e `licença` registrados no repositório da marca (`SOURCES.md`). Nada vem de um anexo de fórum.

## Consequências

### Positivas
- A regra do Tiago é verificada por código e por teste, não por convenção.
- Trocar o kernel é seguro: nenhum módulo incompatível fica carregável sem aviso.
- Respeita GPL e a regra de não redistribuir blobs.

### Negativas
- Cada mudança de kernel exige recompilar e reassinar todos os módulos daquele perfil.
- Aproveitar drivers de outro ROM só funciona se o kernel dele puder ser reproduzido (fonte + defconfig + toolchain): vermagic igual exige a mesma configuração de build. Muitos ROMs não publicam isso.

### Riscos
- **R-VERMAGIC:** reproduzir o `vermagic` de um kernel de terceiros depende da configuração exata, incluindo opções como `SMP`, `preempt`, `mod_unload`, `modversions`. Mitigação: o módulo é compilado no **nosso** perfil de kernel; só o código-fonte do driver é aproveitado.
- **R-GKI:** alterar o kernel GKI desfaz a estabilidade do KMI. Mitigação: tratado como kernel sem GKI.
- **R-AVB:** kernel próprio exige assinar o boot; sem chave customizada no bootloader, o aparelho não inicia. Mitigação: verificar por aparelho (critério C1/C5).

## Plano

1. **Feito:** `vermagic`, `kernel_profile`, `kmi_from_release`, `check_kernel_match` e `DriverManager::incompatible` no `conexus-core`, com testes (host e armv7).
2. **A2:** definir o formato do perfil de kernel (`kernel-profile.json`) e gerá-lo no build do kernel.
3. **A4:** `conexus-driver-manager` chama `incompatible` ao iniciar e depois de cada OTA.
4. **A1/A2:** para cada aparelho candidato, registrar nos repositórios de marca de onde vem a fonte do driver (repositório, commit, licença), nunca de anexo de fórum.
5. **A5:** revisar AVB com chave própria por aparelho.

## Referências

- ADR-009, ADR-010, ADR-014, ADR-015
- `services/core/src/drivers.rs` (`check_kernel_match`, `kmi_from_release`, `incompatible`)
