# SpeakFlow Brand & Logo Assets

Official vector and raster branding assets for **SpeakFlow** — local and cloud speech-to-text dictation for Windows.

> **Name Rule:** The application name is strictly **SpeakFlow** (CamelCase, single word). Use this name consistently across the app and repository.

---

## 1. Design Rationale & Concept

The SpeakFlow visual identity is rooted in the product's core interaction model: **"Quiet Instrument"** — a high-performance Windows tool designed to work right where you type, staying unobtrusive until invoked.

- **The Voice Node (Left Origin):** A solid circular anchor representing the voice origin, microphone capture, and the user's intentional utterance.
- **The Acoustic-to-Text Waves (Triple Forward Flow):** Three parabolic radiating wave arcs with calibrated weights (16px, 20px, 24px) signifying the transformation pipeline:
  1. *Inner arc:* Real-time voice activity detection (VAD) & audio capture.
  2. *Mid arc:* Local / cloud speech recognition streaming (Confucius4-R2T2 / Qwen3-ASR).
  3. *Outer arc:* Fluid text stream flowing directly into the destination application.
- **Warm Copper Signal:** Anchored by warm terracotta copper (`#AE5630`, `#D67A53` to `#914022`), contrasting against the sea of generic tech blues. This matches SpeakFlow's primary action and recording state signals across the desktop UI.
- **Platform Alignment:** Built on Windows Fluent squircle curvature (continuous corner smoothing) with subtle specular top-rim lighting and ground shadow depth.

---

## 2. Color Palette Tokens

| Token | Hex Value | Role / Usage |
|:---|:---|:---|
| `--color-accent` | `#AE5630` | Core brand accent, wordmark "Flow" |
| `--color-accent-hover` | `#C4633A` | Interactive hover state |
| `--color-accent-active` | `#943F22` | Deep pressed / active state |
| `--color-text-brand` | `#D97757` | High-contrast brand text in dark themes |
| Gradient Start | `#D67A53` | Top-left specular copper highlight |
| Gradient Mid | `#B85F3A` | Rich mid-tone copper body |
| Gradient End | `#914022` | Bottom-right roasted umber shadow |
| `--color-text-primary` | `#1A1A18` | Dark typography on light surfaces |
| `--color-text-inverse` | `#F5F5F0` | Light typography on dark surfaces |
| Secondary Text | `#5A5854` / `#B0AEA5` | Subtitles, platform tags, metadata |

---

## 3. Typography Guidelines

- **Primary Logotype Font:** `Segoe UI` (Windows native font system) / `Segoe UI Variable Display` with fallbacks `-apple-system, BlinkMacSystemFont, Roboto, sans-serif`.
- **Wordmark Weight Hierarchy:**
  - **Speak:** `font-weight: 700` (Bold), `#1A1A18` (Light mode) or `#F5F5F0` (Dark mode)
  - **Flow:** `font-weight: 600` (Semi-bold), `#AE5630` (Light mode) or `#D97757` (Dark mode)
- **Category Tagline:** `VOICE TO TEXT · WINDOWS`
  - Uppercase, `letter-spacing: 2.6px - 2.8px`, `font-size: 13px`, `font-weight: 600`.

---

## 4. Asset Catalog

| Filename | Format | Dimensions | Purpose |
|:---|:---|:---|:---|
| `speakflow-icon.svg` | Vector SVG | 512 x 512 | Master vector app icon with squircle container and depth |
| `speakflow-icon.png` | Raster PNG | 512 x 512 | High-res icon badge (512px) |
| `speakflow-icon-256.png` | Raster PNG | 256 x 256 | Windows desktop display icon (256px) |
| `speakflow-icon-128.png` | Raster PNG | 128 x 128 | Windows app installer / settings display (128px) |
| `speakflow-icon-64.png` | Raster PNG | 64 x 64 | Taskbar / tray / navigation item (64px) |
| `speakflow-icon-32.png` | Raster PNG | 32 x 32 | Crisp small icon / notification badge (32px) |
| `speakflow-mark.svg` | Vector SVG | 512 x 512 | Isolated copper brand symbol on transparent background |
| `speakflow-mark.png` | Raster PNG | 512 x 512 | Isolated copper brand symbol on transparent background |
| `speakflow-mark-white.svg` | Vector SVG | 512 x 512 | Isolated white brand symbol on transparent background |
| `speakflow-mark-white.png` | Raster PNG | 512 x 512 | Isolated white brand symbol on transparent background |
| `speakflow-logo-horizontal.svg` | Vector SVG | 800 x 200 | Primary horizontal lockup for light backgrounds |
| `speakflow-logo-horizontal.png` | Raster PNG | 1600 x 400 (@2x) | Crisp high-density horizontal lockup for light surfaces |
| `speakflow-logo-horizontal-dark.svg` | Vector SVG | 800 x 200 | Horizontal lockup for dark backgrounds |
| `speakflow-logo-horizontal-dark.png` | Raster PNG | 1600 x 400 (@2x) | Crisp high-density horizontal lockup for dark surfaces |
| `speakflow-logo-vertical.svg` | Vector SVG | 500 x 500 | Centered vertical lockup for light surfaces / splash screens |
| `speakflow-logo-vertical.png` | Raster PNG | 1000 x 1000 (@2x) | Centered vertical lockup raster for light surfaces |
| `speakflow-logo-vertical-dark.svg` | Vector SVG | 500 x 500 | Centered vertical lockup for dark surfaces |
| `speakflow-logo-vertical-dark.png` | Raster PNG | 1000 x 1000 (@2x) | Centered vertical lockup raster for dark surfaces |

---

## 5. Usage & Integrity Rules

1. **Clear Space:** Maintain a clear margin of at least `0.5x` (where `x` is the height of the brand mark) on all sides of the lockup.
2. **Contrast & Backgrounds:**
   - Use `speakflow-logo-horizontal.svg` / `.png` on white and light neutral surfaces (`#FFFFFF`, `#F5F6F7`, `#EFF1F3`).
   - Use `speakflow-logo-horizontal-dark.svg` / `.png` on obsidian and dark neutral surfaces (`#1A1917`, `#232220`, `#111111`).
3. **No Distortion:** Do not stretch, skew, rotate, or alter the proportional relationship between the mark and the logotype.
4. **Name Integrity:** Always spell as **SpeakFlow** (never "Speak Flow" or "speakflow").
