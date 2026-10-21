// Conexus Mobile OS — Site JavaScript

// Mobile Navigation Toggle
document.addEventListener('DOMContentLoaded', function() {
  const navToggle = document.querySelector('.nav-toggle');
  const navList = document.querySelector('.nav-list');
  
  if (navToggle && navList) {
    navToggle.addEventListener('click', function() {
      const expanded = this.getAttribute('aria-expanded') === 'true';
      this.setAttribute('aria-expanded', !expanded);
      navList.classList.toggle('active');
    });
    
    // Close menu when clicking a link
    navList.querySelectorAll('a').forEach(link => {
      link.addEventListener('click', () => {
        navList.classList.remove('active');
        navToggle.setAttribute('aria-expanded', 'false');
      });
    });
  }
  
  // Smooth scroll for anchor links
  document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function(e) {
      const targetId = this.getAttribute('href');
      if (targetId === '#') return;
      
      const target = document.querySelector(targetId);
      if (target) {
        e.preventDefault();
        target.scrollIntoView({ behavior: 'smooth', block: 'start' });
        target.focus({ preventScroll: true });
      }
    });
  });
  
  // Download page functionality
  initDownloadPage();
});

// Download Page Logic
function initDownloadPage() {
  const deviceSelect = document.getElementById('device-select');
  const channelSelect = document.getElementById('channel-select');
  
  if (!deviceSelect || !channelSelect) return;
  
  // Load release data from GitHub API
  loadLatestRelease();
  
  deviceSelect.addEventListener('change', loadLatestRelease);
  channelSelect.addEventListener('change', loadLatestRelease);
  
  // Load archived releases
  loadArchivedReleases();
}

async function loadLatestRelease() {
  const deviceSelect = document.getElementById('device-select');
  const channelSelect = document.getElementById('channel-select');
  
  if (!deviceSelect || !channelSelect) return;
  
  const device = deviceSelect.value;
  const channel = channelSelect.value;
  
  try {
    // Try to fetch from GitHub API
    const response = await fetch(`https://api.github.com/repos/conexus-mobile-os/conexus-mobile-os/releases`);
    if (!response.ok) throw new Error('GitHub API failed');
    
    const releases = await response.json();
    
    // Filter by device and channel
    const filtered = releases.filter(r => {
      const tag = r.tag_name || '';
      return tag.includes(device) && tag.includes(channel);
    });
    
    if (filtered.length > 0) {
      const latest = filtered[0];
      updateReleaseUI(latest);
    } else {
      // Fallback to mock data for demo
      showMockRelease(device, channel);
    }
  } catch (error) {
    console.warn('Failed to load releases from GitHub:', error);
    const device = deviceSelect?.value || 'akita';
    const channel = channelSelect?.value || 'stable';
    showMockRelease(device, channel);
  }
}

