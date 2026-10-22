# SpeakFlow UI redesign — 22 October 2026

## Research method

Two research tasks ran on GPT-6 Luna with `reasoningEffort: xhigh`, as requested. Both used TinyFish search and fetch. They reviewed public product documentation, repositories, and available app screenshots. They did not install competitor apps. Proposed colors and spacing are design choices, not measured competitor tokens.

## Sources and useful patterns

- **Superwhisper:** the strongest screenshot reference. Its [recording window](https://superwhisper.com/docs/get-started/interface-rec-window), [History](https://superwhisper.com/docs/get-started/history), and [Configuration](https://superwhisper.com/docs/get-started/essential-settings) guides show charcoal app surfaces, neutral navigation selection, and selective blue cues. The recording UI keeps waveform, stop, and cancel close together. Adapted: neutral layering, restrained selection, and an unmistakable recording control.
- **Wispr Flow:** [first dictation](https://docs.wisprflow.ai/articles/6409258247-starting-your-first-dictation) and [Flow Bar](https://docs.wisprflow.ai/articles/5002934560-why-is-the-wispr-bar-is-not-appearing-or-disappearing) documentation establish hotkey-first dictation and compact recording feedback. Available third-party screenshots were mostly macOS; Windows styling parity was not assumed. Adapted: keep hotkey guidance and activity feedback close to the microphone.
- **Handy:** the [official repository](https://github.com/cjpais/Handy) and [Windows screenshot walkthrough](https://gigazine.net/gsc_news/en/20250930-handy/) support a shallow sidebar and grouped settings. The walkthrough is from 2025 and may not match the latest styling. Adapted: retain simple destinations and task-focused settings groups.
- **OpenWhispr:** the [settings reference](https://docs.openwhispr.com/help/customise/settings-reference), [background behavior](https://docs.openwhispr.com/platform/running-in-the-background), and [meeting recording](https://docs.openwhispr.com/help/meetings/record-a-meeting) docs describe grouped configuration and distinct recording states. These were primarily text evidence. Adapted: preserve clear setup, readiness, and processing states while simplifying their visual presentation.
- **VoiceInk:** [General Settings](https://tryvoiceink.com/docs/settings-general) and [Transcription History](https://tryvoiceink.com/docs/transcription-history) describe theme choices and searchable history with transcript details. Adapted: keep appearance controls and existing history recovery actions visible.
- **HEX:** the [current repository](https://github.com/anomalyco/hex) and [public site](https://hex.kitlangton.com/) provided conflicting platform signals during research. No dependable current settings screenshot was found. Used only as an interaction reference, not a verified Windows visual reference.

“Open Whisper” can mean [OpenWhisper](https://www.openwhisper.org/) or OpenWhispr. The former describes a floating microphone pill; the richer workspace documentation belongs to the latter. They were treated as separate products.

## Applied design

The approved direction is design 6, **Workspace Dashboard**, recorded in `.impeccable/mocks/decision/workspace.html` and `workspace-preview.png`. The earlier color-only pass and unified transcript studio with bottom recording dock are superseded.

The implemented shell uses a 208px desktop sidebar, black/charcoal surfaces, and restrained blue actions. A horizontal recording/readiness/model panel leads the dashboard. Recent dictations includes an editable latest transcript and real recent results; a quieter Your setup column contains engine, shortcut, configuration, and conditional Quick Start guidance. Settings uses a vertical section index and open form sections. Compact widths tighten columns, then stack recording and setup; narrow widths use bottom navigation. Native typography, light/system themes, existing integrations, and recovery actions remain. No fake usage statistics or transcript data were added. `DESIGN.md` contains the normative tokens; `.impeccable/design.json` extends them with component examples and responsive/motion metadata.

## Context-awareness research and implementation

A further GPT-6 Luna / xhigh research task read the user's [Superwhisper context documentation](https://superwhisper.com/docs/common-issues/context), [custom mode controls](https://superwhisper.com/docs/modes/custom#context-awareness), and [built-in modes](https://superwhisper.com/docs/modes/built-in). These describe independently configurable selected text, application context, and clipboard sources per mode. They informed source behavior; SpeakFlow follows the user's subsequent choice to make all three native runtime defaults and remove context configuration UI.

Application text, selected text, and clipboard are all enabled by `ContextSources::default()`. Recording startup takes these runtime defaults rather than reading legacy profile switches, so saved false fields cannot disable the sources. The profile no longer persists context preferences; there is no context settings command, source switch, per-mode UI, or toolbar entry.

Application/selection capture starts with recording and binds to the original foreground target. It checks foreground-window identity before and after capture, rejects SpeakFlow's own process and password fields, stays within the focused process, and verifies that focus has not changed. Windows accessibility availability determines what can be captured; missing or unsupported content is omitted. Each source is trimmed and limited to 4,000 characters, then wrapped as labeled reference context with XML CDATA escaping for the existing AI pipeline.

Clipboard text is read at startup and refreshed at completion only when the recording session is still current. Internal selection-marker clipboard content is excluded. There is no continuous monitoring or three-second freshness filter. Completion waits up to 300ms for application/selection capture before continuing with available context. Context is not saved to history, and the user's full name and computer name are not collected. Provider sharing and local/cloud behavior are documented in the updated README/privacy material. These availability and lifecycle safeguards remain even though the source controls were removed.

## Visual verification ledger

The final implementation follows Workspace Dashboard composition using the real SpeakFlow brand, copy, model/device status, recovery actions, and transcript editor. No UI revisions followed the final captures.

| Comparison | Result |
| --- | --- |
| Palette | Black/charcoal surfaces with restrained blue; native Windows fonts and existing semantic status colors. |
| Composition | 208px desktop sidebar, top horizontal recording panel, Recent dictations, and quieter Your setup column. |
| Transcript | Editable latest result and recent text rows; result capture recorded. |
| Recording | Actual waveform/spinner states, shortcut and readiness together; model controls lock during activity. |
| Context UI | Source controls and context navigation absent; runtime defaults enable all three sources. |
| Responsive behavior | Local Playwright Tauri mocks captured desktop 1440 x 900, reference 1586 x 992, compact 900 x 640, and narrow 390 x 844. |
| Settings | Vertical section navigation and open forms on desktop; horizontal scrollable navigation at narrow widths. |
| Browser errors | No page errors reported in the local mock flow. |

Final review captures are `.impeccable/review/desktop.png`, `mobile.png`, `compact.png`, and `transcript-result.png`. Test-only Tauri responses were injected before browser initialization; supporting mocks and captures also live outside the repository under the local UI QA cache. These captures establish layout and mocked interaction behavior. They do not establish live microphone capture, Windows UI Automation compatibility in every third-party app, GPU/runtime behavior, or provider response quality.

Shared preview snapshot attempts failed twice with the raw diagnostic `Preview automation snapshot timed out after 15000ms.` Local Playwright captures provided the visual fallback; this remains a shared-preview limitation.

The completed frontend `pnpm check` passed: 522 unit/component tests and one fluidity test, whitespace, lint, TypeScript/Vite production build, and bundle budgets. Backend tests passed with 775 passed, 12 ignored, and one existing performance test filtered out. Rust formatting and all-targets Clippy passed. The final independent finish review returned SHIP with no material findings. These checks support the tested implementation and do not remove the native-runtime evidence limits above.
