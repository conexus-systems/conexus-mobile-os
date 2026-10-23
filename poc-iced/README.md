# Conexus Harmattan PoC — Iced Framework

**Framework:** Iced 0.12 (wgpu backend)  
**Target:** Linux (Wayland/X11) + Android (NDK - experimental)  
**Spec:** `docs/ui-poc-spec.md`

---

## Quick Start

### Linux
```bash
# Build
cargo build --release

# Run
./target/release/conexus-poc-iced
```

### Android (Experimental)
```bash
# Prerequisites
# - Android NDK r26c+
# - wgpu-android support (experimental)
# - winit-android support (experimental)

# Build
cargo build --release --target aarch64-linux-android

# Note: Android support in Iced is experimental
# May require additional configuration for wgpu + winit on Android
```

---

## Project Structure

```
poc-iced/
├── Cargo.toml
├── design-tokens.json
├── src/
│   ├── main.rs
│   ├── models.rs
│   ├── tokens.rs
│   ├── theme.rs
│   ├── home_screen.rs
│   ├── tile.rs
│   ├── pivot_tabs.rs
│   └── platform.rs
├── .github/workflows/ci.yml
└── README.md
```

---

## Features Implemented

- ✅ Home Grid (responsive 4/5/6/8 columns)
- ✅ 3 Tiles: Small (1×1), Medium (2×2), Wide (4×2)
- ✅ Pivot Tabs: Eventos · Apps · Configurações
- ✅ Theme Toggle: Dark / Light / High Contrast
- ✅ Design Tokens Applied (colors, spacing, typography, motion)
- ✅ Responsive Grid (360dp→4cols, 600dp→5cols, 900dp→6cols, 1200dp→8cols)
- ✅ Keyboard Navigation (Tab, Arrows, Enter, Escape)
- ✅ Touch/Mouse Interaction (Tap, Scroll)

---

## PoC Spec Compliance

| Requirement | Status |
|-------------|--------|
| Linux build (x86_64) | ✅ |
| Android build (aarch64) | ⚠️ Experimental |
| Linux run 60fps | 🔄 Testing |
| Android run | ⚠️ Experimental |
| Theme toggle (Dark/Light/HC) | ✅ |
| Pivot swipe navigation | ✅ |
| Tile flip (600ms Y-axis) | 🔄 Not implemented |
| Axe-core 0 violations | 🔄 Pending |
| Theme toggle instant | ✅ |

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
- [ ] Screen reader compatible

---

## Development

```bash
# Development
cargo run

# Release build
cargo build --release

# Run with debug logging
RUST_LOG=debug cargo run
```

---

## Known Issues / Limitations (Iced)

| Issue | Impact |
|-------|--------|
| Android support experimental | wgpu + winit on Android not stable |
| Tile flip animation | Not yet implemented (requires custom animation) |
| Tile flip 3D (Y-axis) | Requires custom wgpu shader |
| Drag-drop reorder | Not implemented |
| Long-press context menu | Not implemented |
| Reduced motion media query | Not implemented |
| Android APK generation | Requires custom Gradle setup |

---

## Evaluation Criteria (from `docs/ui-poc-spec.md`)

**Binary Gates (must pass):**
- [ ] Linux build ✅
- [ ] Android build ⚠️ Experimental
- [ ] Linux run 60fps 🔄 Testing
- [ ] Android run ⚠️ Experimental
- [ ] Axe-core 0 violations 🔄 Pending
- [ ] Theme toggle works ✅
- [ ] Pivot swipe works ✅
- [ ] Tile flip works 🔄 Not implemented

**Quality Score (if gates pass):**
- Hot reload / iteration: Good (cargo watch + recompile)
- Type safety: High (pure Rust, Elm architecture)
- Component library: Basic (custom widgets)
- Animation performance: TBD
- Android integration: Experimental
- Documentation: Good
- Licensing: MIT (compatible)

---

## Known Issues / TODO

- [ ] Implement tile flip 3D animation (requires wgpu custom shader)
- [ ] Implement drag-drop reorder
- [ ] Implement long-press context menu
- [ ] Implement reduced motion support
- [ ] Stabilize Android build (wgpu + winit)
- [ ] Add axe-core automated testing in CI
- [ ] Add drag-drop visual feedback
- [ ] Implement search modal

---

## Evaluation Notes

**Strengths:**
- Pure Rust, no DSL
- Elm architecture = predictable state management
- MIT license
- Good hot reload via `cargo watch`
- Type-safe UI

**Weaknesses for this PoC:**
- Android support is experimental (blocker for ADR-008)
- No built-in 3D flip animation (requires custom wgpu work)
- Smaller component ecosystem
- wgpu on Android not production-ready

---

## License

Apache-2.0