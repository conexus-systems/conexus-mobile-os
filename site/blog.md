---
layout: default
title: "Blog & Releases"
description: "Notícias, anúncios de release, changelogs e atualizações do projeto Conexus Mobile OS."
---

<div class="container">
  <header class="page-header">
    <h1>Blog & Releases</h2>
    <p>Anúncios oficiais, changelogs detalhados e atualizações do projeto.</p>
  </header>
  
  <div class="blog-filters">
    <button class="filter-btn active" data-filter="all">Todos</button>
    <button class="filter-btn" data-filter="release">Releases</button>
    <button class="filter-btn" data-filter="announcement">Anúncios</button>
    <button class="filter-btn" data-filter="technical">Técnico</button>
  </div>
  
  <div class="blog-grid" id="blog-posts">
    <article class="blog-post release" data-category="release">
      <header class="post-header">
        <span class="post-category release">Release</span>
        <time datetime="2026-10-21">21 Out 2026</time>
      </header>
      <h2><a href="#">Conexus Mobile OS v1.0.0-stable — First Stable Release</a></h2>
      <p class="post-excerpt">First stable release based on LineageOS 23.2 (Android 15 QPR). Conexus Launcher v1.0 with Live Tiles, Metro UI, OTA A/B, Metro UI design system, and production-grade security.</p>
      <footer class="post-footer">
        <span class="post-tag stable">Stable</span>
        <a href="#" class="read-more">Ler mais →</a>
      </footer>
    </article>
    
    <article class="blog-post announcement" data-category="announcement">
      <header class="post-header">
        <span class="post-category announcement">Anúncio</span>
        <time datetime="2026-10-19">19 Out 2026</time>
      </header>
      <h2><a href="#">Conexus Mobile OS Project Announced</a></h2>
      <p class="post-excerpt">Announcing the Conexus Mobile OS project: an open-source mobile OS based on LineageOS 23.2, bringing the Windows 10 Mobile experience to modern devices with Conexus Launcher as default.</p>
      <footer class="post-footer">
        <span class="post-tag announcement">Anúncio</span>
        <a href="#" class="read-more">Ler mais →</a>
      </footer>
    </article>
    
    <article class="blog-post technical" data-category="technical">
      <header class="post-header">
        <span class="post-category technical">Técnico</span>
        <time datetime="2026-10-18">18 Out 2026</time>
      </header>
      <h2><a href="#">Architecture Deep Dive: Conexus System Services</a></h2>
      <p class="post-excerpt">Deep dive into the Conexus System Services architecture: Tile Engine, Notification Broker, Nav Controller, and the IPC abstraction layer (Binder/D-Bus).</p>
      <footer class="post-footer">
        <span class="post-tag technical">Técnico</span>
        <a href="#" class="read-more">Ler mais →</a>
      </footer>
    </article>
    
    <article class="blog-post technical" data-category="technical">
      <header class="post-header">
        <span class="post-category technical">Técnico</span>
        <time datetime="2026-10-17">17 Out 2026</time>
      </header>
      <h2><a href="#">Metro/W10M Design System: Implementation Details</a></h2>
      <p class="post-excerpt">Deep dive into the Metro/W10M design system tokens: Selawik typography, accent colors, spacing, motion, elevation, and component specifications.</p>
      <footer class="post-footer">
        <span class="post-tag technical">Técnico</span>
        <a href="#" class="read-more">Ler mais →</a>
      </footer>
    </article>
    
    <article class="blog-post release" data-category="release">
      <header class="post-header">
        <span class="post-category release">Release</span>
        <time datetime="2026-10-15">15 Out 2026</time>
      </header>
      <h2><a href="#">Conexus Mobile OS v0.9.0-monthly — Monthly Release</a></h2>
      <p class="post-excerpt">Monthly release candidate with bug fixes, performance improvements, and new Metro UI components.</      </article>
    </article>
    
    <article class="blog-post technical" data-category="technical">
      <header class="post-header">
        <span class="post-category technical">Técnico</span>
        <time datetime="2026-10-14">14 Out 2026</time>
      </header>
      <h2><a href="#">Building Conexus Mobile OS for Pixel 8a</a></h2>
      <p class="post-excerpt">Step-by-step guide to building Conexus Mobile OS from source for Google Pixel 8a (akita) using LineageOS 23.2 as base.</      </article>
    </article>
  </div>
  
  <nav class="pagination" aria-label="Paginação">
    <a href="#" class="page-link prev" aria-label="Página anterior">← Anterior</a>
    <span class="page-info">Página 1 de 3</span>
    <a href="#" class="page-link next" aria-label="Próxima página">Próxima →</a>
  </nav>
</div>

<style>
.blog-filters { display: flex; flex-wrap: wrap; gap: var(--spacing-s); margin-bottom: var(--spacing-xl); }
.filter-btn { padding: var(--spacing-s) var(--spacing-m); background: var(--color-surface); border: 1px solid var(--color-divider); border-radius: 0; font: inherit; cursor: pointer; transition: all var(--transition-fast); }
.filter-btn:hover, .filter-btn.active { border-color: var(--color-accent); color: var(--color-accent); background: var(--color-accent); color: var(--color-on-accent); }
.blog-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: var(--spacing-xl); }
.blog-post { background: var(--color-surface); border: 1px solid var(--color-divider); padding: var(--spacing-xl); display: flex; flex-direction: column; transition: transform var(--transition-normal), box-shadow var(--transition-normal); }
.blog-post:hover { transform: translateY(-4px); box-shadow: var(--shadow-3); }
.post-header { display: flex; align-items: center; gap: var(--spacing-m); margin-bottom: var(--spacing-m); flex-wrap: wrap; }
.post-category { padding: 2px 8px; font-size: 0.625rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; border-radius: 0; }
.post-category.release { background: var(--color-success); color: white; }
.post-category.announcement { background: var(--color-accent); color: var(--color-on-accent); }
.post-category.technical { background: var(--color-warning); color: var(--color-text); }
.post-header time { color: var(--color-text-secondary); font-size: 0.8125rem; }
.blog-post h2 { margin: 0 0 var(--spacing-m); font-size: 1.25rem; }
.blog-post h2 a { color: var(--color-text); transition: color var(--transition-fast); }
.blog-post h2 a:hover { color: var(--color-accent); }
.post-excerpt { color: var(--color-text-secondary); line-height: 1.7; flex: 1; margin-bottom: var(--spacing-l); }
.post-footer { display: flex; justify-content: space-between; align-items: center; padding-top: var(--spacing-l); border-top: 1px solid var(--color-divider); }
.post-tag { padding: 2px 8px; font-size: 0.625rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; }
.post-tag.stable { background: var(--color-success); color: white; }
.post-tag.release { background: var(--color-success); color: white; }
.post-tag.announcement { background: var(--color-accent); color: var(--color-on-accent); }
.post-tag.technical { background: var(--color-warning); color: var(--color-text); }
.read-more { color: var(--color-accent); font-weight: 500; font-size: 0.875rem; }
.read-more:hover { text-decoration: underline; }
.pagination { display: flex; justify-content: center; align-items: center; gap: var(--spacing-m); margin-top: var(--spacing-xl); }
.page-link { padding: var(--spacing-s) var(--spacing-m); background: var(--color-surface); border: 1px solid var(--color-divider); border-radius: 0; color: var(--color-text); transition: all var(--transition-fast); }
.page-link:hover { border-color: var(--color-accent); color: var(--color-accent); }
.page-info { color: var(--color-text-secondary); font-size: 0.875rem; }
</style>