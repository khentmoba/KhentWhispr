---
name: SpeakFlow
description: Quiet Windows dictation with charcoal surfaces and restrained blue actions
colors:
  primary: "#315EAB"
  primary-dark: "#79A9FF"
  bg-primary: "#F7F8FA"
  bg-secondary: "#EEEFF2"
  bg-elevated: "#FFFFFF"
  text-primary: "#191C21"
  text-secondary: "#515863"
  border: "#CBD5E1"
  bg-primary-dark: "#101112"
  bg-secondary-dark: "#17191C"
  bg-elevated-dark: "#1B1D20"
  bg-tertiary-dark: "#222529"
  shell-dark: "#0B0C0D"
  shell-active-dark: "#202328"
  text-primary-dark: "#F3F4F5"
  text-secondary-dark: "#A4A8B0"
  text-tertiary-dark: "#858B95"
  border-dark: "#34383E"
typography:
  headline:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "30px"
    fontWeight: 500
    lineHeight: 1.2
    letterSpacing: "-0.03em"
  title:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "20px"
    fontWeight: 550
    letterSpacing: "-0.02em"
  body:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "14px"
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: "-0.008em"
  label:
    fontFamily: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif'
    fontSize: "12px"
    lineHeight: 1.7
  keycap:
    fontFamily: '"Cascadia Mono", Consolas, monospace'
    fontSize: "11px"
rounded:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "14px"
  xl: "18px"
  full: "9999px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "16px"
  lg: "24px"
  xl: "32px"
  2xl: "48px"
components:
  recording-panel-dark:
    backgroundColor: "{colors.bg-elevated-dark}"
    textColor: "{colors.text-primary-dark}"
    rounded: "{rounded.lg}"
    padding: "24px"
  navigation-active-dark:
    backgroundColor: "{colors.shell-active-dark}"
    textColor: "{colors.text-primary-dark}"
    rounded: "{rounded.sm}"
    padding: "10px 12px"
  workspace-link-dark:
    textColor: "{colors.text-secondary-dark}"
    rounded: "{rounded.sm}"
    padding: "9px 12px"
---

# Design System: SpeakFlow

## Overview

**Creative North Star: "Workspace Dashboard"**

SpeakFlow is a quiet everyday dictation workspace beside the user's other Windows applications. Black and charcoal establish a focused environment; restrained blue marks actions, selection, and keyboard focus. Keep the SpeakFlow name, existing brand mark, native window affordances, and English interface.

The approved direction is design 6, recorded in `.impeccable/mocks/decision/workspace.html` and `workspace-preview.png`. Its hierarchy is recording and readiness first, recent dictations second, and a quieter setup column alongside them. Reference artwork guides composition; real product states and editable text take precedence, and no reference bitmap ships as application UI.

**Key Characteristics:**

- Neutral tonal layers and fine separators.
- A horizontal recording panel above recent work.
- Native Windows typography and readable compact controls.
- Clear readiness, activity, recovery, and keyboard feedback.

## Colors

### Primary

Use soft blue in dark mode and deeper blue in light mode for actions, links, selection, and focus. The frontmatter records normative values; `src/styles/theme.css` and `workspace.css` implement their theme roles.

### Neutral

The canvas is black charcoal; the sidebar is the deepest surface. Recording controls use an elevated charcoal fill. Secondary and tertiary text support the main text. Active navigation uses neutral fill and a thin blue inset marker. Dark is the fresh-install default; explicit light and system choices remain supported.

**The Signal Rule.** Blue identifies actions or states. Keep broad surfaces neutral, and preserve the theme's semantic success, error, warning, and assistant colors.

## Typography

Use the native UI and mono stacks in the frontmatter without remote font downloads. Keycaps use mono; counters and timestamps use tabular numerals. Keep the existing Lucide icons and SpeakFlow logo.

Page headings use the headline role; Recent dictations uses title. The recording headline is 21px on desktop and 18px at compact widths. Your setup uses a quieter 15px heading. Setup values are 13px, support labels 11-12px, and editable transcripts 16px with generous line height, reducing to 15px on narrow windows. Settings headings are 18px. Keep type hierarchy tied to the task.

