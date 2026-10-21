---
layout: default
title: "Download"
description: "Baixe o Conexus Mobile OS para seu dispositivo. Releases oficiais, OTA metadata, factory images e checksums."
---

<div class="container">
  <header class="page-header">
    <h1>Download Conexus Mobile OS</h1>
    <p>Escolha seu dispositivo e canal de release. Todos os arquivos são assinados (AVB v2/minisign) com checksums SHA-256.</p>
  </header>
  
  <section class="release-selector">
    <div class="selector-grid">
      <div class="selector-card">
        <h3>Seu Dispositivo</h3>
        <select id="device-select" aria-label="Selecione seu dispositivo">
          <option value="akita" selected>Google Pixel 8a (akita) — Primário</option>
          <option value="pong">Nothing Phone (2) (pong) — Backup</option>
        </select>
      </div>
      <div class="selector-card">
        <h3>Canal de Release</h3>
        <select id="channel-select" aria-label="Selecione o canal">
          <option value="stable" selected>Stable (Trimestral) — Recomendado</option>
          <option value="monthly">Monthly — Stable Candidate</option>
          <option value="weekly">Weekly — Testers</option>
          <option value="nightly">Nightly — Desenvolvedores</option>
        </select>
      </div>
    </div>
  </section>
  
  <section class="latest-release" id="latest-release">
    <header class="section-header">
      <h2>Latest Release — <span id="release-version">v1.0.0-stable</span></h2>
      <div class="release-meta">
        <span class="badge badge-stable">Stable</span>
        <time id="release-date" datetime="2026-10-21">21 Out 2026</time>
        <span class="channel-badge" id="channel-badge">Stable</span>
      </div>
    </header>
    
    <div class="release-info">
      <div class="release-changelog">
        <h3>O que há de novo</h3>
        <div id="changelog-content">
          <ul>
            <li><strong>Conexus Launcher v1.0:</strong> Live Tiles flip 3D, pivôs, CommandBar, Metro UI</li>
            <li><strong>LineageOS 23.2 base:</strong> Android 15 QPR, security patches October 2026</li>
            <li><strong>OTA A/B:</strong> Updates automáticos, rollback, staged rollout</li>
            <li><strong>Metro UI:</strong> Selawik font, accent configurável, reduced motion</li>
            <li><strong>SDK:</strong> Kotlin + TS/RN com TileProvider, NavController, NotificationBroker</li>
            <li><strong>Security:</strong> AVB v2, SELinux enforcing, FBE, chaves offline</li>
          </ul>
        </div>
      </div>
      
      <div class="release-downloads">
        <h3>Downloads</h3>
        <div class="download-grid" id="download-grid">
          <!-- Populated by JS -->
        </div>
      </div>
      
      <div class="release-verify">
        <h3>Verificação</h3>
        <pre id="sha256sums"># SHA256SUMS
# Será preenchido dinamicamente</pre>
        <p><a href="#" id="minisign-link" class="btn btn-sm btn-outline">Verificar assinatura minisign</a></p>
      </div>
    </div>
  </section>
  
  <section class="installation-guide">
    <div class="container">
      <h2>Guia de Instalação Rápida</h2>
      <div class="install-steps">
        <article class="step">
          <div class="step-number">1</div>
          <div class="step-content">
            <h3>Pré-requisitos</h3>
            <ul>
              <li>Device suportado (Pixel 8a / Nothing Phone 2)</li>
              <li>PC com Linux/macOS/Windows + USB-C</li>
              <li>Backup dos dados (unlock apaga tudo)</li>
            </ul>
          </div>
        </article>
        <article class="step">
          <div class="step-number">2</div>
          <div class="step-content">
            <h3>Desbloquear Bootloader</h3>
            <pre><code># Pixel 8a
adb reboot bootloader
fastboot flashing unlock
# Confirmar no device (Volume + Power)

# Nothing Phone (2)
adb reboot bootloader
fastboot oem unlock</code></pre>
          </div>
        </article>
        <article class="step">
          <div class="step-number">3</div>
          <div class="step-content">
            <h3>Flash Conexus OS</h3>
            <pre><code>cd ~/conexus-akita-<version>
