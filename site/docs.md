---
layout: default
title: "Documentação"
description: "Documentação técnica do Conexus Mobile OS: arquitetura, build, porting, API reference e guias."
---

<div class="container">
  <header class="page-header">
    <h1>Documentação Técnica</h1>
    <p>Documentação completa para desenvolvedores, porters e usuários avançados do Conexus Mobile OS.</p>
  </header>
  
  <nav class="doc-nav" aria-label="Navegação da documentação">
    <ul class="doc-nav-list">
      <li><a href="#getting-started" class="doc-nav-link">Começando</a></li>
      <li><a href="#architecture" class="doc-nav-link">Arquitetura</a></li>
      <li><a href="#build" class="doc-nav-link">Build & Desenvolvimento</a></li>
      <li><a href="#porting" class="doc-nav-link">Porting para Novos Devices</a></li>
      <li><a href="#api" class="doc-nav-link">API Reference (SDK)</a></li>
      <li><a href="#security" class="doc-nav-link">Segurança</a></li>
      <li><a href="#contributing" class="doc-nav-link">Contribuindo</a></li>
    </ul>
  </nav>
  
  <main class="doc-content">
    <section id="getting-started" class="doc-section">
      <h2>Começando</h2>
      <div class="doc-grid">
        <article class="doc-card">
          <h3><a href="{{ '/download' | relative_url }}">Instalação Rápida</a></h3>
          <p>Guia passo a passo para instalar no Pixel 8a ou Nothing Phone (2).</p>
          <span class="doc-tag">Iniciante</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/install' | relative_url }}">Guia Completo de Instalação</a></h3>
          <p>Procedimento detalhado: unlock, flash, recovery, primeiro boot, OTA.</p>
          <span class="doc-tag">Iniciante</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/troubleshooting' | relative_url }}">Troubleshooting</a></h3>
          <p>Problemas comuns: bootloop, bootloader unlock falha, OTA falha, etc.</p>
          <span class="doc-tag">Intermediário</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/first-boot' | relative_url }}">Primeiro Boot & Configuração</a></h3>
          <p>O que esperar no primeiro boot, configuração inicial, contas, OTA.</p>
          <span class="doc-tag">Iniciante</span>
        </article>
      </div>
    </section>
    
    <section id="architecture" class="doc-section">
      <h2>Arquitetura</h2>
      <div class="doc-grid">
        <article class="doc-card">
          <h3><a href="{{ '/docs/architecture' | relative_url }}">Visão Geral da Arquitetura</a></h3>
          <p>Camadas: Conexus Experience, Framework, System Services, LineageOS/Android.</p>
          <span class="doc-tag">Visão Geral</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/launcher' | relative_url }}">Conexus Launcher</a></h3>
          <p>Start Screen, App Drawer, Action Center, Navigation, Settings, Lock Screen.</p>
          <span class="doc-tag">Shell</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/system-services' | relative_url }}">System Services</a></h3>
          <p>Tile Engine, Notification Broker, Nav Controller, App Manager, Permission Manager.</p>
          <span class="doc-tag">Services</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/ipc' | relative_url }}">IPC & Binder/D-Bus</a></h3>
          <p>Arquitetura de isolamento: AIDL no Android, D-Bus no Linux, mock in-process.</p>
          <span class="doc-tag">Internals</span>
        </article>
      </div>
    </section>
    
    <section id="build" class="doc-section">
      <h2>Build & Desenvolvimento</h2>
      <div class="doc-grid">
        <article class="doc-card">
          <h3><a href="{{ '/docs/build' | relative_url }}">Guia de Build (Pixel 8a)</a></h3>
          <p>repo init, sync, vendor blobs, kernel GKI, build completo, flash.</p>
          <span class="doc-tag">Desenvolvedor</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/build-environment' | relative_url }}">Ambiente de Build</a></h3>
          <p>Ubuntu 24.04, dependências, ccache, repo tool, hardware recomendado.</        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/vendor-blobs' | relative_url }}">Extração de Vendor Blobs</a></h3>
          <p>extract-files.sh, factory images, validação SHA-256, troubleshooting.</p>
          <span class="doc-tag">Desenvolvedor</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/kernel-build' | relative_url }}">Kernel GKI Build</a></h3>
          <p>build_akita.sh, GKI 6.1, configuração, modules, validação.</        </article>
      </div>
    </section>
    
    <section id="porting" class="doc-section">
      <h2>Porting para Novos Dispositivos</h2>
      <div class="doc-grid">
        <article class="doc-card">
          <h3><a href="{{ '/docs/porting' | relative_url }}">Guia de Porting Completo</a></h3>
          <p>Requisitos, device tree, kernel, vendor, SELinux, build, validação.</        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/porting-requirements' | relative_url }}">Requisitos Mínimos</a></h3>
          <p>Bootloader unlock, kernel source, LineageOS 23.2, device tree, recovery.</p>
          <span class="doc-tag">Pré-requisitos</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/porting-selinux' | relative_url }}">SELinux Policy</a></h3>
          <p>device/conexus/sepolicy, neverallow rules, domain transitions, file contexts.</p>
          <span class="doc-tag">Avançado</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/porting-validation' | relative_url }}">Validação & Testes</a></h3>
          <p>Boot, touch, radio, camera, audio, sensors, battery, OTA, stress tests.</        </article>
      </div>
    </section>
    
    <section id="api" class="doc-section">
      <h2>API Reference (SDK)</h2>
      <div class="doc-grid">
        <article class="doc-card">
          <h3><a href="{{ '/docs/sdk/kotlin' | relative_url }}">Kotlin SDK</a></h3>
          <p>TileProvider, NavController, NotificationBuilder, PermissionManager, FluentTheme.</p>
          <span class="doc-tag">Kotlin</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/sdk/typescript' | relative_url }}">TypeScript/React Native SDK</a></h3>
          <p>Components, Hooks, Theme, Native Bridge (JSI), Tile, Navigation, Notifications.</        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/sdk/contracts' | relative_url }}">Shared Contracts (JSON Schema)</a></h3>
          <p>tile-manifest.json, app-manifest.json, notification.json — validação cross-platform.</p>
          <span class="doc-tag">Contracts</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/sdk/aidl' | relative_url }}">AIDL Interfaces</a></h3>
          <p>ITileEngine, INavController, INotificationBroker, IPermissionManager, etc.</        </article>
      </div>
    </section>
    
    <section id="security" class="doc-section">
      <h2>Segurança</h2>
      <div class="doc-grid">
        <article class="doc-card">
          <h3><a href="{{ '/docs/security/model' | relative_url }}">Threat Model (STRIDE)</a></h3>
          <p>Atores, vetores, mitigações, superfície de ataque, trust boundaries.</p>
          <span class="doc-tag">Security</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/security/verified-boot' | relative_url }}">Verified Boot (AVB v2)</a></h3>
          <p>Assinaturas, rollback protection, anti-rollback, vbmeta, chaves offline.</p>
          <span class="doc-tag">Security</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/security/selinux' | relative_url }}">SELinux Policy</a></h3>
          <p>Enforcing mode, custom policies, neverallow, domain transitions, audit.</p>
          <span class="doc-tag">Security</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/security/supply-chain' | relative_url }}">Supply Chain Security</a></h3>
          <p>SBOM (SPDX), license compliance, vuln scanning (Grype/Trivy), reproducible builds.</p>
          <span class="doc-tag">Security</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/security/keys' | relative_url }}">Key Management</a></h3>
          <p>Dev/CI/Alpha/Production keys, rotação, HSM/YubiKey, assinatura manual release.</        </article>
      </div>
    </section>
    
    <section id="contributing" class="doc-section">
      <h2>Contribuindo</h2>
      <div class="doc-grid">
        <article class="doc-card">
          <h3><a href="{{ '/contribute' | relative_url }}">Guia de Contribuição</a></h3>
          <p>Como contribuir: issues, PRs, code review, CI, code style, testing.</p>
          <span class="doc-tag">Todos</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/conduct' | relative_url }}">Código de Conduta</a></h3>
          <p>Contributor Covenant v2.1 — ambiente inclusivo e respeitoso.</        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/code-style' | relative_url }}">Code Style & Linting</a></h3>
          <p>ktlint, detekt, ESLint, Prettier, ktfmt, ktfmt-gradle, spotless.</p>
          <span class="doc-tag">Desenvolvedor</span>
        </article>
        <article class="doc-card">
          <h3><a href="{{ '/docs/testing' | relative_url }}">Testing Guidelines</a></h3>
          <p>Unit (JUnit/Vitest), Integration (Espresso/UIAutomator), Device, E2E (Detox/Playwright).</p>
          <span class="doc-tag">QA</span>
        </article>
      </div>
    </section>
  </main>
  
  <aside class="doc-sidebar">
    <nav class="doc-toc" aria-label="Sumário">
      <h3>Nesta página</h3>
      <ul class="toc-list">
        <li><a href="#getting-started">Começando</a></li>
        <li><a href="#architecture">Arquitetura</a></li>
        <li><a href="#build">Build & Desenvolvimento</a></li>
        <li><a href="#porting">Porting</a></li>
        <li><a href="#api">API Reference</a></li>
        <li><a href="#security">Segurança</a></li>
        <li><a href="#contributing">Contribuindo</a></li>
      </ul>
    </nav>
  </aside>
