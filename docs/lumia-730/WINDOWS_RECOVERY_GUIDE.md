# Lumia 730 / 735 - Windows 10 Mobile Recovery Guide

**Device:** Lumia 730 (RM-914 single SIM) / Lumia 735 (RM-915 dual SIM)  
**Latest Build:** 10.0.15254.603 (Threshold 2 / Redstone 1)  
**Tool:** Windows Device Recovery Tool (WDRT) - Official Microsoft

---

## 🛠 Method 1: Standard Recovery (Device Boots)

### Prerequisites
- Windows 10/11 PC (x64)
- USB cable (original Microsoft cable recommended)
- Internet connection (downloads ~2-3 GB firmware)

### Steps

1. **Download Windows Device Recovery Tool (WDRT)**
   - Official: https://go.microsoft.com/fwlink/?LinkId=708425
   - Or search "Windows Device Recovery Tool" in Microsoft Store

2. **Install WDRT**
   ```cmd
   # Run installer as Administrator
   # Follow installer prompts
   ```

3. **Prepare Phone**
   - Charge battery ≥ 50% (prefer 80%+)
   - Remove SIM card and SD card (optional but recommended)
   - Keep phone ON and unlocked

4. **Connect & Detect**
   - Connect phone via USB to PC
   - WDRT automatically detects "Nokia Lumia 730" or "Nokia Lumia 735"
   - If not detected: try different USB port / cable / restart WDRT

5. **Download & Install**
   - Click "Install" → Downloads ~2-3 GB firmware
   - **Do not disconnect** during download or installation
   - Phone reboots multiple times (gear icons, progress bars)
   - Process takes 15-30 minutes

6. **Complete**
   - Phone reboots to "Welcome" screen
   - Disconnect, setup as new or restore backup

---

## 🔧 Method 2: Recovery Mode (Bricked / Won't Boot)

### Enter Recovery Mode
```bash
# Lumia 730/735 Recovery Mode Entry:
1. Power OFF completely (hold Power 10s if frozen)
2. Press and HOLD: Power + Volume Down (simultaneously)
3. Hold for 10-15 seconds until phone vibrates
4. RELEASE Power button, KEEP holding Volume Down
5. Wait for "gears" animation (⚙️ rotating gears)
5. Release Volume Down
6. Connect USB cable to PC immediately
```

### In Recovery Mode
- Screen shows rotating gears ⚙️
- WDRT detects "Nokia Lumia 730/735 (Recovery Mode)"
- Click "Install" → Same process as Method 1

### If Gears Don't Appear
```bash
# Try alternate timing:
1. Power off completely
2. Press Volume Down FIRST, then Power
2. Hold both until vibration (10s)
3. Release Power, hold Volume Down
3. Wait up to 60 seconds for gears

# If still no gears:
# - Battery may be too low → charge 30 min first
# - Try different USB cable/port
# - Try on different PC
```

---

## 📥 Method 3: Manual Firmware Flash (Advanced)

### Download Firmware Manually
```bash
# Official Microsoft firmware repository (if available):
# https://firmware.nokia.com/ (legacy, may not work)
# Or via: https://forum.xda-developers.com/t/lumia-730-735-firmware.4659858/

# Files needed for RM-914 (730) or RM-915 (735):
# - FFU file (full flash update)
# - Or individual partitions: boot, dtbo, vbmeta, system, vendor, etc.

# Flash via Thor2 / Lumia Flash Tool (requires EDL mode)
# ⚠️ Advanced - can permanently brick device
```

### Enter EDL Mode (Emergency Download)
```bash
# Lumia 730/735 EDL Entry (test points required):
# 1. Remove back cover
# 2. Locate test points on motherboard (near battery connector)
# 3. Short TP1 & TP2 with tweezers while connecting USB
# 3. Device enumerates as "Qualcomm HS-USB QDLoader 9008"
# 3. Use QFIL / QFIL-Linux / EDL tools to flash FFU
#
# ⚠️ REQUIRES: Opening phone, soldering skills, correct test point diagram
# ⚠️ WRONG TEST POINTS = PERMANENT BRICK
```

