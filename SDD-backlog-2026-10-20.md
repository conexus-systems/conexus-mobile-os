# SDD — backlog aberto do board Conexus-Systems (2026-10-20)

Fonte: export fresco do board (447 cards; 243 abertos fora das listas ✅ Concluído). Classificação automática por palavras-chave + revisão manual dos casos de Aucta/Fábrica. Não substitui o PO: card sem Gherkin volta ao PO antes de desenvolvimento.

## Resumo

- Abertos: 243
- Candidatos a desenvolvimento (Gherkin presente, sem clique humano): 44
- Bloqueados por humano (WAIT_HUMAN / decisão / painel / secret): 87
- Sem Gherkin (voltam ao PO): 106
- Possivelmente concluídos, verificar: 6

## Ondas propostas

| Onda | Unidade | Repo | Dono (agente) | Estado |
|---|---|---|---|---|
| 1 | customer_admin bloqueado (kGhklMlH) | aucta_nexus | backend-rails | Código pronto em `fix/auc-customer-admin-blocked` (527e998); bloqueado por migration duplicada (card QShkFAkw) |
| 1 | Escopar canais realtime por tenant (yIAovZRe) | aucta_nexus | backend-rails | Gherkin a validar; risco médio |
| 2 | bff-admin build TypeScript | bff-admin | frontend-inertia | Feito em `86bfb64`; falta PR |
| — | Épicos AUC-010…121 (tenancy, listings, MP, OLX, financeiro) | aucta_nexus | architect/backend-rails | Épicos de meses: precisam de ADR (AUC-010, AUC-040) antes de código |
| — | Cards 👤 Humano | — | Tiago | Não desenvolvíveis pelo agente |

## Candidatos a desenvolvimento (44)

