# Lumia 730 / 735 - Complete Device Specifications

**Model:** Lumia 730 (RM-914) / Lumia 735 (RM-915)  
**Codename:** `eagle` (shared)  
**Release:** September 2014  
**OS:** Windows Phone 8.1 → Windows 10 Mobile (15254.603)  
**Status:** End of Life (EOL) - No official support since 2020

---

## 📱 Hardware Specifications

### **Processor & Memory**
| Component | Specification |
|-----------|---------------|
| **SoC** | Qualcomm Snapdragon 400 (MSM8226) |
| **CPU** | 4× Cortex-A7 @ 1.2 GHz (ARMv7-A, 32-bit) |
| **GPU** | Adreno 305 @ 400 MHz |
| **RAM** | 1 GB LPDDR3 (533 MHz) |
| **Storage** | 8 GB eMMC 4.5 (~4.5 GB user available) |
| **Storage Type** | eMMC 4.5 (no UFS) |

### **Display**
| Property | Specification |
|----------|---------------|
| **Size** | 4.7 inches |
| **Resolution** | 1280 × 720 (720p) |
| **Density** | 316 PPI |
| **Type** | OLED (AMOLED), ClearBlack |
| **Aspect Ratio** | 16:9 |
| **Protection** | Gorilla Glass 3 |
| **Touch** | Capacitive, multi-touch (10-point), super-sensitive touch |

