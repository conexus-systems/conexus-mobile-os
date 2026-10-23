# Downloads Directory — Coloque Aqui Antes de Começar

```
downloads/
├── WPInternals/
│   └── WPInternals.exe          # Extraído do WPInternals.zip (GitHub Releases)
└── FFU/
    └── RM-914_... .ffu          # OU RM-915_... .ffu (stock firmware do lumiadb.com)
```

---

## 📥 O Que Baixar e Onde Colocar

### 1. WPInternals (Windows — unlock bootloader)
- **Link:** https://github.com/WPInternals/WPInternals/releases/latest
- **Arquivo:** `WPInternals.zip`
- **Ação:** Extraia → copie `WPInternals.exe` para `downloads/WPInternals/`
- **Versão mínima:** 2.8+ (suporta Lumia 730/735)

### 2. FFU Stock Firmware (Windows — donor para unlock + recuperação)
- **Link:** https://lumiadb.com/
- **Search:** `RM-914` (Lumia 730) **OU** `RM-915` (Lumia 735)
- **Selecione:** Modelo exato do seu aparelho (confira em Configurações → Sobre → Modelo)
- **Baixe:** Latest `.ffu` (maior arquivo, ~2-3 GB)
- **Copie:** Para `downloads/FFU/` (não extraia, WPInternals usa .ffu direto)

### 3. WDRT (Windows Device Recovery Tool) — Opcional mas recomendado
- **Link:** https://go.microsoft.com/fwlink/?LinkId=708425
- **Uso:** Recuperar Windows 10 Mobile se brickar
- **Não precisa colocar aqui** — instala no Windows direto

---

## ✅ Checklist Antes de Iniciar

- [ ] `downloads/WPInternals/WPInternals.exe` existe
- [ ] `downloads/FFU/RM-914_... .ffu` **OU** `RM-915_... .ffu` existe
- [ ] Cabo USB original Microsoft conectado
- [ ] Bateria ≥ 80%
- [ ] Backup completo feito (fotos, contatos, 2FA)
- [ ] PC Windows 10/11 nativo (não VM/WSL)

---

## ⚠️ IMPORTANTE

**ESTES ARQUIVOS NÃO SÃO RASTREADOS PELO GIT** (estão no .gitignore).

- WPInternals = binário Windows, copyright Microsoft
- FFU = firmware proprietário Microsoft/Nokia, específico por dispositivo
- **Não commite, não suba para repositório público**

Se perder os arquivos, baixe novamente dos links acima.