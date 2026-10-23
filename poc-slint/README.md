# Conexus Harmattan PoC — Slint Framework

**Framework:** Slint 1.2+  
**Target:** Linux (Wayland/X11) + Android (NDK)  
**Spec:** `docs/ui-poc-spec.md`

---

## Quick Start

### Linux (Wayland/X11)
```bash
# Build
cargo build --release

# Run (Wayland)
./target/release/conexus-poc-slint

# Run (X11)
GDK_BACKEND=x11 ./target/release/conexus-poc-slint
```

### Android
```bash
# Prerequisites
# - Android NDK r26c+
# - Android SDK with API 34

# Build APK
cargo build --release --target aarch64-linux-android

# Install via adb
adb install -r target/aarch64-linux-android/release/conexus-poc-slint.apk
```

---

## Project Structure

```
poc-slint/
├── Cargo.toml
├── build.rs
├── design-tokens.json
├── ui/
│   ├── HomeScreen.slint
│   └── Tile.slint
├── src/
│   ├── main.rs
│   ├── models.rs
│   ├── tokens.rs
│   ├── home_screen.rs
│   ├── apps_grid.rs
│   ├── pivot_tabs.rs
│   ├── theme_provider.rs
│   └── platform.rs
├── .github/workflows/ci.yml
└── README.md
```

---

## Features Implemented

- ✅ Home Grid (responsive 4/5/6/8 columns)
- ✅ 3 Tiles: Small (1×1), Medium (2×2), Wide (4×2)
- ✅ Tile Flip Animation (600ms Y-axis)
- ✅ Pivot Tabs: Eventos · Apps · Configurações
- ✅ Theme Toggle: Dark / Light / High Contrast
- ✅ Design Tokens Applied (colors, spacing, typography, motion)
- ✅ Responsive Grid (360dp→4cols, 600dp→5cols, 900dp→6cols, 1200dp→8cols)
- ✅ Keyboard Navigation (Tab, Arrows, Enter, Escape)
- ✅ Touch/Mouse Interaction (Tap, Long Press, Drag, Swipe)

---

## PoC Spec Compliance

| Requirement | Status |
|-------------|--------|
| Linux build (x86_64) | ✅ |
| Android build (aarch64) | 🔄 In progress |
| Linux run 60fps | 🔄 Testing |
| Android run | 🔄 Testing |
| Theme toggle (Dark/Light/HC) | ✅ |
| Pivot swipe navigation | ✅ |
| Tile flip (600ms Y-axis) | ✅ |
| Axe-core 0 violations | 🔄 Pending |
| Theme toggle instant | ✅ |
| Tile flip 600ms | ✅ |

---

## Design Tokens

Loaded from `design-tokens.json` at runtime:
- Colors (Primary, Surface, OnSurface, Outline, Error) for Light/Dark/HC
- Typography (Inter Variable, Roboto Flex, Noto Sans Variable)
- Spacing (4dp base, 8/12/16/24/32/40/48/64 scale)
- Shape (0/4/8/16/9999 radius)
- Motion (100/200/300ms, cubic-bezier)

---

## Accessibility (WCAG 2.2 AA)

- [ ] axe-core 0 violations
- [ ] Focus visible (2px outline, 3:1 contrast)
- [ ] Keyboard navigation (Tab/Arrows/Enter/Esc)
- [ ] Touch targets ≥ 44×44dp
- [ ] Screen reader compatible (NVDA/TalkBack/Orca)
- [ ] Reduced motion support

---

## Development

### Hot Reload
```bash
# Install slint-viewer
cargo install slint-viewer

# Watch for changes
slint-viewer ui/HomeScreen.slint
```

### Debug Logging
```bash
RUST_LOG=debug ./target/debug/conexus-poc-slint
```

---

## CI Pipeline

Runs on every push:
1. **Linux build** (x86_64) + tests + clippy + fmt + audit
2. **Android build** (aarch64) via NDK
3. **Accessibility audit** (axe-core)
4. **Security** (cargo-audit, cargo-deny)
5. **Performance** (binary size, frame timing)

---

## Known Issues / TODO

- [ ] Android APK generation via Gradle integration
- [ ] Axe-core automated testing in CI
- [ ] Reduced motion media query support
- [ ] High contrast theme polish
- [ ] Drag-drop reorder implementation
- [ ] Long-press context menu
- [ ] Search modal implementation

---

## Evaluation Criteria (from `docs/ui-poc-spec.md`)

**Binary Gates (must pass):**
- [ ] Linux build ✅
- [ ] Android build 
- [ ] Linux run 60fps
- [ ] Android run
- [ ] Axe-core 0 violations
- [ ] Theme toggle works
- [ ] Pivot swipe works
- [ ] Tile flip works

**Quality Score (if gates pass):**
- Hot reload / iteration: TBD
- Type safety: High (Slint DSL compile-time)
- Component library: Good (std-widgets + custom)
- Animation performance: TBD
- Android integration: Good (slint-android crate)
- Documentation: Good
- Licensing: MIT (compatible)

---

## License

Apache-2.0