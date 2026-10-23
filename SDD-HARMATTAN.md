# SDD — Conexus Harmattan
## Especificação de arquitetura e implementação multiplataforma

**Projeto:** Conexus Launcher  
**Nome da plataforma:** Conexus OS  
**Referência histórica:** MeeGo 1.2 Harmattan / Nokia N9  
**Plataformas-alvo:** Android e Linux  
**Linguagem principal:** Rust  
**Kernel:** Linux existente, não reescrito  
**Status:** Especificação proposta — requer auditoria da base existente  
**Versão:** 0.1  
**Data:** 2026-10-22  

---

# 1. Visão do produto

Construir uma plataforma de interface e serviços Conexus inspirada no MeeGo Harmattan, com experiência de usuário própria, arquitetura modular e componentes de sistema escritos em Rust.

O produto deverá funcionar em dois ambientes distintos:

1. **Conexus para Linux:** aplicação de interface e serviços executados sobre uma distribuição Linux compatível.
2. **Conexus para Android:** aplicação instalável inicialmente como launcher, com integração limitada pelas APIs e permissões do Android. A substituição do sistema Android por uma imagem de sistema própria será uma etapa separada, dependente de hardware compatível.

A mesma base de código Rust deverá ser compartilhada sempre que tecnicamente viável, mantendo adaptadores específicos de cada plataforma.

## Objetivos

- Rebranding integral para Conexus.
- Interface própria inspirada na experiência do MeeGo Harmattan.
- Interface gráfica implementada em Rust.
- Serviços centrais implementados em Rust.
- Separação entre UI, core, adaptadores de plataforma e kernel.
- Compatibilidade com Linux desktop e Android.
- Testes automatizados e builds reproduzíveis.
- Ausência de dependências proprietárias obrigatórias da Nokia.
- Reutilização de projetos open source após avaliação de licenças e compatibilidade.

## Fora do escopo inicial

- Reescrever o kernel Linux em Rust.
- Garantir suporte universal a dispositivos Android ou Lumia.
- Presumir que aplicativos nativos antigos do Harmattan serão executados sem adaptações.
- Contornar o sandbox ou as restrições de segurança do Android.
- Fazer flash de dispositivos ou substituir sistemas instalados durante a fase inicial.

---

# 2. Repositórios de referência

O agente deverá inspecionar:

- https://github.com/orgs/harmattan/repositories
- https://github.com/harmattan
- https://github.com/harmattan/kernel-plus-harmattan
- https://github.com/ajalkane/profilematic
- https://github.com/rschiang/moedict-meego
- https://github.com/ajalkane/toggleavailability
- https://github.com/nemomobile/qt-components

Para cada projeto, registrar:

| Campo | Requisito |
|---|---|
| Licença | Identificar licença e obrigações |
| Tecnologia | Linguagem, framework e build |
| Estado | Manutenção, atividade e limitações |
| Compatibilidade | Toolchain e sistemas suportados |
| Reutilização | Direta, adaptada, referência ou descartada |
| Dependências | APIs proprietárias, bibliotecas e serviços |
| Migração | Estratégia de substituição ou integração |

### Diretriz

Os projetos históricos devem servir como referências de comportamento, UI e integração. Não serão automaticamente dependências do runtime final.

A biblioteca `qt-components` poderá ser usada como referência visual e funcional, mas a interface final será implementada com tecnologia Rust, salvo decisão arquitetural posterior justificada por uma limitação comprovada.

O kernel Harmattan será estudado separadamente. Seus patches não serão incorporados ao core Rust sem uma justificativa e uma avaliação específica de compatibilidade.

---

# 3. Arquitetura proposta

## 3.1 Diagrama lógico

    ┌─────────────────────────────────────────┐
    │          Conexus Experience             │
    │ Launcher · Home · Apps · Settings       │
    │ Multitarefa · Gestos · Notificações     │
    └────────────────────┬────────────────────┘
                         │
    ┌────────────────────▼────────────────────┐
    │            Conexus UI Rust              │
    │ Views · Navigation · State · Themes     │
    └────────────────────┬────────────────────┘
                         │
    ┌────────────────────▼────────────────────┐
    │           Conexus Core Rust             │
    │ Config · Events · Profiles · Apps       │
    │ Lifecycle · Permissions · Diagnostics   │
    └────────────────────┬────────────────────┘
                         │
    ┌────────────────────▼────────────────────┐
    │          Platform Abstraction           │
    │ Linux adapters        Android adapters   │
    └────────────────────┬────────────────────┘
                         │
    ┌────────────────────▼────────────────────┐
    │          Operating System               │
    │ Linux userspace / Android framework     │
    │ Linux kernel · drivers · hardware       │
    └─────────────────────────────────────────┘