---

## 🔍 Troubleshooting

| Problem | Solution |
|---------|----------|
| **WDRT doesn't detect phone** | 1. Try different USB port (USB 2.0 preferred) 2. Different cable 3. Restart PC + phone 4. Reinstall WDRT 5. Try different PC |
| **Stuck at "Getting device info"** | Wait 5 min → Restart WDRT → Try different USB port |
| **Stuck at "Downloading" > 30 min** | Check internet → Pause/Resume in WDRT → Restart router |
| **Stuck at "Installing" > 30 min** | Do NOT disconnect → Wait up to 60 min → Force restart only if frozen > 1 hr |
| **Error 0x80070002 / 0x80070003** | Delete `%localappdata%\Packages\Microsoft.WindowsDeviceRecoveryTool_*\LocalCache` → Restart WDRT |
| **Error 0x80070057** | Invalid firmware for device → Check RM-914 vs RM-915 |
| **Phone stuck on gears > 1 hour** | Force restart (Power + Vol Down 15s) → Retry recovery mode |
| **Bootloop after flash** | Recovery mode → Flash again → If persists: EDL mode + stock FFU |

---

## 📋 Pre-Recovery Checklist

- [ ] Backup photos/contacts to OneDrive / PC
- [ ] Remove SIM & SD card
- [ ] Charge battery ≥ 50% (80%+ recommended)
- [ ] Use original Microsoft USB cable
- [ ] Use USB 2.0 port (avoid USB 3.0/USB-C hubs)
- [ ] Disable antivirus/firewall temporarily
- [ ] Run WDRT as Administrator
- [ ] Disable Windows Fast Startup (Control Panel → Power Options)

---

## 📋 Post-Recovery Setup

1. **Initial Setup:** Language → Region → WiFi → Microsoft Account
2. **Updates:** Settings → Update & Security → Check for updates (may need 2-3 rounds)
3. **Apps:** Store → Update all system apps
4. **Backup:** Settings → Backup → Back up now
5. **Optional:** Disable telemetry (Privacy → Diagnostics → Basic)

---

## 📞 Support Resources

| Resource | Link |
|----------|------|
| **Official WDRT Download** | https://go.microsoft.com/fwlink/?LinkId=708425 |
| **WPInternals (Advanced)** | https://github.com/WPInternals/WPInternals |
| **XDA Lumia 730 Forum** | https://xdaforums.com/f/nokia-lumia-730.4529/ |
| **Lumia Firmware DB** | https://firmware.nokia.com/ (legacy) |
| **XDA Recovery Thread** | https://xdaforums.com/t/lumia-730-recovery.4659858/ |

---

## ⚠️ Common Mistakes to Avoid

| ❌ Don't Do This | ✅ Do This Instead |
|------------------|-------------------|
| Unplug during "Installing" | Wait even if seems stuck |
| Use USB 3.0 / USB-C hub | Use USB 2.0 port directly on motherboard |
| Use cheap/third-party cable | Use original Microsoft cable |
| Interrupt "Getting device info" | Wait 5+ minutes |
| Force restart during gears | Wait up to 60 min on gears |
| Flash wrong RM model (914 vs 915) | Verify: Settings → About → Model |

---

## 📋 Device Identification

| Model | RM Code | SIM | Region |
|-------|---------|-----|--------|
| Lumia 730 | RM-914 | Single SIM | Global |
| Lumia 730 DS | RM-914 | Dual SIM | Global |
| Lumia 735 | RM-915 | Single SIM | US/Canada |
| Lumia 735 DS | RM-915 | Dual SIM | Global |

**Verify:** Settings → About → More Info → Model / RM Code

---

## 📞 When to Give Up & Seek Help

- **3+ failed attempts** with different cables/PCs
- **Hardware damage** (water damage, drop damage)
- **EDL mode required** (no test point diagram / no soldering skills)
- **Time cost > device value** (device worth ~R$ 150-250)

**Then:** Sell for parts / recycle → Buy Pixel 3a/4a/6a for LineageOS