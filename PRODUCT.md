# SpeakFlow

<!-- impeccable:product-schema 1 -->

## Platform

web

The React interface runs inside a Tauri desktop application for Windows 10/11 x64; it is not a hosted web service.

## Product Purpose

Turn speech into text in the user's active Windows application without requiring them to switch windows. Hold a hotkey, speak, and release, or use toggle recording. Success means the user can tell when dictation is ready, record, and receive usable text where they are working.

## Users

People dictating into their everyday Windows applications. The user confirmed a quiet, focused everyday workspace.

## Positioning

Windows dictation with a choice of local or cloud recognition, integrated with the active application through a hotkey. Basic local dictation does not require an LLM account; optional AI processing adds rewriting, translation, and assistants. These are product capabilities, not claims of exclusivity, recognition accuracy, or guaranteed latency.

## Operating Context

- Users focus the application they want to type into and dictate using the configured hotkey or recording button. Both hold-to-speak and toggle recording are supported.
- The Dictation tab provides model selection, actual model readiness and activity, recording controls, and the latest transcript. Settings configures microphones, hotkeys, local downloads, cloud credentials, regions, and AI providers.
- Local recognition processes audio on the PC; initial runtime and model downloads require network access. Cloud recognition sends audio to the selected provider.
- Optional AI processing can send text and enabled context to configured services. History and audio saving are opt-in and stored locally; audio saving requires history.

## Capabilities and Constraints

Preserve local and cloud recognition engines, AI providers, dictation, translation, voice and selection assistants, subtitles, history, correction rules, and per-app settings. Tauri, React, TypeScript, and Rust are the existing stack, with Python and native runtimes for local recognition.

The interface is English-only, including built-in prompts and errors; multilingual recognition and user-authored content remain supported. Keep provider integrations and third-party license notices intact. Model downloads, credential setup, loading, and failures must be distinguishable from readiness; a selected model is not necessarily ready to dictate. Prevent model switching during recording, transcription, or active downloads.

## Brand Commitments

Keep the SpeakFlow name and the confirmed quiet, focused desktop experience. Preserve the established interface identity; visual implementation details belong in `DESIGN.md`.

## Evidence on Hand

- `README.md`: user workflows, recognition engines, optional AI features, and data-handling descriptions.
- `docs/privacy.md`: network requests, local storage, and third-party service policies.
- `docs/development.md` and `docs/r2t2-native.md`: supported environment, implementation details, and documented runtime limitations.
- `DESIGN.md`: established interface identity and design tokens.
- `src/` and its tests: implemented UI behavior and regression coverage. Browser checks with mocked desktop APIs are layout evidence, not proof of microphone, GPU, or recognition performance.
- `formal/README.md` and `formal/COVERAGE.md`: verification scope and assumptions, not a proof of every external dependency or recognition result.

No customer testimonials, independent accuracy benchmarks, or comparative performance claims are established by this product record.

## Product Principles

1. Keep dictation in the user's existing workflow rather than requiring a separate writing destination.
2. Make the selected model and its real readiness, activity, and recovery needs clear before recording.
3. Preserve local/cloud choice and make the consequences for setup and data handling explicit.
4. Keep basic dictation usable without optional AI services; retain advanced capabilities without making them prerequisites.
5. Improve the focused everyday workspace without removing existing integrations or user controls.