O diagrama representa as dependências lógicas. Ele não significa que os dois sistemas operacionais exponham as mesmas APIs ou permitam as mesmas operações.

## 3.2 Camada de interface

Implementar em Rust:

- Tela inicial e grade de aplicativos.
- Navegação e histórico de telas.
- Gerenciamento do estado da interface.
- Temas, cores, tipografia e ícones.
- Transições e animações.
- Configurações próprias do Conexus.
- Interfaces de perfis e disponibilidade.
- Painéis e notificações dentro dos limites de cada plataforma.

A interface deverá preservar a linguagem visual do Harmattan sem copiar ativos protegidos indevidamente.

### Framework gráfico

Não fixar definitivamente o framework antes da prova de conceito. Avaliar pelo menos:

- Slint: UI declarativa e possibilidade de compartilhar a interface entre alvos.
- Iced: interface declarativa em Rust e arquitetura orientada a mensagens.
- egui: alternativa imediata para protótipos, ferramentas e interfaces de desenvolvimento.

A seleção deverá considerar suporte efetivo a Android e Linux, acessibilidade, desempenho, maturidade, licenciamento, integração com o sistema de janelas, gestos e facilidade de customização visual.

**Critério de decisão:** um protótipo da mesma tela deverá compilar e executar nos dois alvos antes da adoção definitiva.

## 3.3 Core Rust

O core conterá a lógica compartilhada, sem assumir acesso irrestrito ao sistema operacional.

Módulos propostos:

- `conexus-config`: configuração persistente e migrações.
- `conexus-events`: eventos e comunicação interna.
- `conexus-profiles`: perfis e regras de disponibilidade.
- `conexus-apps`: catálogo, metadados e ciclo de vida de aplicações.
- `conexus-permissions`: políticas e validação de operações.
- `conexus-diagnostics`: logs, métricas e diagnóstico.
- `conexus-platform`: interfaces abstratas e contratos.
- `conexus-ui`: interface gráfica.
- `conexus-launcher`: executável principal.
- `conexus-daemon`: serviço opcional de sistema no Linux.

Os módulos deverão usar APIs explícitas, tratamento consistente de erros e testes independentes.

Não é obrigatório que cada módulo se torne um processo separado. Separar processos exige justificativa de segurança, confiabilidade ou isolamento.

## 3.4 Adaptadores de plataforma

### Linux

Responsabilidades possíveis:

- Descoberta de aplicações e metadados.
- Integração com o ambiente gráfico e o gerenciador de janelas.
- Atalhos, notificações e ciclo de vida de aplicações.
- Persistência de configuração.
- Comunicação com serviços do usuário.
- Serviço de sistema opcional quando necessário.

O agente deverá identificar o ambiente gráfico e as APIs disponíveis. Não presumir que todos os sistemas Linux usam o mesmo protocolo de janelas ou mecanismo de notificações.

### Android

Responsabilidades possíveis:

- Registro como aplicativo de launcher.
- Consulta de aplicações disponíveis por APIs permitidas.
- Navegação e abertura de aplicações.
- Preferências e armazenamento da aplicação.
- Gestos e animações próprios do launcher.
- Integração permitida com notificações e atalhos.

O launcher Android não poderá controlar livremente outros processos, substituir serviços privilegiados nem implementar todas as funções de sistema do Harmattan por conta própria.

A integração com APIs Android deverá ocorrer por um adaptador mínimo e auditável. A interface e a lógica compartilhadas permanecerão em Rust sempre que possível.

Se a implementação Rust exigir integração JNI ou componentes Android específicos, eles deverão ser isolados no adaptador, não espalhados pela lógica central.

