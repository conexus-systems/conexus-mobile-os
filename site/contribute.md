---
layout: default
title: "Contribuir"
description: "Como contribuir com o Conexus Mobile OS: issues, pull requests, code review, desenvolvimento, documentação."
---

<div class="container">
  <header class="page-header">
    <h1>Contribuir com o Conexus Mobile OS</h1>
    <p>Obrigado pelo interesse em contribuir! O Conexus Mobile OS é um projeto open source comunitário e toda contribuição é bem-vinda.</p>
  </header>
  
  <section class="contribute-section">
    <h2>Formas de Contribuir</h2>
    <div class="contribute-grid">
      <article class="contribute-card">
        <div class="contribute-icon">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg>
        </div>
        <h3>Reportar Bugs</h3>
        <p>Encontrou um bug? Abra uma issue no GitHub com steps to reproduce, logs, device info.</p>
        <a href="https://github.com/conexus-mobile-os/conexus-mobile-os/issues/new/choose" target="_blank" rel="noopener" class="btn btn-sm btn-primary">Abrir Issue</a>
      </article>
      
      <article class="contribute-card">
        <div class="contribute-icon">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg>
        </div>
        <h3>Sugerir Features</h3>
        <p>Tem uma ideia para melhorar o Conexus Mobile OS? Abra uma feature request.</p>
        <a href="https://github.com/conexus-mobile-os/conexus-mobile-os/issues/new/choose" target="_blank" rel="noopener" class="btn btn-sm btn-primary">Sugerir Feature</a>
      </article>
      
      <article class="contribute-card">
        <div class="contribute-icon">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg>
        </div>
        <h3>Contribuir Código</h3>
        <p>Pull requests para: launcher, system services, SDK, build system, device trees.</p>
        <a href="#code-contribution" class="btn btn-sm btn-primary">Guia de PR</a>
      </article>
      
      <article class="contribute-card">
        <div class="contribute-icon">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg>
        </div>
        <h3>Documentação</h3>
        <p>Melhore docs, traduções, guias de instalação, API reference, porting guides.</        </article>
        
        <article class="contribute-card">
          <div class="contribute-icon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg>
          </div>
          <h3>Testes & QA</h3>
          <p>Testes em device, relatórios de compatibilidade, regression testing, performance.</p>
          <a href="#testing" class="btn btn-sm btn-primary">Guia de Testes</a>
        </article>
        
        <article class="contribute-card">
          <div class="contribute-icon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg>
          </div>
          <h3>Porting para Novos Devices</h3>
          <p>Ajude a portar para novos dispositivos: device tree, kernel, vendor, SELinux.</p>
          <a href="{{ '/docs/porting' | relative_url }}" class="btn btn-sm btn-primary">Guia de Porting</a>
        </article>
      </div>
    </section>
    
    <section id="code-contribution" class="contribute-section">
      <h2>Guia de Contribuição de Código</h2>
      
      <h3>1. Setup do Ambiente</h3>
      <pre><code># Fork o repo
git clone https://github.com/SEU_USUARIO/conexus-mobile-os.git
cd conexus-mobile-os

# Adicionar upstream
git remote add upstream https://github.com/conexus-mobile-os/conexus-mobile-os.git

