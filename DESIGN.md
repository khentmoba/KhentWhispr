---
name: SpeakFlow
description: A quiet, focused desktop dictation workspace for Windows
colors:
  primary: "#28664F"
  primary-hover: "#20553F"
  primary-active: "#194632"
  primary-subtle: "rgba(40, 102, 79, 0.10)"
  primary-muted: "rgba(40, 102, 79, 0.05)"
  primary-dark: "#9AC9AA"
  primary-dark-hover: "#B8DEC3"
  primary-dark-active: "#7FBF94"
  bg-primary: "#F6F8F7"
  bg-secondary: "#EAF0ED"
  bg-tertiary: "#DEE7E2"
  bg-elevated: "#FFFFFF"
  bg-sunken: "#E6EDE9"
  bg-overlay: "rgba(246, 248, 247, 0.94)"
  bg-primary-dark: "#141D19"
  bg-secondary-dark: "#1B2821"
  bg-tertiary-dark: "#29392F"
  bg-elevated-dark: "#202E26"
  shell: "#E5EDE7"
  shell-text: "#365644"
  shell-active: "#D1E1D6"
  shell-dark: "#18281F"
  shell-text-dark: "#BDCFBF"
  shell-active-dark: "#2E4937"
  text-primary: "#20362E"
  text-secondary: "#51635A"
  text-tertiary: "#617168"
  text-inverse: "#FFFFFF"
  text-primary-dark: "#EAF1EC"
  text-secondary-dark: "#B7C7BD"
  text-tertiary-dark: "#9BAFA1"
  border: "#AABBB1"
  border-subtle: "rgba(0, 0, 0, 0.08)"
  border-accent: "rgba(40, 102, 79, 0.30)"
  border-dark: "#3B5042"
  border-subtle-dark: "rgba(255, 255, 255, 0.06)"
  status-success: "#2E8B57"
  status-error: "#bf4d43"
  status-warning: "#D4890A"
  status-info: "#2C6FBB"
  assistant: "#7557d9"
typography:
  display:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "clamp(22px, 2.5vw, 30px)"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "-0.035em"
  headline:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "1.5rem"
    fontWeight: 600
    lineHeight: 1.3
    letterSpacing: "-0.014em"
  title:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "1.125rem"
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: "-0.014em"
  body:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: "-0.008em"
  label:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "0.75rem"
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: "-0.004em"
rounded:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "14px"
  xl: "18px"
  2xl: "28px"
  full: "9999px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "16px"
  lg: "24px"
  xl: "32px"
  2xl: "48px"
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.text-inverse}"
    rounded: "{rounded.lg}"
    padding: "8px 18px"
  button-primary-hover:
    backgroundColor: "{colors.primary-hover}"
  button-ghost:
    backgroundColor: "transparent"
    textColor: "{colors.text-secondary}"
    rounded: "{rounded.md}"
    padding: "6px 12px"
  nav-item:
    backgroundColor: "transparent"
    textColor: "{colors.shell-text}"
    rounded: "{rounded.sm}"
    padding: "10px 12px"
  nav-item-active:
    backgroundColor: "{colors.shell-active}"
    textColor: "{colors.text-primary}"
  recording-panel:
    backgroundColor: "{colors.bg-secondary}"
    rounded: "{rounded.lg}"
    padding: "23px 18px 18px"
  transcript-canvas:
    backgroundColor: "{colors.bg-elevated}"
    rounded: "{rounded.lg}"
    padding: "20px"
---

# Design System: SpeakFlow

## Overview

**Creative North Star: "The Reading-Room Wayfinding"**

SpeakFlow is a focused desktop dictation instrument built for everyday Windows writing. Instead of floating disconnected controls across an empty window or mimicking neon terminal consoles, SpeakFlow provides an architectural, stationery-grounded workspace that sits naturally beside ordinary Windows desktop applications. The visual identity draws from library index systems, reading-room wayfinding, and tactile desk tools: quiet mineral-green paper surfaces for daylight writing, paired with a warm forest-charcoal dark theme for evening sessions.

The architecture organizes dictation into clear, dedicated spaces: a persistent navigation rail distinguishes live dictation, saved history, and deep configuration. In the primary workspace, a cohesive recording panel sits directly beside a clean text transcript canvas. The recording unit transforms as a single solid instrument—calm and ready when idle, glowing softly with an authentic acoustic waveform only while speech is active. Transcription results populate an unadorned reading canvas where text can be reviewed, edited, or copied immediately without modal interruptions.

Every interface label, button, onboarding tip, and system prompt is English-only, keeping cognitive load low and interaction concise. All typography, iconography, and decorative geometry are code-first, leveraging native Windows system typefaces and inline SVGs with zero runtime font fetches and zero raster asset dependencies.