## 3.5 Kernel e drivers

O kernel Linux continuará sendo a camada de execução de baixo nível.

- Não converter o kernel inteiro para Rust.
- Não modificar drivers sem necessidade comprovada.
- Não pressupor compatibilidade entre o kernel Harmattan e kernels Android modernos.
- Documentar separadamente a arquitetura do processador, bootloader, drivers e requisitos de cada dispositivo.
- Manter qualquer trabalho de portabilidade de kernel em um projeto separado.

O Conexus Launcher poderá funcionar como aplicação sem exigir uma ROM própria. A construção de uma imagem de sistema será uma meta posterior, com requisitos independentes.

---

# 4. Contratos internos

Definir interfaces estáveis entre UI, core e adaptadores.

## 4.1 Contratos essenciais

1. **Configuração:** leitura, escrita, validação e migração de preferências.
2. **Eventos:** publicação e consumo de eventos com tipos explícitos.
3. **Aplicações:** listagem, metadados e solicitações de abertura.
4. **Perfis:** estado atual, regras e transições.
5. **Notificações:** abstração sobre as capacidades de cada plataforma.
6. **Diagnóstico:** logs estruturados, erros e versão dos componentes.
7. **Permissões:** validação antes de executar ações privilegiadas.
8. **Plataforma:** descoberta de capacidades disponíveis em runtime.

Os contratos deverão usar tipos explícitos e versões quando atravessarem processos ou fronteiras de plataforma.

Para IPC entre processos, avaliar D-Bus no Linux ou uma interface local equivalente. Não impor o mesmo mecanismo ao Android sem avaliar suas restrições.

## 4.2 Requisitos de segurança

- Não executar comandos de shell montados a partir de entradas do usuário.
- Validar todas as mensagens recebidas de processos externos.
- Restringir arquivos de configuração às permissões necessárias.
- Evitar privilégios de root no launcher.
- Não armazenar segredos em arquivos de configuração comuns.
- Registrar falhas sem expor dados sensíveis.
- Tratar falha de serviços como condição recuperável sempre que possível.

---

# 5. ADRs — Architecture Decision Records

## ADR-001 — Kernel Linux mantido

**Status:** Aceito.

**Decisão:** manter o kernel Linux existente e desenvolver o core Conexus em Rust no espaço de usuário.

**Motivo:** o objetivo é modernizar serviços e experiência do usuário sem assumir uma reescrita inviável do kernel.

**Consequência:** boot, drivers e compatibilidade de hardware continuam dependendo do sistema operacional e do dispositivo-alvo.

## ADR-002 — Core compartilhado em Rust

**Status:** Aceito.

**Decisão:** implementar a lógica de domínio e os serviços compartilháveis em Rust.

**Consequência:** módulos que dependem de APIs específicas terão adaptadores de plataforma. Não será exigida uma implementação idêntica das integrações em todos os sistemas.

## ADR-003 — Interface implementada em Rust

**Status:** Aceito; framework pendente de prova de conceito.

**Decisão:** reconstruir a interface com um framework gráfico compatível com Rust.

**Motivo:** reduzir dependência da stack histórica Qt/QML no produto final.

**Consequência:** os componentes Qt históricos serão referências de comportamento e aparência, não dependências obrigatórias do launcher final.

**Critério de encerramento:** framework selecionado por uma prova de conceito Android + Linux documentada.

## ADR-004 — Android e Linux como alvos

**Status:** Aceito.

**Decisão:** manter um núcleo compartilhado e adaptadores específicos.

**Consequência:** as funcionalidades serão classificadas por disponibilidade de API em cada sistema. A paridade será definida funcionalidade por funcionalidade.

## ADR-005 — Launcher primeiro; sistema operacional depois

**Status:** Aceito.

**Decisão:** iniciar com uma aplicação executável, sem exigir uma imagem de sistema personalizada.

**Motivo:** validar UI, arquitetura e core sem bloquear o desenvolvimento em boot e drivers.

**Consequência:** funcionalidades que exigem privilégios ou integração de sistema serão implementadas posteriormente, se tecnicamente viáveis.

## ADR-006 — Repositórios históricos como referência

**Status:** Aceito.

**Decisão:** auditar licenças e compatibilidade antes de reutilizar código.