# Criar branch
git checkout -b feat/minha-feature</code></pre>
      
      <h3>2. Padrões de Código</h3>
      <ul>
        <li><strong>Kotlin:</strong> ktlint + detekt + ktfmt (config no repo)</li>
        <li><strong>TypeScript/TSX:</strong> ESLint + Prettier + TypeScript strict</li>
        <li><strong>Commits:</strong> Conventional Commits (<code>feat:</code>, <code>fix:</code>, <code>docs:</code>, <code>refactor:</code>, etc.)</li>
        <li><strong>Branch naming:</strong> <code>feat/nome-da-feature</code>, <code>fix/nome-do-bug</code>, <code>docs/o-que-mudou</code></li>
      </ul>
      
      <h3>3. Processo de Pull Request</h3>
      <ol>
        <li>Crie branch a partir de <code>main</code></li>
        <li>Implemente a mudança com testes</li>
        <li>Rode lint/typecheck/tests localmente</li>
        <li>Abra PR contra <code>main</code> do upstream</li>
        <li>Preencha o template de PR completamente</li>
        <li>Aguarde code review (mínimo 1 aprovação)</li>
        <li>CI deve passar (build, tests, lint, security scan)</li>
        <li>Merge via squash and merge</li>
      </ol>
      
      <h3>4. Templates de PR</h3>
      <p>Use o template apropriado:</p>
      <ul>
        <li><code>.github/PULL_REQUEST_TEMPLATE/feature.md</code> — Nova feature</li>
        <li><code>.github/PULL_REQUEST_TEMPLATE/fix.md</code> — Bug fix</li>
        <li><code>.github/PULL_REQUEST_TEMPLATE/docs.md</code> — Documentação</li>
        <li><code>.github/PULL_REQUEST_TEMPLATE/porting.md</code> — Porting de device</li>
      </ul>
      
      <h3>5. Code Review</h3>
      <ul>
        <li>Mínimo 1 aprovação de maintainer</li>
        <li>CI deve passar (build, unit tests, integration tests, lint, security scan)</li>
        <li>Resolver todos os comments antes do merge</li>
        <li>Squash and merge no final</li>
      </ul>
    </section>
    
    <section id="testing" class="contribute-section">
      <h2>Testes & QA</h2>
      
      <h3>Tipos de Teste</h3>
      <div class="test-types">
        <article class="test-card">
          <h3>Unit Tests</h3>
          <ul>
            <li><strong>Kotlin:</strong> JUnit 5 + MockK — <code>./gradlew test</code></li>
            <li><strong>TypeScript:</strong> Vitest — <code>pnpm test</code></li>
            <li>Cobertura mínima: 80% para novo código</li>
          </ul>
        </article>
        
        <article class="test-card">
          <h3>Integration Tests</h3>
          <ul>
            <li><strong>Android:</strong> Espresso + UIAutomator</li>
            <li><strong>SDK:</strong> Contract tests (Tile, Nav, Notifications)</li>
            <li><strong>Services:</strong> AIDL interface compliance</li>
          </ul>
        </article>
        
        <article class="test-card">
          <h3>Device Tests</h3>
          <ul>
            <li><strong>Boot/Recovery:</strong> 10/10 consecutive boots</li>
            <li><strong>Hardware:</strong> Touch, radio, camera, audio, sensors, battery</li>
            <li><strong>Stress:</strong> Monkey test, thermal, battery drain</li>
          </ul>
        </article>
        
        <article class="test-card">
          <h3>E2E Tests</h3>
          <ul>
            <li><strong>React Native:</strong> Detox</li>
            <li><strong>Web:</strong> Playwright</li>
            <li>Fluxos críticos: boot → launcher → tile → app → notification → settings</li>
          </ul>
        </article>
      </div>
      
      <h3>Executando Testes Localmente</h3>
      <pre><code># Kotlin unit tests
./gradlew test

# TypeScript unit tests
pnpm test

# Android integration tests (precisa device/emulator)
./gradlew connectedAndroidTest

# Lint
./gradlew lint
pnpm lint

