# ADR-014: APK por marca como contêiner de módulos de kernel — **PROPOSTO**

**Status:** Proposto (substitui em parte o ADR-011; complementa o ADR-009)
**Data:** 2026-10-21
**Decisores:** Tech Lead (A0), Security (A5), Platform (A2)
**Assinatura:** pendente

---

## Contexto

O Tiago pediu um core Android único com um APK por marca que instale módulos de kernel. O ADR-011 recusou "um APK por marca com drivers" por dois motivos: privilégio de kernel sem assinatura do projeto e distribuição de blobs proprietários. Revendo, o formato APK em si não é o problema. O problema é **quem carrega** o módulo e **que licença** ele tem. Se o APK for só um contêiner e o carregamento ficar com um serviço de sistema, as duas objeções se resolvem.

Fatos que sustentam a revisão:

- Módulos compilados da fonte GPL publicada pelo fabricante podem ser redistribuídos com a fonte. Isso difere de blobs proprietários de userspace.
- Nós construímos a imagem (LineageOS), então controlamos o kernel, a chave embutida nele e a política SELinux. Não há certificação Android a preservar.
- Módulos de kernel são compilados por kernel e por KMI, não por marca. Um APK por marca precisa de várias variantes dentro dele.

---

## Decisão proposta

**O APK é um contêiner assinado. O serviço de sistema `conexus-driver-manager` verifica e carrega. O APK nunca recebe privilégio de kernel.**

1. **Core Android único:** mesmos serviços e mesma ABI de driver em todos os devices (ADR-009).
2. **Um APK por marca (bundle):** contém várias **variantes**, cada uma compilada para um `device` e um `kernel_release` exatos, com o KMI declarado.
3. **Seleção:** o driver manager seleciona **todas** as variantes que casam com o `device` e o `kernel_release` do aparelho (um driver por componente, ADR-009) e as instala de forma atômica: se qualquer uma falhar, nenhuma é instalada. Nenhuma variante que casa, ou duas variantes com o mesmo nome, é recusado. Se o KMI de um módulo difere do kernel em execução, é recusado.
4. **Verificação em duas camadas:**
   - Assinatura do pacote pelo driver manager (chave de driver do projeto, ADR-009). A mensagem assinada cobre manifest, `device`, `kernel_release` e payload (`signing_message`, encoding canônico com tamanhos); trocar qualquer campo invalida a assinatura. **Estado:** a mensagem e a recusa existem e estão testadas; o verificador criptográfico (ed25519) e a custódia da chave **ainda não existem** (A5).
   - `CONFIG_MODULE_SIG_FORCE=y` no kernel: o próprio kernel recusa módulo sem assinatura válida. A chave pública de driver é embutida na construção do kernel.
5. **Carregamento:** só por um serviço privilegiado do sistema (domínio SELinux próprio, a criar). Módulos instalados pelo usuário ficam em `/data`; `vendor_dlkm` é somente leitura e não é alterado.
6. **Licença:** só entram módulos compilados da fonte GPL publicada. Módulos binários sem fonte passam por revisão do A5 e, se não puderem ser redistribuídos, ficam fora do bundle (o usuário extrai do próprio aparelho, fora do repo).
7. **Cobertura:** o bundle só inclui variantes de modelos que passaram nos critérios de entrada do ADR-010 (C1–C5). Nenhum bundle declara suporte a "todos" os modelos de uma marca.

---

## Consequências

### Positivas
- Atende ao pedido: instalar drivers por um APK, por marca.
- A confiança está na assinatura e no kernel, não no APK.
- A seleção, a instalação atômica e a recusa por manifest/variante adulterados existem e estão testadas no `conexus-core` (`DriverBundle`, `install_bundle`, `signing_message`). Falta a verificação criptográfica real.

### Negativas
- Cada atualização de kernel exige recompilar e reassinar as variantes.
- Um bundle por marca tende a ficar grande (uma variante por modelo e por kernel).
- Exige construir o kernel com a chave de driver embutida, o que prende o projeto à própria cadeia de build.

### Riscos
- **R-KEY:** chave de driver vazada permite módulo malicioso assinado. Mitigação: chave offline/HSM, rotação, revogação pelo `conexus-update`.
- **R-SELINUX:** política do domínio de carregamento mal escrita abre escalada de privilégio. Mitigação: revisão do A5 antes de qualquer imagem `alpha`.
- **R-LIC:** módulo binário sem fonte no bundle. Mitigação: checagem de licença por variante antes de incluir.
- **R-MODULE-BUG:** módulo de terceiros derruba o sistema. Mitigação: carregamento em teste e rollback automático (ADR-008).

---

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| APK que carrega o módulo por conta própria (root) | Dá privilégio de kernel a um app; anula a assinatura |
| Sem bundle, um APK por modelo | Explosão de pacotes e de ciclos de atualização |
| Selecionar uma única variante por (device, kernel) | Impede um driver por componente; trocado por seleção de todas as variantes do aparelho, com instalação atômica |
| Só userspace (ADR-009 classe 1) | Não cobre hardware sem interface de userspace |

---

## `driver-core/` Java

Resolvido pelo ADR-015: o `driver-core` fica como base dos HALs de userspace por marca, sem nenhuma execução privilegiada. O `insmod` direto de `Kernel61.java` foi removido; carregar módulo é só do `conexus-driver-manager`. O `driver-core` ainda não compila, então nenhum código dele entra num APK de marca antes de compilar e passar nos testes.

## Plano

1. **A5:** aprovar o domínio SELinux do driver manager e a cadeia de chaves de driver.
2. **A2:** confirmar `CONFIG_MODULE_SIG_FORCE` e a chave embutida no kernel do Pixel 8a (leitura de config, sem flash).
3. **A4:** definir o formato do bundle dentro do APK (`driver-manifest.schema.json` e a tabela de variantes).
4. **A2:** montar a primeira variante verificada, para um device do ADR-010, com a fonte GPL do fabricante.

---

## Referências

- ADR-009 (kernel e drivers), ADR-010 (critérios de device), ADR-011 (marcas)
- `services/core/src/drivers.rs` (`DriverBundle`, `install_bundle`)