**Key Characteristics:**
- **Quiet Mineral & Charcoal Surfaces:** Restrained pale mineral-green (`#F6F8F7`) and evergreen rail (`#E5EDE7`) daytime surfaces; warm forest-charcoal (`#141D19`) dark theme.
- **Persistent Two-Zone Workspace:** Persistent sidebar rail routing between Dictation, History, and Settings; 2-column recording and transcript grid on desktop.
- **Adaptive Responsive Continuity:** Clean transition from 164px sidebar to 132px on compact screens, collapsing into a bottom navigation bar at 540px; 2-column dictation grid cleanly stacks vertically at 760px.
- **Code-First & Zero Raster Overhead:** Native Windows `Segoe UI Variable` typography and crisp vector SVGs; no remote web fonts or raster bitmap images.
- **English-Only Interface Chrome:** Purpose-built, concise English UI vocabulary across all status hints, empty states, and error dialogues.
- **Accessible & Restrained Motion:** High-contrast WCAG AA compliance, distinct `:focus-visible` keyboard rings, and full `prefers-reduced-motion` suppression.

## Colors

The palette balances mineral paper tones with deep evergreen structure and single-signal emerald accents, avoiding neon distraction.

### Primary
- **Deep Mineral Green** (`#28664F`): The primary daytime brand signal. Used for active recording controls, primary action buttons, active navigation markers, and keyboard focus outlines.
- **Mineral Sage (Dark Theme)** (`#9AC9AA`): The evening counterpart to Deep Mineral Green. Provides luminous, accessible contrast against forest-charcoal surfaces without glare.

### Secondary
- **Evergreen Shell Rail** (`#E5EDE7` light / `#18281F` dark): Structural container tone for the persistent navigation rail, framing the working canvas.
- **Shell Active Fill** (`#D1E1D6` light / `#2E4937` dark): Background fill for active navigation tabs and hovered rail destinations.
- **Assistant Violet** (`#7557D9` light / `#A78BFA` dark): Dedicated semantic accent for AI assistant conversations and secondary prompt expansion in the subtitle overlay.

### Neutral
- **Mineral Primary Canvas** (`#F6F8F7` light / `#141D19` dark): The root app background, providing a glare-free mineral paper texture.
- **Secondary Working Surface** (`#EAF0ED` light / `#1B2821` dark): Surface fill for the recording instrument panel, cards, and input backgrounds.
- **Tertiary Recessed Fill** (`#DEE7E2` light / `#29392F` dark): Subtle sunken background for chips, switches, and sliders.
- **Elevated Canvas** (`#FFFFFF` light / `#202E26` dark): Pure paper-white canvas for the transcript results area and tactile keycap tiles.
- **Deep Forest Text** (`#20362E` light / `#EAF1EC` dark): Primary typography color exceeding WCAG AA contrast standards.
- **Muted Sage Text** (`#51635A` light / `#B7C7BD` dark): Secondary text for captions, hotkey labels, and auxiliary descriptions.
- **Border Sage** (`#AABBB1` light / `#3B5042` dark): Crisp boundary line separating panels, headers, and navigation boundaries.
- **Subtle Divider** (`rgba(0, 0, 0, 0.08)` light / `rgba(255, 255, 255, 0.06)` dark): Hairline separators inside lists and cards.

### Named Rules
**The Single Signal Rule.** The primary green accent (`#28664F` / `#9AC9AA`) is reserved strictly for active voice status, primary button commits, and focus indicators. Ambient backgrounds remain strictly neutral.
**The No-Faux-Glass Rule.** Translucent glassmorphism with heavy blurs is prohibited in the main workspace. Working surfaces are opaque mineral planes. Glass overlay blurs are restricted solely to floating desktop overlays (`subtitle` and `selection-toolbar`).

## Typography

**Display Font:** "Segoe UI Variable", "Segoe UI", system-ui, sans-serif
**Body Font:** "Segoe UI Variable", "Segoe UI", system-ui, sans-serif
**Label/Mono Font:** "Cascadia Mono", Consolas, monospace

**Character:** Native Windows typographic precision. By employing the operating system's installed Segoe UI Variable and Cascadia Mono fonts, typography renders with crisp native hinting, instant startup, zero network latency, and zero layout shift.

### Hierarchy
- **Display** (SemiBold 600, `clamp(22px, 2.5vw, 30px)`, line-height 1.2, tracking `-0.035em`): Workspace welcome heading ("Make room for your words.").
- **Headline** (SemiBold 600, `1.5rem` / 24px, line-height 1.3, tracking `-0.014em`): Major section titles and settings category heads.
- **Title** (SemiBold 600, `1.125rem` / 18px to `1.25rem` / 20px, line-height 1.4, tracking `-0.014em`): Panel titles, empty state greetings, and overlay headers.
- **Body** (Regular 400 to Medium 500, `0.875rem` / 14px to `0.9375rem` / 15px, line-height 1.6 to 1.85, tracking `-0.008em`): Dictation transcript text, reading paragraphs, and history items.
- **Label** (Medium 500 to SemiBold 600, `0.75rem` / 12px to `0.8125rem` / 13px, line-height 1.4, tracking `-0.004em`): Navigation destinations, tactile hotkey labels, status chips, and metadata badges.