**Consequência:** não incorporar automaticamente componentes históricos ao runtime. Preservar os avisos de licença e as atribuições obrigatórias.

## ADR-007 — Arquitetura modular antes de microserviços

**Status:** Aceito.

**Decisão:** iniciar como aplicação modular e usar processos separados somente quando houver uma justificativa concreta.

**Motivo:** reduzir complexidade de implantação, IPC e depuração na primeira versão.

## ADR-008 — Framework de UI selecionado por evidência

**Status:** Pendente.

**Decisão a tomar:** selecionar Slint, Iced, egui ou outra alternativa após construir uma tela equivalente nos dois alvos.

**Critérios:** build, rendering, gestos, acessibilidade, integração, manutenção, licença e desempenho medido.

---

# 6. Fases executáveis

## Fase 0 — Auditoria da base

Tarefas:

- Ler instruções do projeto e estrutura de diretórios.
- Identificar linguagem, framework, dependências e build atual.
- Localizar testes, assets, componentes e serviços existentes.
- Inspecionar os repositórios de referência.
- Gerar matriz de licenças, dependências e compatibilidade.
- Registrar dúvidas e riscos.

Entregas:

- `docs/audit.md`
- `docs/repository-matrix.md`
- `docs/architecture.md`
- `docs/decisions/ADR-*.md`

Critério de saída: arquitetura atual documentada e estratégia de migração fundamentada.

## Fase 1 — Prova de conceito da interface

Tarefas:

- Avaliar frameworks gráficos Rust.
- Implementar uma tela inicial com dados fictícios.
- Demonstrar navegação e um tema Conexus.
- Compilar e executar a mesma tela no Linux e Android.
- Documentar limitações e dependências de plataforma.

Critério de saída: uma prova de conceito reproduzível nos dois alvos, ou relatório explícito do bloqueio que impede a seleção.

## Fase 2 — Workspace Rust

Tarefas:

- Definir workspace Cargo.
- Configurar lint e formatação.
- Definir regras de dependências.
- Criar contratos e tipos comuns.
- Configurar testes e builds por plataforma.
- Documentar comandos de desenvolvimento.

Critério de saída: build limpo, testes iniciais passando e estrutura modular documentada.

## Fase 3 — Core mínimo

Tarefas:

- Implementar configuração persistente.
- Implementar eventos e tratamento de erros.
- Implementar diagnóstico.
- Definir a interface de descoberta de aplicações.
- Criar adaptadores de plataforma com capacidades explícitas.
- Testar o comportamento independente da UI.

Critério de saída: o core executa e é testável sem depender de um dispositivo específico.

## Fase 4 — Launcher funcional

Tarefas:

- Implementar home e grade de aplicações.
- Implementar navegação, estado e preferências.
- Implementar descoberta de aplicações por plataforma.
- Implementar abertura de aplicações através das APIs permitidas.
- Integrar gestos e animações.
- Aplicar o branding Conexus.

Critério de saída: launcher executável com funcionalidades básicas documentadas nos dois alvos.

## Fase 5 — Serviços e integrações

Tarefas:

- Implementar perfis e disponibilidade.
- Integrar notificações onde suportado.
- Implementar persistência e migrações.
- Isolar funções que exigem privilégios.
- Definir políticas de recuperação de falhas.
- Avaliar IPC e serviços de sistema no Linux.

Critério de saída: cada serviço possui testes, contratos e matriz de suporte por plataforma.

## Fase 6 — Migração de funcionalidades Harmattan

Tarefas:

- Mapear cada funcionalidade relevante do Harmattan.
- Comparar o comportamento original com o Conexus.
- Implementar equivalentes ou documentar limitações.
- Reutilizar componentes históricos somente após revisão legal e técnica.
- Remover dependências antigas gradualmente.

Critério de saída: matriz de funcionalidades com estados implementado, parcial, não suportado ou planejado.

## Fase 7 — Empacotamento e distribuição

Tarefas:

- Gerar pacote Linux compatível com o ambiente selecionado.
- Gerar artefato Android instalável.
- Documentar assinatura, permissões e instalação.
- Automatizar builds e testes.
- Registrar versões, hashes e dependências.
- Documentar desinstalação e recuperação.

