# Conexus Harmattan PoC — egui Framework

**Framework:** egui 0.28 (immediate mode)  
**Target:** Linux (Wayland/X11) + Android (NDK via winit - experimental)  
**Spec:** `docs/ui-poc-spec.md`

---

## Quick Start

### Linux
```bash
# Build
cargo build --release

# Run
./target/release/conexus-poc-egui
```

### Android (Experimental)
```bash
# Prerequisites
# - Android NDK r26c+
# - winit-android support (experimental)
# - egui_winit + wgpu on Android (experimental)

# Build
cargo build --release --target aarch64-linux-android

# Note: Android support in egui is experimental
# Requires winit-android + wgpu-android which are not stable
```

---

## Project Structure

```
poc-egui/
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
| Tile flip (600ms Y-axis) | ❌ Not implemented |
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
- [ ] Screen reader compatible (NVDA/TalkBack/Orca)

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

## Known Issues / Limitations (egui)

| Issue | Impact |
|-------|--------|
| Android support experimental | winit-android + wgpu-android not stable |
| Tile flip 3D animation | Not implemented (immediate mode limitations) |
| Drag-drop reorder | Not implemented |
| Long-press context menu | Not implemented |
| Reduced motion media query | Not implemented |
| Android APK generation | Requires custom Gradle setup |
| Tile flip 3D Y-axis | Requires custom animation system |

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
- [ ] Tile flip works ❌ Not implemented

**Quality Score (if gates pass):**
- Hot reload / iteration: Fast (immediate mode, but recompile needed)
- Type safety: High (pure Rust)
- Component library: Minimal (build your own)
- Animation performance: Good for simple, limited for complex
- Android integration: Experimental
- Documentation: Good
- Licensing: MIT (compatible)

---

## Known Issues / TODO

- [ ] Implement tile flip 3D animation (requires custom animation system)
- [ ] Implement drag-drop reorder
- [ ] Implement long-press context menu
- [ ] Implement reduced motion support
- [ ] Stabilize Android build (wgpu + winit on Android)
- [ ] Add axe-core automated testing in CI
- [ ] Implement search modal
- [ ] Implement drag-drop visual feedback

---

## Evaluation Notes

**Strengths:**
- Pure Rust, immediate mode = simple mental model
- MIT license
- Very fast iteration (no DSL compilation)
- Excellent for tools/debug UIs
- Small binary size
- No runtime overhead

**Weaknesses for this PoC:**
- Android support experimental (blocker for ADR-008)
- No built-in animation system (3D flip requires custom work)
- Immediate mode = rebuild UI every frame
- No built-in component library (build from primitives)
- No declarative animation system

---

## Evaluation Notes

**egui is excellent for developer tools and simple UIs, but for a production launcher with complex animations (3D flip), Android support, and accessibility requirements, it falls short compared to Slint.**

---

## License

Apache-2.0