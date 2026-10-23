# Maemo, MeeGo e Symbian — o que podemos aproveitar

**Data:** 2026-10-22 · **Status:** pesquisa (A1), sem código incorporado

## O que foi lido e o que não foi

**Lido de verdade** (clonado e examinado por arquivo, com commit fixado): a revisão do repo `conexus-launcher`, em `docs/SOURCE_REVIEW.md`, `docs/SYMBIAN_REVIEW.md`, `docs/LEGACY_ASSETS.md` (08/10/2026):

| Fonte | Arquivos lidos | Licença observada |
|---|---|---|
| Hildon Desktop (Maemo Leste) | `hd-task-navigator.c`, `hd-note.c`, `hd-home-view-container.c`, `hd-launcher-tree.c` | cabeçalhos LGPL 2.1; o repositório declara GPL-2.0 no GitHub |
| libmeegotouch (arquivo Nemo) | `mswiperecognizer.cpp`, `mnotificationgroup.cpp` | LGPL 2.1 nos cabeçalhos |
| Lipstick (Sailfish) | `notificationpreviewpresenter.cpp`, `notificationmanager.cpp`, `notificationlistmodel.cpp`, `launchermodel.cpp` | LGPL 2.1 nos cabeçalhos; o GitHub não mostra licença do repo |
| Symbian homescreen (`oss.FCL.sf.app.homescreen`) | `hsmenuservice.cpp`, `hsmenuserviceutils.cpp`, `hsaddappstocollectionstate.cpp`, `hsmenuitemmodel.cpp`, `hswallpaperloaderthread.cpp`, `hswallpaperselectionstate.cpp`, `installs_symbian.pri` | EPL 1.0 nos cabeçalhos Nokia |
| Temas livres | MeeGoTouch/Nemo (LGPL 2.1), Hildon Beta (GPL-3.0), wallpapers Symbian (EPL 1.0) | já importados no launcher com avisos e hashes |

**Confirmado agora no GitHub (2026-10-22):** `SymbianSource` tem 216 repositórios públicos; `maemo-leste` 270; `nemomobile` 170 e `nemomobile-graveyard` 33; `sailfishos` 965; `mer-hybris` 244. `hildon-desktop` está ativo (último push 2025-11-15), `libmeegotouch` está parado desde 2013 no arquivo Nemo.

**Não lido, e por isso não afirmado:** nenhuma imagem de firmware N9/N900 foi examinada; o kernel Symbian (EKA2) e o código do MeeGo original da Nokia (Harmattan) não foram abertos nesta pesquisa. As ideias de arquitetura marcadas como *(conhecimento prévio)* abaixo vêm do que se sabe publicamente sobre esses sistemas e precisam ser conferidas no código antes de virar requisito.

## Regra de reaproveitamento

O Conexus Mobile OS é Apache-2.0. Código GPL ou LGPL não entra na árvore Apache, e código EPL só em arquivos separados.

| O que | Pode? | Como |
|---|---|---|
| **Comportamento e ideias** (gestos, fluxo de notificação, organização) | Sim | Reimplementação original, documentada em `docs/re/`, sem copiar código (clean room, `LEGAL_BOUNDARIES.md`) |
| **Código GPL/LGPL/EPL** | Não na árvore principal | Só em módulos separados e opcionais, se um dia for necessário, com a licença própria e o aviso preservados. Nenhum hoje |
| **Assets livres** (ícones, wallpapers dos três pacotes) | Sim | Já no launcher, com `NOTICE`, licença e hash. Não são relicenciáveis como Apache |
| **Marcas** (Nokia, Ovi, Symbian, MeeGo, Maemo, Lumia) | Não | Nome e logo não entram no produto |

## O que aproveitar, por plataforma