function showMockRelease(device, channel) {
  const mockReleases = {
    'akita-stable': {
      tag_name: 'v1.0.0-stable',
      name: 'Conexus Mobile OS v1.0.0-stable (Pixel 8a)',
      published_at: '2026-10-21T10:00:00Z',
      body: `## Conexus Mobile OS v1.0.0-stable

### 🎉 First Stable Release

**Conexus Mobile OS v1.0.0-stable** is the first stable release based on LineageOS 23.2 (Android 15 QPR).

### ✨ Features
- **Conexus Launcher v1.0**: Live Tiles flip 3D, pivôs, CommandBar, Metro UI
- **LineageOS 23.2 base**: Android 15 QPR, security patches October 2026
- **OTA A/B**: Updates automáticos, rollback, staged rollout
- **Metro UI**: Selawik font, accent configurável, reduced motion
- **SDK**: Kotlin + TS/RN com TileProvider, NavController, NotificationBroker
- **Security**: AVB v2, SELinux enforcing, FBE, chaves offline

### 📱 Supported Devices
- Google Pixel 8a (akita) — Primary
- Nothing Phone (2) (pong) — Backup

### 🔒 Security
- AVB v2 verified boot
- SELinux enforcing
- File-Based Encryption
- Rollback protection
- Production keys offline (air-gapped)`,
      assets: [
        { name: 'conexus-akita-v1.0.0-stable.zip', size: '2.1 GB', type: 'OTA Package', url: '#' },
        { name: 'boot.img', size: '64 MB', type: 'Boot Image', url: '#' },
        { name: 'dtbo.img', size: '8 MB', type: 'DTBO', url: '#' },
        { name: 'vbmeta.img', size: '4 KB', type: 'VBMeta', url: '#' },
        { name: 'vendor_boot.img', size: '96 MB', type: 'Vendor Boot', url: '#' },
        { name: 'super.img', size: '2.1 GB', type: 'Super Partition', url: '#' },
        { name: 'recovery.img', size: '128 MB', type: 'Recovery', url: '#' },
        { name: 'SHA256SUMS', size: '2 KB', type: 'Checksums', url: '#' },
        { name: 'conexus-akita-v1.0.0-stable.asc', size: '1 KB', type: 'Signature', url: '#' }
      ]
    },
    'akita-monthly': {
      tag_name: 'v1.1.0-monthly',
      name: 'Conexus Mobile OS v1.1.0-monthly (Pixel 8a)',
      published_at: '2026-11-15T10:00:00Z',
      body: 'Monthly release with bug fixes and improvements.',
      assets: []
    },
    'akita-weekly': {
      tag_name: 'v1.1.0-weekly.20261108',
      name: 'Conexus Mobile OS v1.1.0-weekly (Pixel 8a)',
      published_at: '2026-11-08T10:00:00Z',
      body: 'Weekly build for testers.',
      assets: []
    },
    'pong-stable': {
      tag_name: 'v1.0.0-stable',
      name: 'Conexus Mobile OS v1.0.0-stable (Nothing Phone 2)',
      published_at: '2026-10-21T10:00:00Z',
      body: 'First stable release for Nothing Phone (2).',
      assets: []
    }
  };
  
  const key = `${device}-${channel}`;
  const release = mockReleases[key] || mockReleases['akita-stable'];
  updateReleaseUI(release);
}

