# Dictation model selector screenshots

Captured from the implemented React UI at a 907 × 736 CSS-pixel viewport in dark mode. Tauri's official IPC/event mocks supplied backend state; the selector and status components are the real application code. The screenshots verify presentation, not microphone access, GPU availability, cloud authentication, or recognition quality. The hardware name and missing-key message are mock data.

- `ready-dark.png`: Qwen3-ASR selected, ready status, hardware label, and enabled recording control.
- `needs-attention-dark.png`: GLM-ASR selected with a simulated missing-key event, error guidance, Retry action, and disabled recording control.

No baseline screenshot was captured; the PR's before-state description is based on the base commit's source.