### Named Rules
**The Zero-Download Type Rule.** Never request web fonts from external networks (e.g., Google Fonts). All type must render directly from the native Windows platform font stack.
**The Tabular Precision Rule.** All numeric counters, recording timers, latency measurements, and hotkey keycaps must declare `font-variant-numeric: tabular-nums` to eliminate layout jitter during live updates.
**The English Chrome Rule.** All application navigation titles, button labels, empty states, and system feedback strings must remain in English to preserve editorial consistency.

## Layout

SpeakFlow uses a grid-based spatial model centered on persistent spatial wayfinding and content-first reading density.

- **Main Desktop Shell:** A persistent 2-column split featuring a 164px navigation rail (`.app-sidebar`) docked on the left and an open workspace canvas (`.app-workspace`) occupying the remaining space (`grid-template-columns: 164px minmax(0, 1fr)`).
- **Dictation Grid:** The main workspace organizes the dictation view into a 2-column split (`grid-template-columns: minmax(180px, 0.85fr) minmax(0, 1.4fr)`). The left column contains the self-contained recording instrument panel; the right column holds the expansive transcript canvas.
- **Wide Desktop Enhancement (>= 1200px):** Spacing expands comfortably (`padding: 28px 40px 36px`), and the recording panel locks into a proportional `minmax(240px, 0.7fr) minmax(0, 1.6fr)` ratio.
- **Stacked Grid Breakpoint (<= 760px):** The sidebar condenses to 132px. The dictation grid collapses to a single vertical column (`grid-template-columns: minmax(0, 1fr)`), placing the recording instrument directly above the transcript results area.
- **Bottom Navigation Breakpoint (<= 540px):** The shell switches to `flex-direction: column-reverse`. The vertical sidebar transforms into a fixed horizontal bottom navigation bar (`app-navigation` flex-direction: row) with evenly distributed touch targets, hiding the sidebar brand mark and note to prioritize vertical canvas height.
- **Spacing Rhythm:** Built on an 8pt baseline scale: `4px` (`--space-xs`), `8px` (`--space-sm`), `16px` (`--space-md`), `24px` (`--space-lg`), `32px` (`--space-xl`), `48px` (`--space-2xl`).

## Elevation & Depth

SpeakFlow avoids high-contrast drop shadows and artificial floating card stacks, favoring quiet tonal layering and precise flat borders.

Depth is achieved through background tonal steps: the root canvas (`--color-bg-primary`) sits beneath slightly recessed secondary panels (`--color-bg-secondary`), which frame elevated crisp white reading canvases (`--color-bg-elevated`). Boundaries are established via crisp 1px borders (`--color-border-subtle`).

### Shadow Vocabulary
- **Subtle Surface** (`box-shadow: 0 1px 2px rgba(23, 32, 48, 0.06)`): Applied to tactile keycaps and quiet unpressed buttons.
- **Card Hover** (`box-shadow: 0 4px 12px rgba(23, 32, 48, 0.12)`): Smooth lift on interactive containers and hover states.
- **Recording Glow Ring** (`box-shadow: 0 0 0 4px rgba(40, 102, 79, 0.20)`): Subtle acoustic focus ring around the active recording button.
- **Overlay Window** (`box-shadow: 0 8px 24px rgba(0, 0, 0, 0.16)`): Applied to desktop floating windows (`subtitle` and `selection-toolbar`) over native Windows apps.

### Named Rules
**The Tonal Layering Rule.** Establish depth by contrasting surface tints (`#F6F8F7` vs `#EAF0ED` vs `#FFFFFF`) and hairline borders rather than heavy blur drop shadows.
**The Flat Workspace Rule.** Inside the main window, result cards and history items sit flat with zero drop shadow to read as continuous stationery.

## Shapes

SpeakFlow's form language uses squircle-inspired rounded rectangles that soften digital density while maintaining architectural order.

- **Micro Controls & Chips (`4px` to `8px`):** Small buttons, status badges, and tactile keycaps use `--radius-xs` (4px) and `--radius-sm` (8px).
- **Cards & Interactive Containers (`12px` to `14px`):** Settings cards, history cards, and the recording instrument panel use `--radius-md` (12px) and `--radius-lg` (14px).
- **Primary Recording Instrument (`28px`):** The main recording button uses a soft squircle shape (`border-radius: 28px`), distinguishing it from generic circular media controls.
- **Pills & Indicator Badges (`9999px`):** Mode tags, toggle switches, and status pills use `--radius-full`.
- **Icon Stroke Consistency:** All iconography uses Lucide SVG icons rendered with a uniform `stroke-width: 1.5` to `1.7`.