function updateReleaseUI(release) {
  // Update version
  const versionEl = document.getElementById('release-version');
  if (versionEl) versionEl.textContent = release.tag_name || 'v1.0.0-stable';
  
  // Update date
  const dateEl = document.getElementById('release-date');
  if (dateEl && release.published_at) {
    const date = new Date(release.published_at);
    dateEl.textContent = date.toLocaleDateString('pt-BR', { day: '2-digit', month: 'short', year: 'numeric' });
    dateEl.dateTime = release.published_at;
  }
  
  // Update channel badge
  const channelBadge = document.getElementById('channel-badge');
  if (channelBadge) {
    const channel = release.tag_name?.split('-').pop() || 'stable';
    channelBadge.textContent = channel.charAt(0).toUpperCase() + channel.slice(1);
    channelBadge.className = `channel-badge badge-${channel}`;
  }
  
  // Update changelog
  const changelogEl = document.getElementById('changelog-content');
  if (changelogEl && release.body) {
    // Simple markdown to HTML conversion for basic formatting
    changelogEl.innerHTML = release.body
      .replace(/^### (.+)$/gm, '<h4>$1</h4>')
      .replace(/^\*\*(.+)\*\*:/gm, '<strong>$1:</strong>')
      .replace(/^- (.+)$/gm, '<li>$1</li>')
      .replace(/\n\n/g, '</p><p>')
      .replace(/^(.+)$/gm, '<p>$1</p>')
      .replace(/<p><li>/g, '<ul><li>')
      .replace(/<\/li><\/p>/g, '</li></ul>')
      .replace(/<p><h4>/g, '<h4>')
      .replace(/<\/h4><\/p>/g, '</h4>');
  }
  
  // Update downloads
  const downloadGrid = document.getElementById('download-grid');
  if (downloadGrid && release.assets && release.assets.length > 0) {
    downloadGrid.innerHTML = release.assets.map(asset => `
      <article class="download-card">
        <h4>${asset.name}</h4>
        <div class="meta">${asset.size} • ${asset.type}</div>
        <a href="${asset.url}" class="btn btn-sm ${asset.url === '#' ? 'btn-outline' : 'btn-primary'}" ${asset.url === '#' ? 'onclick="return false;"' : ''}>
          ${asset.url === '#' ? 'Indisponível (demo)' : 'Baixar'}
        </a>
      </article>
    `).join('');
  } else if (downloadGrid) {
    downloadGrid.innerHTML = '<p class="no-assets">Nenhum asset disponível para este release.</p>';
  }
  
  // Update SHA256
  const shaEl = document.getElementById('sha256sums');
  if (shaEl && release.assets) {
    const sums = release.assets
      .filter(a => a.type !== 'Checksums' && a.type !== 'Signature')
      .map(a => `${'a'.repeat(64)}  ${a.name}`)
      .join('\n');
    shaEl.textContent = `# SHA256SUMS\n${sums}\n\n# Signature\nconexus-${release.tag_name}.asc`;
  }
}

async function loadArchivedReleases() {
  const container = document.getElementById('archived-list');
  if (!container) return;
  
  try {
    const response = await fetch('https://api.github.com/repos/conexus-mobile-os/conexus-mobile-os/releases?per_page=20');
    if (!response.ok) throw new Error('Failed');
    
    const releases = await response.json();
    container.innerHTML = releases.map(r => `
      <article class="archived-release">
        <header>
          <h3><a href="${r.html_url}" target="_blank" rel="noopener">${r.name || r.tag_name}</a></header>
          <time datetime="${r.published_at}">${new Date(r.published_at).toLocaleDateString('pt-BR')}</time>
          <p>${r.body?.split('\n')[0] || 'Sem descrição'}</p>
        </article>
      `).join('');
  } catch (error) {
    container.innerHTML = '<p>Não foi possível carregar releases anteriores. <a href="https://github.com/conexus-mobile-os/conexus-mobile-os/releases" target="_blank" rel="noopener">Ver no GitHub</a></p>';
  }
}

// Copy to clipboard utility
function copyToClipboard(text, button) {
  navigator.clipboard.writeText(text).then(() => {
    const original = button.textContent;
    button.textContent = 'Copiado!';
    setTimeout(() => button.textContent = original, 2000);
  });
}

// Theme toggle (if implemented)
function initThemeToggle() {
  const toggle = document.getElementById('theme-toggle');
  if (!toggle) return;
  
  const savedTheme = localStorage.getItem('theme') || 'system';
  applyTheme(savedTheme);
  
  toggle.addEventListener('click', () => {
    const themes = ['light', 'dark', 'system'];
    const current = themes.indexOf(savedTheme);
    const next = themes[(current + 1) % themes.length];
    localStorage.setItem('theme', next);
    applyTheme(next);
  });
}

function applyTheme(theme) {
  const html = document.documentElement;
  if (theme === 'system') {
    html.removeAttribute('data-theme');
  } else {
    html.setAttribute('data-theme', theme);
  }
}

// Smooth scroll polyfill for older browsers
if (!('scrollBehavior' in document.documentElement.style)) {
  const script = document.createElement('script');
  script.src = 'https://cdn.jsdelivr.net/npm/smoothscroll-polyfill@0.4.4/dist/smoothscroll.min.js';
  document.head.appendChild(script);
}

// Performance: Lazy load images
if ('loading' in HTMLImageElement.prototype) {
  document.querySelectorAll('img[loading="lazy"]').forEach(img => {
    img.loading = 'lazy';
  });
} else {
  // Fallback for browsers without native lazy loading
  const script = document.createElement('script');
  script.src = 'https://cdn.jsdelivr.net/npm/lozad@1.16.0/dist/lozad.min.js';
  script.onload = () => {
    const observer = lozad('.lazy-load', { rootMargin: '100px' });
    observer.observe();
  };
  document.head.appendChild(script);
}