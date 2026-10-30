# ORCHESTRATION_BACKLOG

Prioridade: P0 bug/prod · P1 bloqueios de entrega · P2 features · P3 deps/débito · P4 melhorias

| ID | Prio | Projeto | Origem | Descrição | Dep. | Status |
|---|---|---|---|---|---|---|
| T-001 | P0 | frota | Trello #1276 (Erros) | Bugs de produção achados pelo smoke e2e-fleet | — | A investigar |
| T-002 | P1 | condo_nexus | Trello 1641/1642/1644 (Deploy) | Portaria: guards de papel, colunas notifications, política de câmeras — validar implementação real antes de manter em Deploy | — | A verificar |
| T-003 | P1 | condo_nexus | Trello #1454 + issue #245 | Contratos GraphQL + divergência Terraform×cluster (apply = humano) | T-002 | Bloqueio parcial |
| T-004 | P1 | conexus-launcher | Trello 1638/1636/1603 (Erros) | CI github-release.yml, Data Safety, Modo Uma Mão | — | A investigar |
| T-005 | P1 | conexus-app | Trello 1470–1475 (Erros) | Staging, k6, backup, Grafana, LGPD, docs | — | A investigar |
| T-006 | P1 | ordempro/AS/renovafacil | Trello #1201 | P1 nested table wrap | — | A investigar |
| T-007 | P2 | conexus-app | Trello #1492 (Em Dev) | Testes unit/integration/E2E + CI | — | Em andamento |
| T-008 | P2 | medrate | Trello #1646 (Em Dev) | Migração Kotlin/Compose piloto | — | Em andamento |
| T-009 | P2 | agents | Trello #1270 (Em Dev) | Solid stack + agentes bff-* | — | Em andamento |
| T-010 | P2 | aucta_nexus | Trello #1321 (Em testes) | AUC-020 Listings | — | Validar testes |
| T-011 | P2 | condo_nexus | Trello 1450–1461 (Deploy) | 9 cards Expo/Tailwind/smoke: verificar se realmente entregues | — | A verificar |
| T-012 | P3 | frota (25+ repos) | 92 PRs Dependabot | Triar: todos BLOCKED; 1 conflito; ~60 com checks falhando. Sem merge sem checks verdes + revisão | — | Classificado |
| T-013 | P3 | gov_nexus | issues EP01–EP18 | Épicos P1; escopo enorme, não executável em 1 ciclo | — | Backlog |
| T-014 | P3 | fin-hub | issues #432–#475 | Épicos on-ramp/IR/copy trading | — | Backlog |
| T-015 | P4 | worktrees | git worktree list | 2 detached + 5 claude/* em bdfe8a4 (= main): candidatas a limpeza (exige confirmação) | — | Backlog |

## Bloqueios humanos (12 cards 👤 + Jira)
Jira (OAuth), terraform apply condo_nexus, MercadoPago notif. URL, NOWPayments, Play 1ª publicação (med_orbit, antispam), decisões de negócio Aucta (AUC-044/063), PIX/boleto E2E, vídeos guerra-tech, sysfincon Google login.
