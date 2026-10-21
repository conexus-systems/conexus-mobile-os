# Lumia 730 - Technical Assessment for Conexus Mobile OS

**Device:** Lumia 730 / 735 (RM-914 / RM-915)  
**Codename:** `eagle` (shared with 735)  
**Status:** ❌ **NOT SUPPORTED** for Conexus Mobile OS / Android Ports

---

## 📋 Hardware Specifications

| Component | Specification | Conexus Requirement | Status |
|-----------|---------------|---------------------|--------|
| **SoC** | Snapdragon 400 (MSM8226) - 4× Cortex-A7 @ 1.2 GHz | ARMv8 (64-bit) | ❌ **32-bit only (ARMv7)** |
| **GPU** | Adreno 305 | Vulkan 1.1+ / OpenGL ES 3.1+ | ❌ OpenGL ES 3.0 only |
| **RAM** | 1 GB LPDDR3 | ≥ 2 GB (4 GB recommended) | ❌ **Insufficient** |
| **Storage** | 8 GB eMMC (≈4 GB user) | ≥ 16 GB (32 GB recommended) | ❌ **Insufficient** |
| **Display** | 4.7" 720p OLED | 720p+ supported | ✅ OK |
| **Architecture** | ARMv7-A (32-bit) | ARMv8-A (64-bit) required | ❌ **32-bit only** |
| **Bootloader** | Locked (Microsoft) | Unlockable required | ❌ **Permanently Locked** |
| **Kernel Source** | None public | Required | ❌ **Unavailable** |

---

## ❌ Why Conexus Mobile OS Cannot Run on Lumia 730

### 1. **Architecture Incompatibility**
- **Lumia 730:** ARMv7-A (32-bit, Cortex-A7)
- **Conexus Minimum:** ARMv8-A (64-bit, ARMv8-A)
- **Impact:** Modern Android 14/15 + Conexus requires ARMv8-A (64-bit). 32-bit support dropped in Android 12+.

### 2. **Bootloader Permanently Locked**
- Microsoft **never released** bootloader unlock for Lumia 730/735
- No `fastboot oem unlock` support
- No EDK2/UEFI port exists (unlike Lumia 950/XL which have WOA project)

### 3. **No Kernel Sources**
- No public kernel source for MSM8226 (Snapdragon 400)
- No device tree sources
- No vendor blobs, firmware, or HAL implementations
- No camera, WiFi, Bluetooth, modem, GPS, sensor drivers

### 4. **Hardware Below Minimum Requirements**
| Resource | Lumia 730 | Conexus Minimum | Gap |
|----------|-----------|-----------------|-----|
| **RAM** | 1 GB | 2 GB (4 GB rec) | **50% deficit** |
| **Storage** | 8 GB (4 GB free) | 16 GB min | **75% deficit** |
| **API Level** | Max Android 10 (theoretical) | API 34 (Android 14) | 4 versions behind |

### 5. **No Upstream Support**
- Zero LineageOS/AOSP device trees
- Zero kernel sources on GitHub/Gerrit
- Zero vendor blobs
- Zero active maintainers
- No TWRP, no custom recovery

---

## ✅ What CAN Be Installed on Lumia 730

| Option | Status | Notes |
|--------|--------|-------|
| **Windows 10 Mobile** (15254.603) | ✅ Official | Already installed or recoverable via WDRT |
| **Windows 10 IoT Core** | ⚠️ Difficult | FFU images removed by Microsoft; drivers incomplete |
| **Windows Phone 8.1** | ✅ Official | Via WDRT; no modern app support |
| **Android / LineageOS / Custom ROMs** | ❌ **Impossible** | Bootloader locked, no kernel source |

---

## 🔧 Recovery Options (Windows Only)

### Official Microsoft Recovery (Recommended)
```bash
# 1. Download Windows Device Recovery Tool (WDRT)
# https://go.microsoft.com/fwlink/?LinkId=708425

# 2. Connect Lumia 730 via USB
# 3. Run WDRT → Detects device → Downloads latest firmware (15254.603) → Flashes

# If device won't boot (bricked):
# 1. Power off → Power + Volume Down (10s)
# 2. Release Power, hold Volume Down → gears appear
# 4. Connect USB → WDRT detects "Nokia Lumia 730" in recovery mode
```

### Emergency Recovery (If WDRT Fails)
```bash
# Advanced tools (use at own risk):
# - WPInternals: https://github.com/WPInternals/WPInternals/releases
# - Lumia Flash Tool (unofficial)
# - Thor2 / Thor2 Lite (EDL mode via test points)
# 
# ⚠️ Risk: Permanent brick if done incorrectly
```

---

## 📱 Lumia 730 Still Works For (2024)

| ✅ Works Well | ❌ Broken/Unusable |
|---------------|-------------------|
| Calls / SMS / MMS | WhatsApp (discontinued 2020) |
| Edge Browser (basic) | Instagram / TikTok / Modern apps |
| Here Maps / Here WeGo (offline GPS) | Banking apps / PIX |
| Email (IMAP/Exchange/Outlook) | Banking apps / Banking apps |
| Calendar / Contacts (CalDAV/CardDAV) | Uber / iFood / Rappi |
| Camera (5 MP + Zeiss optics) | Netflix / Spotify / YouTube app |
| Bluetooth audio / File transfer | Modern games |
| FM Radio | Banking apps |

**Best use case 2024:** Dedicated **offline GPS navigator** + **camera** + **MP3 player** + **SMS/Call backup phone**

---

## 💰 Economic Reality

| Option | Cost | Outcome |
|--------|------|---------|
| **Keep Lumia 730** | Free | GPS/Camera/MP3 backup phone |
| **Sell Lumia 730** | +R$ 150-250 | Funds toward supported device |
| **Buy Pixel 3a (used)** | ~R$ 300 | LineageOS 21, 4 GB RAM, 64 GB |
| **Buy Pixel 4a (used)** | ~R$ 400 | LineageOS 21, 6 GB RAM |
| **Buy Pixel 6a (used)** | ~R$ 600 | LineageOS 21, Tensor, 6 GB |

**Recommendation:** Sell Lumia 730 (R$ 150-250) + add R$ 150 = **Pixel 3a with LineageOS 21** (4 GB RAM, 64 GB, 64-bit, unlocked bootloader, official LineageOS support).

---

## 📁 Files in This Repo for Lumia 730

```
docs/
└── lumia-730/
    ├── TECHNICAL_ASSESSMENT.md     # This file
    ├── WINDOWS_RECOVERY_GUIDE.md   # Step-by-step Windows recovery
    └── DEVICE_SPECS.md             # Detailed hardware specs
```

---

## 📚 References

- **WPInternals:** https://github.com/WPInternals/WPInternals (advanced Windows Phone hacking)
- **WOA Project:** https://github.com/WOA-Project (Windows on ARM - only for 950/950 XL)
- **LineageOS Device List:** https://wiki.lineageos.org/devices/ (Lumia 730 NOT listed)
- **Microsoft WDRT:** https://go.microsoft.com/fwlink/?LinkId=708425

---

**Conclusion:** Lumia 730 is **not viable** for Conexus Mobile OS or any modern Android port. Use it as a dedicated offline GPS/camera/MP3 device, or sell it toward a supported device (Pixel 3a/4a/6a).