## Layout

The desktop shell uses a 208px sidebar, flexible workspace, and 48px native title area. Dictation and History lead navigation; Settings and Updates anchor its lower part. Dictation scrolls within the shell, with 24px 40px 28px content padding at ordinary desktop widths.

A horizontal recording panel spans the dashboard top: microphone, readiness, and hotkey guidance beside model selection and real engine status. Below it, flexible Recent dictations sits beside a 270px Your setup column with a vertical rule and 36px gap. The latest transcript is editable and followed by up to five recent entries. Setup contains the engine, shortcut, Configure action, active-application guidance, and conditional first-use Quick Start; it stays visually quieter than recording and text.

At 1100px and below, the sidebar becomes 170px, setup becomes 210px, and gaps tighten. At 760px and below, navigation becomes a 72px icon rail, recording controls stack, and setup moves below recent work. At 540px and below, navigation moves to the bottom with labels, the title area becomes 40px, and content padding is 16px. At 1500px and above, Dictation padding grows to 30px 56px 36px.

Settings uses a vertical section index beside open form sections: 178px normally, 150px at compact widths, and a horizontal scrollable index below 760px. Preserve save semantics, providers, and native controls. History retains search, filters, entries, and detail/copy/export/delete/reprocess actions. Updates preserves progress, cancellation, and recovery states.

## Elevation & Depth

The workspace uses tonal layers and borders rather than lifted cards. The recording panel has elevated fill and a visible border; results and setup stay open and flat. Keep shared theme shadows for menus and overlays without adding idle glow. Subtitle and selection windows retain transparent desktop roots and restrained overlay elevation.

## Shapes

Use the shared radius scale for recurring controls. The recording panel has 14px corners; its 65px microphone action has 16px corners. Navigation and workspace links use 8px, the model selector 7px, and keycaps 5px with a stronger bottom border. Fine rules separate text and settings sections; status dots remain circular.

## Components

### Recording panel

Keep microphone, shortcut, model selection, and real readiness together. Use actual waveform events during recording and a spinner while connecting or processing. Distinguish downloads, repair, errors, connecting, recording, and processing from ready. Lock model selection during recording, processing, downloads, and selected-model saves. Preserve accessible name and pressed state; idle controls must not simulate activity.

### Recent dictations and setup

Lead with the section heading and View history action. Empty guidance is left aligned. Preserve latest-transcript editing, copy, correction learning, processing, audit notices, errors, and recovery. Recent results are separated text rows. Setup uses compact labels and quiet actions; dismissible Quick Start appears only in its supported first-use state. Do not fabricate transcripts or usage statistics.

### Navigation and fields

Use neutral active fill, blue inset marker, and accessible current-page state. Fields retain themed borders, caret, and selection. Keyboard focus uses a 2px accent outline with 3px offset. Preserve disabled semantics, reduced motion, and native drag/minimize/close behavior. Use the existing short duration/easing tokens for state transitions.

### Context awareness

Application text, selected text, and clipboard are native runtime sources, all on by default independently of legacy saved false values. There is no context settings page, toolbar entry, mode selector, source switch, or persisted context preference/command. Availability, password-field exclusion, original-window/session binding, and a 4,000-character per-source bound remain. See `docs/ui-redesign-research.md` for behavior and verification limits.

## Do's and Don'ts

### Do:

- **Do** preserve the recording-first dashboard and quieter setup column.
- **Do** derive status and waveform feedback from real lifecycle events.
- **Do** retain transcript editing, keyboard access, light/system themes, and recovery.
- **Do** reflow dashboard and settings navigation at implemented breakpoints.

### Don't:

- **Don't** restore the discarded unified studio with a bottom recording dock.
- **Don't** add context configuration UI or persisted source switches.
- **Don't** wash the workspace in blue, animate fake idle activity, or download UI fonts.
- **Don't** treat browser mocks as proof of microphone, Windows accessibility, GPU, or provider performance.