# Type check
pnpm typecheck</code></pre>
    </section>
    
    <section class="contribute-section">
      <h2>Reportando Bugs</h2>
      <p>Use o template de bug report no GitHub Issues. Inclua:</p>
      <ul>
        <li><strong>Device:</strong> Modelo exato (ex: Pixel 8a, Nothing Phone 2)</li>
        <li><strong>Build:</strong> Versão exata (ex: v1.0.0-stable, lineage_akita-userdebug)</li>
        <li><strong>Steps to reproduce:</strong> Passos mínimos para reproduzir</li>
        <li><strong>Expected vs Actual:</strong> O que esperava vs o que aconteceu</li>
        <li><strong>Logs:</strong> <code>adb logcat</code>, <code>dmesg</code>, <code>last_kmsg</code></li>
        <li><strong>Screenshots/Vídeo:</strong> Se aplicável</li>
      </ul>
      <a href="https://github.com/conexus-mobile-os/conexus-mobile-os/issues/new?template=bug_report.md" target="_blank" rel="noopener" class="btn btn-primary">Abrir Bug Report</a>
    </section>
    
    <section class="contribute-section">
      <h2>Comunidade & Suporte</h2>
      <div class="community-links">
        <a href="https://github.com/conexus-mobile-os/conexus-mobile-os/discussions" target="_blank" rel="noopener" class="community-link">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z"/></svg>
          <span>GitHub Discussions</span>
          <small>Dúvidas, ideias, discussões gerais</small>
        </a>
        <a href="https://matrix.to/#/#conexus-mobile-os:matrix.org" target="_blank" rel="noopener" class="community-link">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4l4 4 4-4h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 12H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z"/></svg>
          <span>Matrix Chat</span>
          <small>Chat em tempo real, suporte rápido</small>
        </a>
        <a href="https://github.com/conexus-mobile-os/conexus-mobile-os/issues" target="_blank" rel="noopener" class="community-link">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg>
          <span>GitHub Issues</span>
          <small>Bug reports, feature requests</small>
        </a>
      </div>
    </section>
    
    <section class="contribute-section">
      <h2>Reconhecimento</h2>
      <p>Todos os contribuidores são listados no <a href="https://github.com/conexus-mobile-os/conexus-mobile-os/graphs/contributors" target="_blank" rel="noopener">GitHub Contributors</a> e no arquivo <code>CONTRIBUTORS.md</code> do repositório.</p>
      <p>Obrigado a todos que tornam o Conexus Mobile OS possível! 🚀</p>
    </section>
  </div>
</div>

<style>
.contribute-section { margin-bottom: var(--spacing-xxxl); }
.contribute-section h2 { padding-bottom: var(--spacing-m); border-bottom: 1px solid var(--color-divider); margin-bottom: var(--spacing-xl); }
.contribute-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: var(--spacing-xl); }
.contribute-card { background: var(--color-surface); border: 1px solid var(--color-divider); padding: var(--spacing-xl); transition: transform var(--transition-normal), box-shadow var(--transition-normal); }
.contribute-card:hover { transform: translateY(-4px); box-shadow: var(--shadow-3); }
.contribute-icon { width: 48px; height: 48px; color: var(--color-accent); margin-bottom: var(--spacing-m); }
.contribute-card h3 { margin: 0 0 var(--spacing-s); font-size: 1.125rem; }
.contribute-card p { color: var(--color-text-secondary); font-size: 0.9375rem; margin-bottom: var(--spacing-m); }
.contribute-section h3 { margin-top: var(--spacing-xl); margin-bottom: var(--spacing-m); }
.test-types { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: var(--spacing-xl); margin-top: var(--spacing-xl); }
.test-card { background: var(--color-surface); border: 1px solid var(--color-divider); padding: var(--spacing-xl); }
.test-card h3 { margin-top: 0; margin-bottom: var(--spacing-m); }
.test-card ul { padding-left: var(--spacing-xl); }
.test-card li { margin: var(--spacing-s) 0; }
.community-links { display: flex; flex-wrap: wrap; gap: var(--spacing-m); margin-top: var(--spacing-m); }
.community-link { display: flex; flex-direction: column; align-items: center; gap: var(--spacing-s); padding: var(--spacing-xl); background: var(--color-surface); border: 1px solid var(--color-divider); border-radius: 0; text-decoration: none; color: var(--color-text); min-width: 180px; transition: transform var(--transition-normal), box-shadow var(--transition-normal); }
.community-link:hover { transform: translateY(-4px); box-shadow: var(--shadow-3); border-color: var(--color-accent); }
.community-link svg { width: 32px; height: 32px; color: var(--color-accent); }
.community-link span { font-weight: 600; font-size: 1rem; }
.community-link small { color: var(--color-text-secondary); font-size: 0.8125rem; }
</style>