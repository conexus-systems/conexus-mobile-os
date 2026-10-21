---
layout: default
title: "Dispositivos Suportados"
description: "Lista de dispositivos suportados pelo Conexus Mobile OS com status detalhado de compatibilidade."
---

<div class="container">
  <header class="page-header">
    <h1>Dispositivos Suportados</h1>
    <p>O Conexus Mobile OS foca em dispositivos com bootloader desbloqueável oficial, kernel source disponível e suporte LineageOS 23.2 ativo.</  </header>
  
  <section class="device-categories">
    <div class="category">
      <h2><span class="category-badge primary">Primário</span> Dispositivo Principal</h2>
      <div class="device-detail">
        <article class="device-detail-card primary">
          <header class="device-header">
            <div class="device-badge primary">Primário</div>
            <h2>Google Pixel 8a</h2>
            <span class="codename">akita</span>
          </header>
          
          <div class="device-grid">
            <div class="device-image-large">
              <svg viewBox="0 0 200 400" aria-hidden="true">
                <rect x="20" y="20" width="160" height="360" rx="30" fill="#1a1a2e" stroke="#333" stroke-width="3"/>
                <rect x="30" y="40" width="140" height="300" rx="15" fill="#0d0d1a"/>
                <rect x="70" y="340" width="60" height="8" rx="4" fill="#333"/>
              </svg>
            </div>
            
            <div class="device-specs-detailed">
              <h3>Especificações</h3>
              <dl class="specs-list">
                <div><dt>SoC</dt><dd>Google Tensor G3 (GS101)</dd></div>
                <div><dt>CPU</dt><dd>1×3.0 GHz Cortex-X3 + 4×2.45 GHz Cortex-A715 + 4×2.14 GHz Cortex-A510</dd></div>
                <div><dt>GPU</dt><dd>ARM Mali-G715s MC10</dd></div>
                <div><dt>RAM</dt><dd>8 GB LPDDR5X</dd></div>
                <div><dt>Storage</dt><dd>128/256 GB UFS 3.1</dd></div>
                <div><dt>Display</dt><dd>6.1" 1080x2400 OLED, 120 Hz, 431 PPI</dd></div>
                <div><dt>Battery</dt><dd>4492 mAh</dd></div>
                <div><dt>Camera</dt><dd>64 MP (wide) + 13 MP (ultrawide) • Frontal 13 MP</dd></div>
              </dl>
              
              <h3>Suporte de Software</h3>
              <ul class="support-list">
                <li><strong>Bootloader:</strong> Oficial (`fastboot flashing unlock`) — sem wait, sem conta</li>
                <li><strong>Kernel:</strong> 6.1 GKI (`android-gs-akita-6.1-android16`) — mainline upstream</li>
                <li><strong>LineageOS:</strong> 23.2 oficial, builds semanais, mantenedor `mikeioannina`</li>
                <li><strong>Partitioning:</strong> A/B + Dynamic Partitions (`super`)</li>
                <li><strong>Recovery:</strong> LineageOS Recovery oficial</li>
                <li><strong>Vendor Blobs:</strong> `android_vendor_google_akita` (LineageOS)</li>
              </ul>
              
              <div class="device-actions">
                <a href="{{ '/download' | relative_url }}" class="btn btn-primary">Baixar para Pixel 8a</a>
                <a href="https://wiki.lineageos.org/devices/akita" target="_blank" rel="noopener" class="btn btn-outline">LineageOS Wiki</a>
              </div>
            </div>
          </div>
        </article>
      </div>
      
      <div class="category">
        <h2><span class="category-badge backup">Backup</span> Dispositivo Alternativo</h2>
        <div class="device-detail">
          <article class="device-detail-card backup">
            <header class="device-header">
              <div class="device-badge backup">Backup</div>
              <h2>Nothing Phone (2)</h2>
              <span class="codename">pong</span>
            </header>
            
            <div class="device-grid">
              <div class="device-image-large">
                <svg viewBox="0 0 200 400" aria-hidden="true">
                  <rect x="20" y="20" width="160" height="360" rx="30" fill="#1a1a2e" stroke="#333" stroke-width="3"/>
                  <rect x="30" y="40" width="140" height="300" rx="15" fill="#0d0d1a"/>
                  <rect x="70" y="340" width="60" height="8" rx="4" fill="#333"/>
                </svg>
              </svg>
            </div>
            
            <div class="device-specs-detailed">
              <h3>Especificações</h3>
              <dl class="specs-list">
                <div><dt>SoC</dt><dd>Qualcomm Snapdragon 8+ Gen 1 (SM8475)</dd></div>
                <div><dt>CPU</dt><dd>1×3.0 GHz Cortex-X2 + 3×2.5 GHz Cortex-A710 + 4×1.8 GHz Cortex-A510</dd></div>
                <div><dt>GPU</dt><dd>Adreno 730</dd></div>
                <div><dt>RAM</dt><dd>8/12 GB LPDDR5</dd></div>
                <div><dt>Storage</dt><dd>128/256/512 GB UFS 3.1</dd></div>
                <div><dt>Display</dt><dd>6.7" 1080x2412 OLED, 120 Hz LTPO</dd></div>
                <div><dt>Battery</dt><dd>4700 mAh</dd></div>
                <div><dt>Camera</dt><dd>50 MP (wide) + 50 MP (ultrawide) • Frontal 32 MP</dd></div>
              </dl>
              
              <h3>Suporte de Software</h3>
              <ul class="support-list">
                <li><strong>Bootloader:</strong> Oficial (`fastboot oem unlock`) — fácil, sem wait</li>
                <li><strong>Kernel Source:</strong> Liberado pela Nothing (3 dias pós-launch) — GitHub NothingOSS</li>
                <li><strong>LineageOS:</strong> 23.2 unofficial — builds community</li>
                <li><strong>Partitioning:</strong> A/B + Dynamic Partitions</li>
                <li><strong>Recovery:</strong> LineageOS 23.2 + TWRP</li>
                <li><strong>Vendor Blobs:</strong> Extraídos de firmware stock (TheMuppets)</li>
              </ul>
              
              <div class="device-actions">
                <a href="{{ '/download' | relative_url }}" class="btn btn-primary">Baixar para Nothing Phone (2)</a>
                <a href="https://github.com/NothingOSS" target="_blank" rel="noopener" class="btn btn-outline">NothingOSS GitHub</a>
              </div>
            </div>
          </div>
        </article>
      </div>
      
      <div class="category">
        <h2><span class="category-badge tertiary">Terceira Opção</span> Viável com Ressalvas</h2>
        <div class="device-cards">
          <article class="device-card">
            <div class="device-badge tertiary">3ª Opção</div>
            <h3>OnePlus 12</h3>
            <span class="codename">waffle</span>
            <ul>
              <li>Snapdragon 8 Gen 3</li>
              <li>12/16/24 GB RAM</li>
              <li>LineageOS 22.2 unofficial</li>
              <li>⚠️ Variants T-Mobile: OEM unlock desabilitado</li>
              <li>⚠️ $700+ • ColorOS/OxygenOS unified base</li>
            </ul>
          </article>
        </div>
      </div>
      
      <div class="category">
        <h2><span class="category-badge rejected">Rejeitado</span> Não Viável</h2>
        <div class="device-cards">
          <article class="device-card rejected">
            <div class="device-badge rejected">Rejeitado</div>
            <h3>Xiaomi 14</h3>
            <span class="codename">houji</span>
            <ul>
              <li>Bootloader unlock: wait 7-168h + limite 1 device/ano/conta</li>
              <li>LineageOS: thread fechada no XDA (beta/closed)</li>
              <li>Política anti-desenvolvedor</li>
            </ul>
          </article>
        </div>
      </div>
      
      <div class="category">
        <h2><span class="category-badge research">Pesquisa</span> Lumia (Não-Produto)</h2>
        <div class="device-cards">
          <article class="device-card research">
            <div class="device-badge research">Pesquisa</div>
            <h3>Lumia 950 XL</h3>
            <span class="codename">RM-1116</span>
            <ul>
              <li>Snapdragon 810 • 3 GB RAM • 32 GB</li>
              <li>UEFI: Working (Lumia950XLPkg)</li>
              <li>Linux Boot: Partial (display, touch, storage)</li>
              <li>❌ Modem, Camera, Audio broken</li>
              <li>⚠️ Sem telefonia/emergência confiável</li>
            </ul>
            <p class="research-note">Trilha paralela M5 — Go/No-go para imagem dedicada após M4.</p>
          </article>
        </div>
      </div>
    </div>
  </section>
  
  <section class="porting-guide">
    <div class="container">
      <h2>Quer portar para outro dispositivo?</h2>
      <p>O Conexus Mobile OS aceita ports da comunidade. Requisitos mínimos:</p>
      <ul class="porting-requirements">
        <li>Bootloader desbloqueável oficialmente (sem wait times excessivos)</li>
        <li>Kernel source GPL disponível (GitHub/GitLab)</li>
        <li>LineageOS 23.2 oficial ou unofficial estável</li>
        <li>Device tree público e mantido</li>
        <li>Partition scheme A/B + Dynamic Partitions (preferível)</li>
        <li>Recovery funcional (LineageOS Recovery ou TWRP)</li>
      </ul>
      <a href="{{ '/docs/porting' | relative_url }}" class="btn btn-primary">Guia de Porting Completo</a>
    </div>
  </section>
</div>