### Maemo (Hildon)
- **Navegador de tarefas com notificação associada ao app** (`hd-task-navigator`): reaproveitar o *conceito* de cartão de app com contador e prévia, no Action Center e no multitarefa do Conexus Launcher.
- **Banner de notificação abaixo da barra de título**, com resumo, horário e destino (`hd-note`): comportamento para o banner interno do launcher; não substitui o heads-up do Android.
- **Catálogo de apps pela árvore de `applications.menu`** (`hd-launcher-tree`): mapeia para um catálogo único via PackageManager, com categorias editáveis.
- **Maemo Leste como base Linux** para o alvo N900 (ADR-013): é Debian/Devuan com kernel e pacotes comunitários. Serve de userspace e de referência de boot; o código continua com as licenças deles.

### MeeGo (Harmattan, MeeGo Touch, Mer/Nemo)
- **Reconhecedor de swipe** (`mswiperecognizer`): considera distância, tempo, ângulo e jitter. Aproveitar os *parâmetros e o desenho*, reimplementados no Gesture Engine do launcher (já existe a classe de gestos testável).
- **Grupos de notificação com resumo, categoria, contador e ID** (`mnotificationgroup`): modelo para o `NotificationBroker` do core Rust (`services/core`), com atualização pelo mesmo ID.
- **Três painéis (Eventos, Apps, Multitarefa):** já é a base do launcher. Manter.
- **Mer e Nemo como userspace Linux enxuto** (alvo N9, ADR-013). Status do bootloader do N9 continua sem confirmação.

### Symbian
- **Coleções de apps com seleção múltipla e confirmação** (`hsaddappstocollectionstate`): máquina de estados para pastas e coleções.
- **Ordens de lista explícitas** (nome, mais recente, mais antigo, personalizada): oferecer as mesmas, mais frequência como extensão nossa (não é algoritmo Symbian).
- **Carga eficiente de ícones e wallpaper** (`hsmenuitemmodel`, `hswallpaperloaderthread`): cache limitado, decodificação no tamanho alvo, preload só da próxima tela.
- **Modelo de segurança por capacidades e pacote assinado** *(conhecimento prévio)*: o Symbian atribuía capacidades a executáveis e exigia assinatura. Isso é parente direto do nosso pacote de driver assinado (ADR-014) e do modelo de permissões do SDK. Vale estudar o desenho; não copiar nomes.
- **Separação entre driver lógico e driver físico (LDD/PDD) no kernel EKA2** *(conhecimento prévio)*: corresponde bem às camadas do `driver-core` (HAL genérico acima, implementação por marca abaixo, ADR-015). Útil como argumento de arquitetura, não como código.

### Lipstick / Sailfish (descendente do MeeGo)
- **Fila de previews de notificação, atualização por `replacesId`, filtros por privacidade e estado da tela, persistência com expiração:** o melhor modelo disponível para o `NotificationBroker`. Hoje o broker do core só guarda em memória e tem limite de 100 ativas; persistência e expiração são o passo seguinte.
- **Separação home / notificações / compositor / bloqueio** em componentes distintos: reforça a modularidade (ADR-016).

## O que não dá para aproveitar

- **Compositor e shell nativo** (Hildon, Lipstick, Hb): dependem de X11/Wayland e Qt/GTK. No Android, o Conexus Launcher não substitui o SystemUI; isso fica para um módulo próprio de SystemUI, em outra fase.
- **Miniaturas reais de todas as tarefas, fechar processos de terceiros, instalar apps em silêncio:** o Android não permite a um launcher comum.
- **Kernel e drivers** dessas plataformas: não se aplicam ao Android; só ao alvo Linux ARMv7 (ADR-013).
- **Aparência idêntica:** não é objetivo nem permitido (marcas e assets proprietários).

## Próximos passos

1. **A1:** transformar cada item acima em linha da `BEHAVIOR_MATRIX.md` com critério de equivalência testável.
2. **A4:** modelar persistência e expiração de notificações no `NotificationBroker` (Rust), com base no que a Lipstick faz, sem copiar código.
3. **A5:** ler o modelo de capacidades do Symbian (fonte pública da Symbian Foundation) e compará-lo com as permissões do SDK e com a política de pacote assinado.
4. **A1:** conferir no código as duas afirmações marcadas como *(conhecimento prévio)* antes de usá-las em ADR.
