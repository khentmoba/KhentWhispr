# Theme default screenshots

Captured from the real React UI served by the Vite dev server at a 920 x 740 CSS-pixel viewport, rendered at 2x device scale. The Tauri IPC bridge was stubbed in the page so the application shell renders in a browser; the theme resolution, the `useTheme` hook and the Appearance settings section are the real application code. The screenshots verify theme selection, not microphone access, GPU availability, cloud authentication or recognition quality. Engine status and the model name are stub data.

No baseline screenshot was captured for the base commit before this change; the `before` images are the base commit `5d31526` with no stored theme choice.

| File | Commit | Stored theme | Result |
| --- | --- | --- | --- |
| `before.png` | `5d31526` base | none | App opens in the light theme |
| `before-appearance.png` | `5d31526` base | none | Appearance shows System selected |
| `after.png` | this branch | none | App opens in the dark theme |
| `after-appearance.png` | this branch | none | Appearance shows Dark selected |
| `after-stored-light.png` | this branch | `light` | An explicit light choice is still honoured |