## Components

### Buttons
- **Shape:** Rounded rectangle (`--radius-lg` 14px for primary, `--radius-md` 12px for ghost).
- **Primary:** Deep mineral green background (`#28664F`), white text, subtle top inner highlight (`inset 0 1px 0 rgba(255,255,255,0.12)`), padding `8px 18px`.
- **Hover / Focus:** Hover shifts to `#20553F` with `-1px` transform; active scales to `0.97` with `--ease-micro`. Focus-visible exhibits an accented 2px outline offset by 2px.
- **Ghost:** Transparent background, `--color-text-secondary` type, subtle accent background on hover (`rgba(40, 102, 79, 0.05)`).

### Tactile Hotkey Badge
- **Style:** Compact hardware key mockup. Raised white/elevated keycap (`min-width: 32px`, padding `3px 8px`, border `1px solid var(--color-border)`, box-shadow `0 2px 0 var(--color-border-subtle)`).
- **State:** Displays current hotkey binding (e.g. `F2`) with clear action instructions ("Hold to speak" or "Press to toggle"). When recording, the badge gains an active glow.

### Recording Panel (Signature Component)
- **Shape:** Unified instrument container (`border-radius: 14px`, padding `23px 18px 18px`).
- **Color:** Soft mineral secondary background (`#EAF0ED`), shifting to subtle green tint (`rgba(40, 102, 79, 0.10)`) during active recording.
- **Behavior:** Houses header status, center recording core (84px squircle button), acoustic live equalizer bars, hotkey badge, and persistent footer instruction ("Works in your active app").

### Transcript Canvas & Result Card
- **Shape:** Elevated crisp paper surface (`#FFFFFF` in light, `#202E26` in dark, `--radius-lg` 14px, 1px subtle border).
- **Empty State:** Centered stationery prompt with `FileText` icon, clean headline ("Your words will appear here."), and reassuring guidance.
- **Active State:** Direct inline editable textarea (`min-height: 180px`, line-height `1.85`), metadata counters (duration, word/char count, detected language), and instantaneous one-click clipboard copy.

### Navigation (App Sidebar & Bottom Bar)
- **Style:** Left docked rail (164px desktop / 132px tablet) with brand header and footer philosophy note; converts to sticky bottom bar at `<= 540px`.
- **Items:** 43px height touch targets with 18px SVG icons (Dictation, History, Settings). Active destination receives `--color-shell-active` fill and accent green icon tint.

### Desktop Overlays (Subtitle & Selection Toolbar)
- **Subtitle Overlay:** Floating desktop capsule pinned bottom-center over target applications. Fully transparent root window, backdrop blur, live streaming text, and expandable AI assistant conversation drawer.
- **Selection Assistant:** Floating toolbar positioned near user cursor. Elevated panel with quick AI transform pills (Proofread, Polish, Translate, Explain, Shorten, Expand) and in-place text replacement.

## Do's and Don'ts

### Do:
- **Do** keep all interface labels, empty states, system dialogs, and error messages strictly in English.
- **Do** rely exclusively on native system font stacks (`Segoe UI Variable`, `Cascadia Mono`) without introducing remote web font dependencies.
- **Do** pair pale mineral-green working surfaces with deep forest-charcoal dark mode tokens.
- **Do** collapse the 164px persistent sidebar into a balanced horizontal bottom navigation bar at `<= 540px`.
- **Do** stack the 2-column dictation grid into a single vertical column at `<= 760px`.
- **Do** enforce visible `:focus-visible` keyboard rings (`outline: 2px solid var(--color-accent)`) for accessible keyboard navigation.
- **Do** respect `@media (prefers-reduced-motion: reduce)` by immediately dropping animation and transition durations across both the workspace and overlays.
- **Do** construct all visual elements, icons, and indicators code-first using inline SVG and CSS geometry without adding raster images.

### Don't:
- **Don't** download or import remote runtime fonts (e.g. Google Fonts or webfont CDNs).
- **Don't** introduce new PNG, JPG, WEBP, or other raster bitmap image files.
- **Don't** float the primary recording trigger as a detached floating bubble over empty space; anchor it inside the structured recording instrument panel.
- **Don't** use decorative multi-color rainbow gradients or neon glowing halos around idle controls.
- **Don't** introduce non-English UI strings or dual-language switcher widgets into the interface chrome.
- **Don't** rely on heavy blur drop shadows in place of clear tonal background layering and crisp hairline borders.
- **Don't** suppress keyboard focus rings on interactive controls.