fastboot flash boot boot.img
fastboot flash dtbo dtbo.img
fastboot flash vbmeta vbmeta.img
fastboot flash vendor_boot vendor_boot.img
fastboot flash super super.img
fastboot flash recovery recovery.img
fastboot reboot</code></pre>
          </div>
        </article>
        <article class="step">
          <div class="step-number">4</div>
          <div class="step-content">
            <h3>Primeiro Boot & OTA</h3>
            <ul>
              <li>Primeiro boot leva ~2-3 min (otimização ART)</li>
              <li>Configurar Wi-Fi, conta Google (opcional)</li>
              <li>Settings → Sistema → Atualização → Verificar</li>
              <li>Canal padrão: Stable (alterável em Settings)</li>
            </ul>
          </div>
        </article>
      </div>
      
      <div class="install-links">
        <a href="{{ '/docs/install' | relative_url }}" class="btn btn-primary">Guia Completo de Instalação</a>
        <a href="{{ '/docs/troubleshooting' | relative_url }}" class="btn btn-outline">Troubleshooting</a>
      </div>
    </div>
  </section>
  
  <section class="ota-section">
    <div class="container">
      <h2>OTA (Over-The-Air) Updates</h2>
      <div class="ota-grid">
        <article class="ota-card">
          <h3>Como funciona</h3>
          <p>Updates A/B automáticos via <code>conexus-update</code> service. Download em background, aplica no slot inativo, swap no reboot.</p>
        </article>
        <article class="ota-card">
          <h3>Canais</h3>
          <ul>
            <li><strong>Stable:</strong> Trimestral, 100% rollout</li>
            <li><strong>Monthly:</strong> Mensal, 25% → 100%</li>
            <li><strong>Weekly:</strong> Semanal, 5% → 100%</li>
            <li><strong>Nightly:</strong> Diária, opt-in apenas</li>
          </ul>
        </article>
        <article class="ota-card">
          <h3>Rollback</h3>
          <p>A/B: automático se boot falhar. Manual: Settings → Sistema → Atualização → Rollback. Recovery: flash slot anterior via fastboot.</p>
        </article>
      </div>
      
      <div class="ota-metadata">
        <h3>OTA Metadata Endpoint</h3>
        <p>Cliente <code>conexus-update</code> consulta:</p>
        <code>https://conexus-mobile-os.github.io/ota/ota_metadata.json</code>
        <p><a href="{{ '/ota/ota_metadata.json' | relative_url }}" target="_blank" class="btn btn-sm btn-outline">Ver Metadata JSON</a></p>
      </div>
    </div>
  </section>
  
  <section class="factory-images">
    <div class="container">
      <h2>Factory Images (Restauração Stock)</h2>
      <p>Sempre teste a restauração stock <strong>antes</strong> de flashear Conexus OS.</p>
      <div class="factory-links">
        <a href="https://flash.android.com/" target="_blank" rel="noopener" class="btn btn-primary">Android Flash Tool (Oficial)</a>
        <a href="https://developers.google.com/android/images" target="_blank" rel="noopener" class="btn btn-outline">Google Factory Images</a>
        <a href="https://github.com/NothingOSS" target="_blank" rel="noopener" class="btn btn-outline">Nothing Firmware</a>
      </div>
      <details>
        <summary>Hashes SHA-256 (Pixel 8a - android-15.0.0_r1)</summary>
        <pre><code># Exemplo - substituir pelo build atual
boot.img:          a1b2c3d4e5f6...
dtbo.img:          f6e5d4c3b2a1...
vbmeta.img:        1a2b3c4d5e6f...
vendor_boot.img:   9f8e7d6c5b4a...
super.img:         5a6b7c8d9e0f...
recovery.img:      0f1e2d3c4b5a...</code></pre>
      </details>
    </div>
  </section>
  
  <section class="archived-releases">
    <div class="container">
      <h2>Releases Anteriores</h2>
      <p><a href="https://github.com/conexus-mobile-os/conexus-mobile-os/releases" target="_blank" rel="noopener" class="btn btn-link">Ver todos no GitHub Releases →</a></p>
      <div id="archived-list">
        <!-- Populated by JS from GitHub API -->
      </div>
    </div>
  </section>
</div>