### **Camera**
| Component | Specification |
|-----------|---------------|
| **Rear** | 6.7 MP (Carl Zeiss optics, f/1.9, 28mm, 1/3.06") |
| **Features** | Autofocus, LED flash, 1080p@30fps video, living images |
| **Front** | 5 MP (f/2.4, 22mm), 1080p@30fps video |
| **Features** | Wide-angle selfie, auto-HDR, panorama |

### **Connectivity**
| Feature | Specification |
|---------|---------------|
| **2G** | GSM 850/900/1800/1900 |
| **3G** | HSDPA 850/900/1900/2100 (42 Mbps) |
| **4G** | LTE Cat 4 (150/50 Mbps) - Bands 1,2,3,4,5,7,8,20 |
| **WiFi** | 802.11 a/b/g/n (2.4/5 GHz), WiFi Direct, Hotspot |
| **Bluetooth** | 4.0 LE (A2DP, AVRCP, HFP, HSP, MAP, PAN, PBAP) |
| **NFC** | Yes (with secure element) |
| **GPS** | A-GPS, GLONASS, BeiDou |
| **USB** | microUSB 2.0 (OTG supported) |
| **Sensors** | Accelerometer, Gyroscope, Proximity, Light, Compass, Magnetometer |

### **Audio**
| Feature | Specification |
|---------|---------------|
| **Speakers** | Mono (bottom), Dolby Audio |
| **Microphones** | Dual (noise cancellation) |
| **Headphone Jack** | 3.5 mm (CTIA) |
| **Audio Formats** | MP3, AAC, WMA, WAV, FLAC, AMR |
| **Video Formats** | MP4, H.264, H.263, WMV |

### **Battery & Power**
| Specification | Value |
|---------------|-------|
| **Capacity** | 2200 mAh (user-replaceable) |
| **Type** | Li-Ion (BP-4G) |
| **Talk Time** | Up to 16h (3G) |
| **Standby** | Up to 480h (3G) |
| **Music Playback** | Up to 67h |
| **Charging** | 5V/1.5A (7.5W) via microUSB |
| **Wireless Charging** | Qi (built-in, Lumia 735 only) |

### **Physical**
| Property | Lumia 730 | Lumia 735 |
|----------|-----------|-----------|
| **Dimensions** | 134.7 × 68.5 × 8.7 mm | 134.7 × 68.5 × 8.7 mm |
| **Weight** | 130.4 g | 134 g |
| **SIM** | Micro-SIM (single/dual) | Micro-SIM (single/dual) |
| **Colors** | Orange, Green, White, Black | Orange, Green, White, Black |
| **Body** | Polycarbonate unibody | Polycarbonate unibody |
| **Wireless Charging** | ❌ | ✅ (Qi) |

---

## 🔧 Software & Firmware

### **Official Firmware History**
| OS Version | Build | Release Date | Notes |
|------------|-------|--------------|-------|
| Windows Phone 8.1 | 8.10.14219.341 | Sep 2014 | Launch OS |
| Windows 10 Mobile | 10.0.10586.x | Nov 2015 | Threshold 1 |
| Windows 10 Mobile | 10.0.14393.x | Aug 2016 | Anniversary (Redstone 1) |
| Windows 10 Mobile | 10.0.15063.x | Apr 2017 | Creators (Redstone 2) |
| Windows 10 Mobile | 10.0.15254.x | Oct 2017 | Fall Creators (Redstone 3) |
| **Final Build** | **15254.603** | **Jan 2020** | **Final security update** |

### **App Compatibility (2024)**
| Category | Status | Notes |
|----------|--------|-------|
| **WhatsApp** | ❌ Dead | Stopped working Feb 2020 |
| **Messenger** | ❌ Dead | No updates since 2019 |
| **Instagram** | ❌ Dead | No app / web wrapper fails |
| **Facebook** | ❌ Dead | Web wrapper only |
| **Edge Browser** | ✅ Works | Basic HTML5, no extensions |
| **Email/Calendar** | ✅ Works | IMAP/Exchange/CalDAV/CardDAV |
| **Maps (Here WeGo)** | ✅ Excellent | Offline maps, turn-by-turn |
| **Camera** | ✅ Works | Full resolution, living images |
| **OneDrive/Office** | ⚠️ Limited | Web only, apps deprecated |
| **Store** | ⚠️ Limited | Only legacy UWP apps |

---

## 🔧 Hardware Variants

| Model | RM Code | SIM | Region | Wireless Charging |
|-------|---------|-----|--------|-------------------|
| Lumia 730 | RM-914 | Single SIM | Global | ❌ |
| Lumia 730 Dual SIM | RM-914 | Dual SIM | Global | ❌ |
| Lumia 735 | RM-915 | Single SIM | US/CA | ✅ Qi |
| Lumia 735 Dual SIM | RM-915 | Dual SIM | Global | ✅ Qi |

**Identification:**
- Settings → About → More Info → Model: RM-914 / RM-915
- Settings → About → More Info → Hardware Version: 1.0.0.x
- Bootloader: `fastboot getvar product` → `eagle`

---

## 🔧 Developer Information

### **Bootloader**
- **Status:** Locked (Microsoft)
- **Unlock:** Not officially supported
- **Fastboot:** Not available (no fastboot mode)
- **Recovery:** Windows Phone Recovery (gears ⚙️)
- **EDL Mode:** Test points on motherboard (requires disassembly)

### **Partition Layout (Typical)**
| Partition | Size | Purpose |
|-----------|------|---------|
| `sbl1` | ~1 MB | Secondary Bootloader 1 |
| `rpm` | ~1 MB | Resource Power Manager |
| `tz` | ~4 MB | TrustZone |
| `hyp` | ~1 MB | Hypervisor |
| `aboot` | ~2 MB | Android Boot / Little Kernel |
| `boot` | ~32 MB | Kernel + Ramdisk |
| `dtbo` | ~8 MB | Device Tree Blob Overlay |
| `vbmeta` | ~64 KB | Verified Boot Metadata |
| `system` | ~2 GB | OS System |
| `vendor` | ~1 GB | Vendor Partition |
| `userdata` | ~4.5 GB | User Data |
| `modem` | ~64 MB | Baseband Firmware |
| `persist` | ~32 MB | Persistent Data |

### **Partition Commands (EDL Mode)**
```bash
# In EDL mode (QDLoader 9008):
# Read partition info
ls /dev/block/by-name/

# Backup critical partitions
dd if=/dev/block/by-name/boot of=/sdcard/backup/boot.img
dd if=/dev/block/by-name/dtbo of=/sdcard/backup/dtbo.img
dd if=/dev/block/by-name/vbmeta of=/sdcard/backup/vbmeta.img
```

---

## 🔐 Security Features

| Feature | Status |
|---------|--------|
| **Secure Boot** | ✅ (Windows only) |
| **Device Encryption** | ✅ BitLocker (Windows 10 Mobile) |
| **Secure Boot** | ✅ UEFI Secure Boot |
| **TPM** | ❌ No discrete TPM |
| **Secure Enclave** | ❌ No |
| **Biometrics** | ❌ No fingerprint/iris |
| **SIM Lock** | Carrier dependent |

---

## 📦 Accessories Compatibility

| Accessory | Compatibility |
|-----------|---------------|
| **Qi Wireless Charging** | ✅ (735 only) |
| **USB OTG** | ✅ (keyboard, mouse, flash drive) |
| **HDMI Out** | ❌ (no MHL/Slimport) |
| **DisplayPort** | ❌ |
| **Gamepad (Bluetooth)** | ✅ |
| **Keyboard (Bluetooth/USB)** | ✅ |
| **Headset (3.5mm)** | ✅ |
| **Car Kit (Bluetooth)** | ✅ |

---

## 📦 What's in the Box (Original)

| Item | Included |
|------|----------|
| Lumia 730/735 handset | ✅ |
| Battery (BP-4G, 2200 mAh) | ✅ |
| Charger (DC-19, 5V/1.5A) | ✅ |
| microUSB cable | ✅ |
| Stereo headset (WH-208) | ✅ |
| Quick Start Guide | ✅ |
| Warranty Card | ✅ |

---

## 📋 Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│                    LUMIA 730 / 735 QUICK REF                 │
├─────────────────────────────────────────────────────────────┤
│ Model:          Lumia 730 (RM-914) / 735 (RM-915)           │
│ SoC:            Snapdragon 400 (MSM8226)                     │
│ CPU:            4× Cortex-A7 @ 1.2 GHz (32-bit)             │
| GPU:            Adreno 305                                   |
| RAM:            1 GB LPDDR3                                  |
| Storage:        8 GB (4.5 GB free)                           |
| Display:        4.7" 720p OLED, Gorilla Glass 3             |
| Rear Cam:       6.7 MP Zeiss f/1.9 + LED flash              |
| Front Cam:      5 MP f/2.4 wide-angle                        |
| Battery:        2200 mAh (replaceable)                       |
| OS Max:         Windows 10 Mobile 15254.603                 |
| Bootloader:     LOCKED (no unlock)                           |
| Android Port:   IMPOSSIBLE (32-bit, locked BL, no kernel)   |
| Best Use 2024:  Offline GPS / Camera / MP3 / Backup Phone   |
└─────────────────────────────────────────────────────────────┘
```

---

## 🔗 Useful Links

| Resource | URL |
|----------|-----|
| **WDRT Download** | https://go.microsoft.com/fwlink/?LinkId=708425 |
| **WPInternals** | https://github.com/WOA-Project/WOA-Project |
| **XDA Lumia 730** | https://xdaforums.com/f/nokia-lumia-730.4529/ |
| **Lumia Firmware** | https://firmware.nokia.com/ (legacy) |
| **LineageOS Devices** | https://wiki.lineageos.org/devices/ (730 NOT listed) |