| Lista | Card | Link |
|---|---|---|
| Aucta — Backlog | Aucta — [AUC-010] ADR: estratégia de tenancy (schema-per-tenant vs coluna tenant_id) | https://trello.com/c/GBc88H3V |
| Aucta — Backlog | Aucta — [AUC-011] Implementar isolamento de dados por tenant nas tabelas de domínio | https://trello.com/c/RC3jN541 |
| Aucta — Backlog | Aucta — [AUC-012] Resolução de tenant por host + Current.tenant em requests e jobs | https://trello.com/c/dMwAGEfr |
| Aucta — Backlog | Aucta — [AUC-013] ACL e Cognito conscientes de tenant (grupos por tenant) | https://trello.com/c/ceYnRUKu |
| Aucta — Backlog | Aucta — [AUC-014] Suíte de vazamento entre tenants (RSpec/Minitest + Playwright com subdom | https://trello.com/c/IpzzBucP |
| Aucta — Backlog | Aucta — [AUC-015] Provisionamento de tenant (criar tenant, domínio, seeds idempotentes) | https://trello.com/c/w89YZtZn |
| Aucta — Backlog | Aucta — [AUC-020] Tabela e model de Listings (anúncios) ligados a lote/leilão e canal | https://trello.com/c/kCzHOWum |
| Aucta — Backlog | Aucta — [AUC-021] Serviço de ciclo de vida do anúncio (draft → published → sold/expired/re | https://trello.com/c/1qCn76M0 |
| Aucta — Backlog | Aucta — [AUC-022] Vitrine lê de Listings (hoje serializa Auction) com paginação e filtros | https://trello.com/c/OlvKig5Z |
| Aucta — Backlog | Aucta — [AUC-023] Admin: criar, publicar e despublicar anúncios por canal | https://trello.com/c/FD2lsXN0 |
| Aucta — Backlog | Aucta — [AUC-030] Spike: contrato real da API OLX (auth, categorias, limites) com evidênci | https://trello.com/c/OfD6NTsT |
| Aucta — Backlog | Aucta — [AUC-031] Credenciais OLX por tenant (substituir token global em ENV) | https://trello.com/c/DuM5oTFj |
| Aucta — Backlog | Aucta — [AUC-032] Publicação disparada pelo Listing, com defaults de categoria/CEP/telefon | https://trello.com/c/QNEF61Sd |
| Aucta — Backlog | Aucta — [AUC-033] Sincronizar ciclo de vida OLX (remover ao vender, reconciliar status) | https://trello.com/c/PAZd5t4i |
| Aucta — Backlog | Aucta — [AUC-034] Testes da integração OLX com HTTP stubado + cenários Gherkin | https://trello.com/c/AMds7EjB |
| Aucta — Backlog | Aucta — [AUC-040] ADR: Mercado Pago direto no app vs via bff-payment | https://trello.com/c/Um854CY4 |
| Aucta — Backlog | Aucta — [AUC-041] Criar pagamento/checkout do arrematante para lote arrematado | https://trello.com/c/xTEO7bPQ |
| Aucta — Backlog | Aucta — [AUC-042] Mapear status do Mercado Pago para Payment e liquidação do arrematante | https://trello.com/c/fne3lRUX |
| Aucta — Backlog | Aucta — [AUC-043] Idempotência e reconciliação de webhooks do Mercado Pago | https://trello.com/c/N7bvSz3u |
| Aucta — Backlog | Aucta — [AUC-044] Credenciais e split por tenant (marketplace) — requer decisão de negócio | https://trello.com/c/QZCrUrTl |
| Aucta — Backlog | Aucta — [AUC-050] Status de montagem do leilão com bloqueio de edição por status | https://trello.com/c/LDVphPCw |
| Aucta — Backlog | Aucta — [AUC-051] Métrica de fechamento configurável e fechamento de lote com paridade | https://trello.com/c/ItrYKPp2 |
| Aucta — Backlog | Aucta — [AUC-052] Lance automático (proxy) e lances sugeridos | https://trello.com/c/WEpNG0I1 |
| Aucta — Backlog | Aucta — [AUC-053] Rollback de lance quando espelho Firebase falha + lance admin por client | https://trello.com/c/jbvkjERn |
| Aucta — Backlog | Aucta — [AUC-054] Parâmetros do leilão (tempos de pregão, dou-lhe, fuso, transmissão, limi | https://trello.com/c/va8K3mes |
| Aucta — Backlog | Aucta — [AUC-055] Tipos de leilão ampliados (presencial, interno, inverso, venda direta, c | https://trello.com/c/sFPM0pCA |
| Aucta — Backlog | Aucta — [AUC-060] Cobrança do arrematante: comissão % + extras, multa de cancelamento de 2 | https://trello.com/c/ySeGFDfz |
| Aucta — Backlog | Aucta — [AUC-061] Boleto registrado (Bradesco/BB/Itaú) — confirmar paridade e certificados | https://trello.com/c/rSdrED4T |
| Aucta — Backlog | Aucta — [AUC-062] Prestação de contas ao comitente (custos, status aprovada/reprovada) | https://trello.com/c/sRuCVdr5 |
| Aucta — Backlog | Aucta — [AUC-063] Valores de comissões/estadia/remoção do contrato do comitente (preciso d | https://trello.com/c/1gI6QSqv |
| Aucta — Backlog | Aucta — [AUC-070] Envio real de e-mail (SES) e os 11 templates transacionais | https://trello.com/c/JZIgpo2A |
| Aucta — Backlog | Aucta — [AUC-071] Notificações via bff-api (e-mail/SMS/Discord) em vez de SES direto | https://trello.com/c/kNUvf4Jc |
| Aucta — Backlog | Aucta — [AUC-080] Admin Hotwire: fechar telas parciais (dashboard, vendedores, relatórios, | https://trello.com/c/8irrasxH |
| Aucta — Backlog | Aucta — [AUC-081] UI pública: login/cadastro e área do arrematante (origem/frontend só tem | https://trello.com/c/e2K2GhAP |
| Aucta — Backlog | Aucta — [AUC-090] Cutover de dados MySQL → Postgres com validação de contagem | https://trello.com/c/yJtBh7KC |
| Aucta — Backlog | Aucta — [AUC-091] Testes de paridade legado × novo (API/regras de lance e fechamento) | https://trello.com/c/dvque0w3 |
| Aucta — Backlog | Aucta — [AUC-092] Atualizar docs desatualizados (Rails 8.1 + Solid Queue, não Sidekiq) | https://trello.com/c/RRjmUhkS |
| Aucta — Backlog | Aucta — [AUC-101] Observabilidade: Sentry (Rails+React) e traços LangSmith do pipeline de  | https://trello.com/c/JEXNe0fI |
| Aucta — Backlog | Aucta — [AUC-110] Épico futuro: site de busca de anúncios/leilões multi-tenant | https://trello.com/c/L3Sa7UNl |
| Aucta — Backlog | Aucta — [AUC-121] Pack apps/aucta_nexus (skills próprias, contexto, stack real) sem reusar | https://trello.com/c/u2Rmv9ND |
| 📥 Backlog | Aucta — Cognito: reset/refresh/activation para admin provisionado (Customer#cognito_userna | https://trello.com/c/fQqkdd46 |
| 📥 Backlog | Aucta — Escopar canais realtime (Turbo lot_<id>, Firebase RTDB) por tenant | https://trello.com/c/yIAovZRe |
| 📥 Backlog | Aucta — authenticate_customer_admin! deve negar usuário blocked | https://trello.com/c/kGhklMlH |
| 📥 Backlog | Fábrica — adapter Hermes/OpenCode na main (conexus_trello.py, install --only) | https://trello.com/c/sF9aFeQi |

## Sem Gherkin (PO primeiro) (106)

| Lista | Card | Link |
|---|---|---|
| Aucta — Backlog | Aucta — E-Bidding | https://trello.com/c/FXM3Wvex |
| Aucta — Backlog | Aucta — E-Catalog | https://trello.com/c/H36oGddW |
| Aucta — Backlog | Aucta — E-Customers-KYC | https://trello.com/c/Y5lB2vub |
| Aucta — Backlog | Aucta — E-DevOps-Linode | https://trello.com/c/xwdQshI9 |
| Aucta — Backlog | Aucta — E-Identity | https://trello.com/c/96qD4U1f |
| Aucta — Backlog | Aucta — E-Integrations-MP | https://trello.com/c/TvOqtEgw |
| Aucta — Backlog | Aucta — E-Integrations-OLX | https://trello.com/c/ypyJazhQ |
| Aucta — Backlog | Aucta — E-QA-E2E | https://trello.com/c/90yZ8Cp0 |
| Aucta — Backlog | Aucta — E-Settlement | https://trello.com/c/k1NnVAGt |
| Aucta — Backlog | Aucta — E-Vitrine | https://trello.com/c/2HX4yQjn |
| Aucta — Backlog | Aucta — P1 — Admin UI Hotwire: customers KYC approve/reject screens | https://trello.com/c/MLIjudGc |
| Aucta — Backlog | Aucta — P1 — CORE GAP-19 Área cliente API/UI parity (favorites, dispute, support, withdraw | https://trello.com/c/e3gRNOal |
| Aucta — Backlog | Aucta — P1 — CORE GAP-20 KYC update-request decisão admin | https://trello.com/c/9E8c9EsS |
| Aucta — Backlog | Aucta — P1 — KYC PF/PJ admin approve/reject parity | https://trello.com/c/lSuUpqJO |
| Aucta — Backlog | Aucta — P1 — [S2] Admin credential KYC creation queue + password/deactivate | https://trello.com/c/HK96qBFh |
| Aucta — Backlog | Aucta — P1 — [S2] Admin customer admin-profile + password-reset-email + won-lots | https://trello.com/c/VZWqWOEi |
| Aucta — Backlog | Aucta — P1 — [S2] Admin customers settlement records CRUD | https://trello.com/c/9INnELPk |
| Aucta — Backlog | Aucta — P1 — [S2] Admin data reports suite (sales/inadimplentes/resultado/…) | https://trello.com/c/wmR5mRtp |
| Aucta — Backlog | Aucta — P1 — [S2] Admin email-templates Cognito/SES CRUD | https://trello.com/c/OnFN56Xw |
| Aucta — Backlog | Aucta — P1 — [S2] Admin scripts list/execute parity | https://trello.com/c/KRJIZDVM |
| Aucta — Backlog | Aucta — P1 — [S2] Admin sellers CRUD parity | https://trello.com/c/vrjQtD6R |
| Aucta — Backlog | Aucta — P1 — [S2] Customer PF/PJ profile update + CNH/address uploads | https://trello.com/c/krY1Qm3T |
| Aucta — Backlog | Aucta — P1 — [S2] Document models (modelos de documentos) upload/list | https://trello.com/c/fyDSfW63 |
| Aucta — Backlog | Aucta — P1 — [S2] ImageProcessor + S3 auction/lot media pipeline | https://trello.com/c/FIyd6wso |
| Aucta — Backlog | Aucta — P1 — [S2] Import Excel lotes: fila + erros (import_job) | https://trello.com/c/DYcKzhhH |
| Aucta — Backlog | Aucta — P1 — [S2] Legal documents: revisions/publish/acceptances | https://trello.com/c/TeGok5w0 |
| Aucta — Backlog | Aucta — P1 — [S2] Legal files / legal documents admin | https://trello.com/c/RHJzNT9i |
| Aucta — Backlog | Aucta — P1 — [S2] Public payment_status on won lots / selling notes | https://trello.com/c/Jbq3S2St |
| Aucta — Backlog | Aucta — P1 — [S2] Sell interest endpoint público | https://trello.com/c/UVDXKFbI |
| Aucta — Backlog | Aucta — P2 — CORE GAP-17 Admin Hotwire épicos D–G (sellers, politics, reports, scripts) | https://trello.com/c/qVxVABDc |
| Aucta — Backlog | Aucta — P2 — CORE GAP-18 Admin UI módulos só no frontend/dist (fora Astro src) | https://trello.com/c/oUR7ykbG |
| Aucta — Backlog | Aucta — P2 — CORE GAP-21 Public UI auth + área cliente (signin/signup/customer/*) — refina | https://trello.com/c/XBHUKKtv |
| Aucta — Backlog | Aucta — P2 — CORE GAP-23 Bidder settlement UI + BR-SET (API OK) | https://trello.com/c/eXdEfQ1G |
| Aucta — Backlog | Aucta — P2 — E2E Playwright cadastro→lance→arremate | https://trello.com/c/2DBdI1WF |
| Aucta — Backlog | Aucta — P2 — Live realtime / Firestore listener docs + smoke | https://trello.com/c/UPDnRp3b |
| Aucta — Backlog | Aucta — P2 — Public frontend: home + PLP + PDP leilão (Vite/Hotwire) | https://trello.com/c/KTK4zSeM |
| Aucta — Backlog | Aucta — P2 — [S3] Admin UI Hotwire: auctions CRUD + auditorium + enabled | https://trello.com/c/5lIOTgA4 |
| Aucta — Backlog | Aucta — P2 — [S3] Admin UI Hotwire: reports suite (Habilitados/Vendas/Inadimplentes) | https://trello.com/c/1pnr865g |
| Aucta — Backlog | Aucta — P2 — [S3] Admin UI: scripts runner page | https://trello.com/c/Ja9IyZSd |
| Aucta — Backlog | Aucta — P2 — [S3] Admin UI: selling-note + payment modal + ranking | https://trello.com/c/yKsCAXPq |
| Aucta — Backlog | Aucta — P2 — [S3] Admin data endpoints / scripts parity | https://trello.com/c/He2PHLve |
| Aucta — Backlog | Aucta — P2 — [S3] Auth ACL migrator / Ability parity audit | https://trello.com/c/Eg1yvZkn |
| Aucta — Backlog | Aucta — P2 — [S3] Domain Financial Ops (fornecedor + cheque) | https://trello.com/c/hmddGFDk |
| Aucta — Backlog | Aucta — P2 — [S3] Domain Financing CRUD + elegibilidade | https://trello.com/c/6eAgYqxT |
| Aucta — Backlog | Aucta — P2 — [S3] Domain Judicial Auctions (cases/lots/attachments) | https://trello.com/c/BxsRgf7p |
| Aucta — Backlog | Aucta — P2 — [S3] Domain Live Realtime (sessions/events/lot state) | https://trello.com/c/2vbeZVFa |
| Aucta — Backlog | Aucta — P2 — [S3] Domain Materials + material inspection | https://trello.com/c/dyP4tiUO |
| Aucta — Backlog | Aucta — P2 — [S3] Domain Reports Export (templates/jobs/files) | https://trello.com/c/IrYaN9oY |
| Aucta — Backlog | Aucta — P2 — [S3] Domain Vehicle Operations + import queue | https://trello.com/c/AuEPxv8u |
| Aucta — Backlog | Aucta — P2 — [S3] Frontend-admin feature map → Hotwire pages backlog | https://trello.com/c/CJSxfCU9 |
| Aucta — Backlog | Aucta — P2 — [S3] Material inspection model + API (beyond lots import) | https://trello.com/c/xIgd0yAr |
| Aucta — Backlog | Aucta — P2 — [S3] Politics pages: privacy/terms/membership | https://trello.com/c/yiEL7l6c |
| Aucta — Backlog | Aucta — P2 — [S3] Property inspection API | https://trello.com/c/ygTRXcmq |
| Aucta — Backlog | Aucta — P2 — [S3] Public auditorium + Stream.io video embed | https://trello.com/c/TwtDG1Fg |
| Aucta — Backlog | Aucta — P2 — [S3] Stream.io video tokens (public /stream) | https://trello.com/c/raUxYknu |
| Aucta — Backlog | Aucta — P2 — [S3] Yard movement + supplier ops production decision | https://trello.com/c/XbsQgFG1 |
| Aucta — Backlog | Aucta — P2 — [S3] Yard movement / supplier ops (se usado em produção) | https://trello.com/c/MOUKLg3p |
| Aucta — Backlog | Aucta — P3 — Multitenant por subdomínio (skill multi-tenancy) | https://trello.com/c/6tN3nKdm |
| Aucta — Backlog | Aucta — P3 — Search hub MVP (busca de anúncios/leilões na plataforma) | https://trello.com/c/mikSRCHh |
| Aucta — Backlog | Aucta — P3 — [S4] IGNORE hygiene: auction_system + sodre + *_master zips | https://trello.com/c/MEAJfJd2 |
| Aucta — Backlog | Aucta — P3 — [S4] Track ERP: inventário osiris → decisões KEEP/REIMPLEMENT | https://trello.com/c/gO9YnQ3v |
| Aucta — Backlog | Aucta — P3 — [S4] Track V.Lance serrano: multi-tenant learning only | https://trello.com/c/BpACNLQh |
| Aucta — Backlog | Aucta — P3 — [S4] Track Vistoria API: decidir REIMPLEMENT mobile | https://trello.com/c/R11QE3bG |
| Aucta — Backlog | Aucta — [EPIC] E0 · Segurança e higiene de segredos | https://trello.com/c/7RWU7cu7 |
| Aucta — Backlog | Aucta — [EPIC] E1 · Multitenancy (isolamento de dados) | https://trello.com/c/Dw0NUoaJ |
| Aucta — Backlog | Aucta — [EPIC] E10 · Infra, deploy Linode LKE e observabilidade | https://trello.com/c/9jTSzd5P |
| Aucta — Backlog | Aucta — [EPIC] E11 · Site de busca (fase futura) | https://trello.com/c/Wvu9IK6o |
| Aucta — Backlog | Aucta — [EPIC] E12 · Fábrica: LangGraph e pack aucta_nexus | https://trello.com/c/8fFX4GwP |
| Aucta — Backlog | Aucta — [EPIC] E2 · Anúncios / Listings (persistência + ciclo de vida) | https://trello.com/c/ogj7jjLp |
| Aucta — Backlog | Aucta — [EPIC] E3 · Integração OLX | https://trello.com/c/gXmf7Nn8 |
| Aucta — Backlog | Aucta — [EPIC] E4 · Integração Mercado Pago | https://trello.com/c/Dy9sEWWo |
| Aucta — Backlog | Aucta — [EPIC] E5 · Leilão core — paridade de regras | https://trello.com/c/7avjwMPQ |
| Aucta — Backlog | Aucta — [EPIC] E6 · Pós-leilão e financeiro | https://trello.com/c/g8B7uqO0 |
| Aucta — Backlog | Aucta — [EPIC] E7 · Notificações | https://trello.com/c/dIvodg3N |
| Aucta — Backlog | Aucta — [EPIC] E8 · Frontend (Hotwire admin + React embedado) | https://trello.com/c/gN9dBNrO |
| Aucta — Backlog | Aucta — [EPIC] E9 · Migração de dados e paridade | https://trello.com/c/k7WQjSIi |
| Condo_Nexus — Backlog | condo_nexus — ERB CoreUI Fase 2 (#234) não é P0 soak | https://trello.com/c/FhseS89S |
| Condo_Nexus — Backlog | condo_nexus — app Expo (padrão public-mock) | https://trello.com/c/lDWWEOjx |
| FinHub — Backlog | FinBot App | https://trello.com/c/6lnP2wkf |
| FinHub — Backlog | FinHub | https://trello.com/c/8e5UcmzH |
| FinHub — Backlog | FinHub — Open Finance somente leitura + IA (OpenRouter) para detectar gastos | https://trello.com/c/annoEU7V |
| FinHub — Backlog | FinHub — Triagem Sentry (unresolved 14d) 2026-10-02 | https://trello.com/c/FU31vU5S |
| FinHub — Backlog | FinHub — backtest noturno: robô sem saldo usa a cripto de maior valor | https://trello.com/c/PFLiSwl8 |
| FinHub — Backlog | FinHub — enxugar scripts de trade (só os lucrativos) | https://trello.com/c/FM3nKmTi |
| FinHub — Backlog | FinHub — portar os 39 robôs MT5 (metatrader.zip) como scripts paper | https://trello.com/c/ZjKnrhMi |
| FinHub — Backlog | FinHub — robôs profit-first (só operar com lucro líquido) | https://trello.com/c/O0DXRz5f |
| FinHub — Backlog | FinHub — técnicas avançadas de entrada/saída nos 16 scripts de trade | https://trello.com/c/Q98kWKqI |
| Public-Mock-Exams — Backlog | Simulados mobile — paginação em todas as telas (Figma Pagination) | https://trello.com/c/yQkdI5J9 |
| Public-Mock-Exams — Backlog | mock-exams — Rebrand mobile (Figma) | https://trello.com/c/EOudYWU3 |
| Public-Mock-Exams — Backlog | public-mock-exams (Simulados) — Play Billing 8+ e target API 36 em todas as faixas | https://trello.com/c/HI2Fqx3g |
| SysFinCon — Backlog | sysfincon — zerar quarentena de specs (346 exemplos) | https://trello.com/c/0mfIepzR |
| ❌ Erros/Bloqueios | aucta_nexus — migrations com versão duplicada 20261019120000 impedem o boot dos testes | https://trello.com/c/QShkFAkw |
| 📥 Backlog | Appointment Scheduler | https://trello.com/c/fSiYu5pk |
| 📥 Backlog | B.O.R.A. Automações | https://trello.com/c/0tV72fpU |
| 📥 Backlog | Calculadoras-SEO | https://trello.com/c/aK7T6OSX |
| 📥 Backlog | CloudSage Engine | https://trello.com/c/IhV8OmbS |
| 📥 Backlog | Frota mobile: anúncios e mensalidade para gerar renda | https://trello.com/c/E4LhH3XS |
| 📥 Backlog | Frota — bugs de produção achados pelo smoke e2e-fleet | https://trello.com/c/vBpgj72C |
| 📥 Backlog | Frota — rack-proxy GHSA-42qh-8mx8-7wqm (vite_ruby 3.11.1) | https://trello.com/c/kZDPtXfR |
| 📥 Backlog | Fábrica — auditoria agentes × codebase real (ajustes) | https://trello.com/c/6VrW9s0e |
| 📥 Backlog | Hybrid Inertia leftover — domain-checker, bora, agendaflow | https://trello.com/c/zPehp02g |
| 📥 Backlog | Main vermelha — medrate / med_orbit / public-mock-exams | https://trello.com/c/uMH3gkum |
| 📥 Backlog | Notifire | https://trello.com/c/zvFGLDkC |
| 📥 Backlog | P1 — nested table wrap (ordempro, AS, renovafacil) | https://trello.com/c/wyAfE2yV |
| 📥 Backlog | PayLoop | https://trello.com/c/iIpmTdZF |
| 🚧 Em Desenvolvimento | bff-admin — build Docker quebrado: vendor no bundle install (corrigido) e TypeScript do fr | https://trello.com/c/x7Ql0Ea6 |

## Bloqueados por humano (87)

| Lista | Card | Link |
|---|---|---|
| Aucta — Backlog | Aucta — CI boot Rails 8 + primeiro deploy LKE | https://trello.com/c/FVT4rUNB |
| Aucta — Backlog | Aucta — [AUC-001] Rotacionar webhook do Discord vazado em origem/plataforma | https://trello.com/c/Wj1Qnq8G |
| Aucta — Backlog | Aucta — [AUC-002] Varrer e purgar segredos e certificados de origem/ (chaves, senhas, .env | https://trello.com/c/3FdnAtwP |
| Aucta — Backlog | Aucta — [AUC-100] Deploy no Linode LKE via platform-infra (GHCR, rails-lke, health /up) | https://trello.com/c/GQqP7tln |
| Aucta — Backlog | Aucta — [AUC-120] Grafo LangGraph: ler card → tech-lead → agente dono → QA/BDD → review →  | https://trello.com/c/fGjGfau5 |
| Condo_Nexus — Backlog | WAIT_HUMAN — 1ª publicação Play com.conexus.condonexus (AAB local pronto, 404) | https://trello.com/c/yL53OKmR |
| Condo_Nexus — Backlog | [condo_nexus] fix CI SimpleCov baseline path under web/docs | https://trello.com/c/38rxNCAB |
| Condo_Nexus — Backlog | condo_nexus — WAIT_HUMAN terraform apply TF vs cluster (#245) | https://trello.com/c/SvecsIpC |
| Condo_Nexus — Backlog | condo_nexus — cutover: deploy main no LKE ✅ → soak Auth/Billing Heloisa | https://trello.com/c/ITqSspgA |
| Condo_Nexus — Backlog | condo_nexus — login SPA: Esqueci a senha + Entrar com Google (Auth BFF) | https://trello.com/c/gW11S7Da |
| Conexus-Launcher — Backlog | conexus-launcher — 0.1.6 integração BFFs (conta Conexus + doação) · falta OAuthClient | https://trello.com/c/Qs7VQpfh |
| FinHub — Backlog | Expo — WAIT_HUMAN: secret EXPO_TOKEN (public-mock-exams + fin-hub) | https://trello.com/c/SV2QCOlt |
| FinHub — Backlog | FinHub — Platform V2: incorporar robôs open source (análise + roadmap + Sprints 1–4 + tele | https://trello.com/c/IA0iaySa |
| FinHub — Backlog | FinHub — Roadmap de recursos estilo Quantfury e Foxbit | https://trello.com/c/26vfFRe7 |
| FinHub — Backlog | FinHub — allow list do Postgres não cobre nós novos do LKE (deploy 503) | https://trello.com/c/ppLF4Kt3 |
| FinHub — Backlog | FinHub — apagar PV e volume Linode do Redis removido (US$ cobrado) | https://trello.com/c/EsJG8oIe |
| FinHub — Backlog | FinHub — ativar todos os scripts em paper (Bitget + Kraken), backtest e vendas no Discord  | https://trello.com/c/vBOo9o9H |
| FinHub — Backlog | FinHub — falso "Pendência de atualização" nos robôs de dividendos Bitget (rTokens) | https://trello.com/c/HSKxRzsg |
| FinHub — Backlog | FinHub — merge de todas as demandas + Solid Queue sem Redis + deploy prod (07/10) | https://trello.com/c/uhjMwvKU |
| FinHub — Backlog | FinHub — plano de renda / tuning (Bitget + Kraken, 04/10) | https://trello.com/c/ugiGH5cB |
| FinHub — Backlog | WAIT_HUMAN — criar Play com.conexus.finhub (404) | https://trello.com/c/7YMHHsnk |
| FinHub — Backlog | fin-hub — Dependabot mobile + EAS preview (01/10) | https://trello.com/c/hPbpoCpj |
| FinHub — Backlog | fin_hub — P0: adapter Foxbit (spot BRL, chaves do usuário) | https://trello.com/c/ijuAlVtE |
| FinHub — Backlog | fin_hub — PF Fase 2: extrato, metas, orçamento e passivos | https://trello.com/c/ktb4OPGp |
| FinHub — Backlog | fin_hub — migrar Auth/Billing para BFFs | https://trello.com/c/KW7CnvGC |
| FinHub — Backlog | fin_hub — técnicas avançadas nos 16 scripts de trade (renda passiva) | https://trello.com/c/5Mr6rdOj |
| Posto-Check — Backlog | posto-check — Auth/Billing BFF + Sentry P2 (sem flip messaging) | https://trello.com/c/mTv08uHM |
| Public-Mock-Exams — Backlog | Batch A — smoke SSO/checkout mock-exams + Med Orbit | https://trello.com/c/x4jHDTlR |
| Public-Mock-Exams — Backlog | Simulados Play: link de fonte gov inacessível (declarações enganosas) | https://trello.com/c/pUGOdHxG |
| Public-Mock-Exams — Backlog | Simulados — email ops em carga de concursos, estudos e cadastros | https://trello.com/c/KVBpPfXn |
| Public-Mock-Exams — Backlog | Simulados — suporte e afins no Discord (tópicos) | https://trello.com/c/tGX1vqp4 |
| Public-Mock-Exams — Backlog | WAIT_HUMAN — Simulados: criar blocos AdMob (rewarded/app open) + mensagens de privacidade | https://trello.com/c/6vvUMGfi |
| SysFinCon — Backlog | Cliente assina mensalidade SysFinCon (GTM checkout) | https://trello.com/c/2fnATFfz |
| SysFinCon — Backlog | SysFinCon — ops 1ª fatura (secret, planos, webhook, smoke) | https://trello.com/c/v3ZxbqDd |
| SysFinCon — Backlog | conexus-platform — release 0.5.0 + re-vendor (sysfincon redirect_uri) | https://trello.com/c/kNrl4Mc2 |
| SysFinCon — Backlog | sysfincon — Adriana Onda 1: router e alçada em todas transições M03 | https://trello.com/c/mBG88x0h |
| SysFinCon — Backlog | sysfincon — Entrar com Google multi-tenant + E2E + app mobile Expo | https://trello.com/c/nrkSmTot |
| SysFinCon — Backlog | sysfincon — drift de schema em produção (public + tenants) | https://trello.com/c/GbVKwCRT |
| SysFinCon — Backlog | sysfincon — main quebrada (boot prod + build frontend) | https://trello.com/c/KjnrqXjO |
| 👤 Humano — só Tiago | Aucta — DECISÃO HUMANA: fluxo de fundos Mercado Pago + cadastro MP | https://trello.com/c/tdQHTFPv |
| 📥 Backlog | AbaFind — login app=site + identidade Broadsheet | https://trello.com/c/O6k9ihjs |
| 📥 Backlog | AntiSpam BR — M0/M1 Android nativo offline + M2 API comunitária PostgreSQL | https://trello.com/c/wswWoKxF |
| 📥 Backlog | AntiSpam BR — liberar conta de serviço no app com.conexus.antispam (Play Console) | https://trello.com/c/ASUMEEfE |
| 📥 Backlog | Auditoria de gaps 2026-10-01 — orquestradores | https://trello.com/c/jQGQKxil |
| 📥 Backlog | Batch B — soak SSO/billing MicroSaaS | https://trello.com/c/jxciEWcH |
| 📥 Backlog | Cloudflare — Origin CA nos ingress LKE, depois flip SSL Flexible → Full (strict) | https://trello.com/c/QqB59pPn |
| 📥 Backlog | Dependabot — WAIT_HUMAN: liberar acesso do Dependabot ao platform-infra (destrava 9 repos) | https://trello.com/c/vYzlIzzm |
| 📥 Backlog | Dependabot — alertas de segurança mobile + triagem PRs (03/10) | https://trello.com/c/v2ln2b0m |
| 📥 Backlog | Discord — ativo em todos os sistemas (canal por app + aviso de deploy) | https://trello.com/c/O46zetLl |
| 📥 Backlog | Frota producao — 12 frentes (levantamento 22/09) | https://trello.com/c/W7Ub7tT7 |
| 📥 Backlog | Frota — AdMob em todos os apps: criar apps/blocos no AdMob + Play "Contém anúncios" | https://trello.com/c/qcYjK3gE |
| 📥 Backlog | Frota — apagar 8 volumes Linode dos Redis removidos + pendências do cutover Solid | https://trello.com/c/9152B8o9 |
| 📥 Backlog | Fábrica — WAIT_HUMAN: desligar atribuição de commits do agente no Cursor (Co-authored-by) | https://trello.com/c/xwbY5kzI |
| 📥 Backlog | Fábrica — agente trello-card-runner | https://trello.com/c/ZKvRFYhx |
| 📥 Backlog | Fábrica — agentes app-portfolio + brand-designer, skills app-fleet / brand-identity / stor | https://trello.com/c/dol8VvQx |
| 📥 Backlog | Fábrica — reorganização total do board Conexus-Systems | https://trello.com/c/6sbTgrPB |
| 📥 Backlog | Fábrica — smoke do adapter Hermes no servidor (PR agents#5) | https://trello.com/c/ZDS5VHNj |
| 📥 Backlog | Fábrica — suíte E2E completa como gate de entrega | https://trello.com/c/rVLh26U8 |
| 📥 Backlog | HUMANO — MercadoPago notification URL (payment + subscription_preapproval) | https://trello.com/c/epEowPM3 |
| 📥 Backlog | HUMANO — aprovar PRs (auto-merge dependabot + fixes) | https://trello.com/c/1ALjSgtW |
| 📥 Backlog | Humano — gh auth delete_repo + apagar 7 *-app/*-mobile | https://trello.com/c/cgjpQdAX |
| 📥 Backlog | P0 — Fábrica aponta para boards Trello mortos + SHARED_CONTEXT desatualizado | https://trello.com/c/jIbMtiDq |
| 📥 Backlog | P0 — Trabalho fora da main: commits só locais e branches sem PR | https://trello.com/c/qYLDU95P |
| 📥 Backlog | P0 — aba-find: admin de seed com senha fixa em produção | https://trello.com/c/gOn98g74 |
| 📥 Backlog | P1 — Frota: CI sem trigger/vermelho e deploys parados | https://trello.com/c/l865RBMV |
| 📥 Backlog | Play — testes internos (adicionar testers) | https://trello.com/c/sU04B3eE |
| 📥 Backlog | WAIT_HUMAN — Med Orbit Play com.conexus.med_orbit: enviar 1ª produção | https://trello.com/c/FL9ApmqB |
| 📥 Backlog | WAIT_HUMAN — PIX/boleto E2E (ambiente + PSP + fatura teste) | https://trello.com/c/C5QpcGFL |
| 📥 Backlog | WAIT_HUMAN — Play MCP sem service account (~/secrets/google-play-service-account.json) | https://trello.com/c/FAzSLBLe |
| 📥 Backlog | WAIT_HUMAN — k8s: confirmar workload xmr-miner (política Linode/Akamai) | https://trello.com/c/9g7MXpme |
| 📥 Backlog | Wave 6 — teardown dual-run (OmniAuth/MP/Stripe local) | https://trello.com/c/XBdMton1 |
| 📥 Backlog | aba-find — Playwright E2E SSO e lead unlock | https://trello.com/c/WwwDaPq8 |
| 📥 Backlog | bff-e2e — repo central de E2E de todos os sistemas (Playwright, k6, smoke da frota) | https://trello.com/c/ECt5M7Xi |
| 📥 Backlog | fin-hub — issues abertas: PRs para review (Quantfury/Foxbit/Open Finance) | https://trello.com/c/FhXGDDQZ |
| 📥 Backlog | hermes-agent — autonomia: esteira assume premissas em vez de perguntar | https://trello.com/c/BMWXiUjy |
| 📥 Backlog | med_orbit — Portal do tutor: gaps competitivos vs SimplesVet | https://trello.com/c/xAVPEQ4Z |
| 📥 Backlog | med_orbit — gaps para voltar à produção (503 desde ~01/10) | https://trello.com/c/NFsDY5vv |
| 📥 Backlog | med_orbit_app — 1ª publicação Play + mensalidade web | https://trello.com/c/BYVngQEv |
| 📥 Backlog | medrate — Auth/Billing BFF + Sentry P2 + KUBE_CONFIG_B64 | https://trello.com/c/oTEwb1Au |
| 🚧 Em Desenvolvimento | API BFF — sistema MCP Discord + Solid Queue (LIVE, bot ativo) | https://trello.com/c/YU3o39Ld |
| 🚧 Em Desenvolvimento | Auth BFF — endpoints de serviço de identidade (email_verified, groups, delete) para Aucta | https://trello.com/c/twy3mNbT |
| 🚧 Em Desenvolvimento | Auth BFF — provisionamento de identidades por app (POST /identities, login, senha, email)  | https://trello.com/c/twiZVtjC |
| 🚧 Em Desenvolvimento | Catálogo comercial — produtos/planos públicos + checkout Payments BFF | https://trello.com/c/jyURjHmA |
| 🚧 Em Desenvolvimento | Fábrica — Solid stack, dispatch no orquestrador e agentes bff-auth/bff-payment/bff-api | https://trello.com/c/yjEx0khC |
| 🚧 Em Desenvolvimento | HUMANO — Discord/Twilio só no API BFF (alerts + mapa) | https://trello.com/c/f1AjIaG0 |
| 🚧 Em Desenvolvimento | Payments BFF — pagamento cripto rastreado (NOWPayments): mensalidade + doação | https://trello.com/c/iAarcccT |
| 🚧 Em Desenvolvimento | bff-api — Unificação api_core_nexus → bff-api (docs   arquivamento repo) | https://trello.com/c/Q6IuSmWs |

## Possivelmente concluídos (6)

| Lista | Card | Link |
|---|---|---|
| FinHub — Backlog | FinHub — Correções pós-review da Performance Fase 1 (#478–#482) | https://trello.com/c/SWGeSscD |
| FinHub — Backlog | FinHub — merge de todas as branches + Dependabot e deploy prod (05/10) | https://trello.com/c/YzRoIB0W |
| 📥 Backlog | Dependabot — 9 majors segurados (8 resolvidos, 1 segurado) | https://trello.com/c/Xzei6Fg5 |
| 📥 Backlog | Fábrica — grafo padrão do deploy: Validate branch → testes em paralelo → build → deploy | https://trello.com/c/KooA2Frm |
| 📥 Backlog | GCP — autorizar redirect URIs Google OAuth da frota | https://trello.com/c/ZIpZPMna |
| 📥 Backlog | Mobile frota — RN 0.87 fora do Expo SDK 57 (expo install --fix) | https://trello.com/c/quz34Xyb |

## Dependências encontradas (por que a onda 1 não avança)

- **Escopo de canais realtime por tenant (yIAovZRe):** depende de isolamento de tenant (AUC-011/012), que ainda não existe no schema (0 `tenant_id`). Sem a fundação de tenancy, o Gherkin não é testável. Depende de ADR AUC-010.
- **customer_admin bloqueado (kGhklMlH):** código pronto em `fix/auc-customer-admin-blocked` (527e998), mas a suíte não roda por migration duplicada `20261019120000` em `main`. Card QShkFAkw.
- **Épicos AUC-010…121:** precisam de ADR de tenancy (AUC-010) e de decisão de Mercado Pago (AUC-040) antes de qualquer código.