</div>

<style>
.doc-nav { position: sticky; top: 80px; margin-bottom: var(--spacing-xl); }
.doc-nav-list { display: flex; flex-wrap: wrap; gap: var(--spacing-s); list-style: none; padding: 0; margin: 0; }
.doc-nav-link { padding: var(--spacing-s) var(--spacing-m); background: var(--color-surface); border: 1px solid var(--color-divider); border-radius: 0; font-size: 0.8125rem; color: var(--color-text-secondary); transition: all var(--transition-fast); }
.doc-nav-link:hover { border-color: var(--color-accent); color: var(--color-accent); }
.doc-content { display: grid; grid-template-columns: 1fr 250px; gap: var(--spacing-xl); }
@media (max-width: 900px) { .doc-content { grid-template-columns: 1fr; } .doc-sidebar { display: none; } }
.doc-section { margin-bottom: var(--spacing-xxxl); }
.doc-section h2 { padding-bottom: var(--spacing-m); border-bottom: 1px solid var(--color-divider); margin-bottom: var(--spacing-xl); }
.doc-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: var(--spacing-xl); }
.doc-card { background: var(--color-surface); border: 1px solid var(--color-divider); padding: var(--spacing-xl); transition: transform var(--transition-normal), box-shadow var(--transition-normal); }
.doc-card:hover { transform: translateY(-4px); box-shadow: var(--shadow-3); }
.doc-card h3 { margin-top: 0; margin-bottom: var(--spacing-s); font-size: 1.125rem; }
.doc-card p { color: var(--color-text-secondary); font-size: 0.9375rem; margin-bottom: var(--spacing-m); }
.doc-tag { display: inline-block; padding: 2px 8px; font-size: 0.625rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; background: var(--color-accent); color: var(--color-on-accent); }
.doc-sidebar { position: sticky; top: 100px; max-height: calc(100vh - 180px); overflow-y: auto; padding: var(--spacing-xl); background: var(--color-surface); border: 1px solid var(--color-divider); border-radius: 0; }
.doc-toc h3 { font-size: 0.875rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: var(--spacing-m); color: var(--color-text-secondary); }
.toc-list { list-style: none; padding: 0; margin: 0; }
.toc-list li { margin: var(--spacing-s) 0; }
.toc-list a { color: var(--color-text-secondary); font-size: 0.875rem; transition: color var(--transition-fast); display: block; padding: 2px 0; }
.toc-list a:hover { color: var(--color-accent); }
</style>