Critério de saída: artefatos reproduzíveis instalados e validados em ambientes identificados.

## Fase 8 — Pesquisa de sistema próprio

Esta fase não bloqueia o launcher.

Tarefas:

- Selecionar dispositivos candidatos.
- Estudar bootloader, kernel, drivers e árvore de dispositivo.
- Identificar requisitos de imagem de sistema.
- Verificar compatibilidade de display, touchscreen, áudio, Wi-Fi e energia.
- Definir estratégia de recuperação.
- Elaborar uma prova de conceito isolada.

Critério de saída: relatório de viabilidade por dispositivo. Não declarar um sistema inicializável sem teste real ou emulação adequada.

---

# 7. Estratégia de testes

## Testes unitários

- Serialização e validação de configurações.
- Transições de estado.
- Regras de perfis.
- Validação de eventos.
- Tratamento de erros.
- Políticas de permissões.

## Testes de integração

- UI consumindo o core.
- Adaptadores simulados.
- Descoberta e abertura de aplicações.
- Persistência entre reinicializações.
- Falhas e recuperação de serviços.

## Testes multiplataforma

Para cada funcionalidade, registrar:

- Linux: implementado, parcial ou não suportado.
- Android: implementado, parcial ou não suportado.
- Dependências necessárias.
- Comando de teste.
- Resultado observado.
- Ambiente utilizado.

## Segurança

- Entradas inválidas.
- Tentativas de acesso a arquivos fora do escopo.
- Mensagens IPC malformadas.
- Falha de permissões.
- Dependências vulneráveis.
- Execução acidental com privilégios excessivos.

## Interface

- Resolução e escala.
- Navegação e gestos.
- Foco e acessibilidade.
- Estado vazio e erro.
- Rotação e redimensionamento quando suportados.
- Consistência visual da identidade Conexus.

Nenhum teste poderá ser marcado como aprovado sem execução e registro do resultado.

---

# 8. Critérios de aceitação do produto

A versão inicial será considerada pronta quando:

1. O código da interface e do core compartilhado estiver em Rust.
2. A prova de conceito de UI funcionar no Android e Linux selecionados.
3. O framework gráfico estiver documentado em um ADR.
4. O launcher apresentar identidade Conexus consistente.
5. A descoberta e abertura de aplicações funcionarem por APIs permitidas em cada alvo.
6. O core possuir testes unitários e de integração.
7. A configuração persistir corretamente entre reinicializações.
8. Erros e limitações de plataforma forem registrados.
9. Builds e instruções de instalação forem reproduzíveis.
10. Licenças e atribuições de código de terceiros estiverem documentadas.
11. O kernel permanecer fora do escopo de reescrita.
12. Não houver declarações de compatibilidade com hardware que não tenha sido validado.

A versão inicial não precisa reproduzir todos os recursos do Harmattan. A compatibilidade será ampliada por etapas, com critérios verificáveis.

---

# 9. Regras para o OpenCode

- Começar pela Fase 0.
- Não alterar arquivos antes de compreender a base atual.
- Não remover código legado durante a auditoria.
- Não inventar APIs, comandos, resultados ou compatibilidade.
- Não introduzir dependências sem justificar sua necessidade.
- Não migrar a UI para um framework antes da prova de conceito.
- Não assumir que o launcher Android possui privilégios de sistema.
- Não confundir aplicativo multiplataforma com sistema operacional próprio.
- Não alterar ou gravar imagens de dispositivos sem autorização explícita.
- Manter os ADRs e a documentação sincronizados com as decisões reais.
- Fazer mudanças pequenas, testáveis e reversíveis.
- Ao concluir cada fase, apresentar arquivos alterados, testes executados, resultados, limitações e próxima etapa.

## Primeira execução

Executar exclusivamente a Fase 0.

Entregar a auditoria, a matriz de repositórios, a arquitetura revisada e a lista de decisões pendentes. A Fase 1 só deverá começar depois que a estrutura real do Conexus Launcher for compreendida e os critérios da prova de conceito estiverem definidos.

**Estado atual:** especificação proposta. Nenhuma alteração no repositório, build, instalação ou teste é declarada como executada.