# Upstream review ledger

SpeakFlow started from [sypsyp97/light-whisper](https://github.com/sypsyp97/light-whisper).
It is independently branded and released; upstream work still needs compatibility
review. Copyright, GPL and third-party notices stay intact.

Use the project skill `/skill:upstream-check` to review new updates. Run `/reload`
in Pi after first adding the skill. Reviews do not merge code automatically.

## Review checkpoint

- Source URL: `https://github.com/sypsyp97/light-whisper.git`
- Source branch: `main`
- Shared base: `81b6da0b3942d518366706c5d1854a6059df9225`
- SpeakFlow starting commit: `3f2ae5a` (version 1.0.5).
- Last reviewed upstream tip: `fec2bbc9e058c8f9e6667307f1430e8eb1918822` (v1.7.7).
- This is a **review checkpoint, not an integrated ancestor**. Partial and deferred
  items below must be reconsidered alongside new commits on future checks.

## Decisions

### Ported: async ownership and recovery (`c43606b`, adapted)

Source: [`c43606ba565b4185c392bfe09e2f15005afe4b6e`](https://github.com/sypsyp97/light-whisper/commit/c43606ba565b4185c392bfe09e2f15005afe4b6e).

- Serialize microphone monitor ownership across settings instances; ignore retired
  tests/effects and compensate for late starts.
- Keep pending dictations separate, bind them to their original target window,
  drain after terminal paths, and offer original-text recovery after failed or
  unsafe pastes. Retain SpeakFlow's opt-in external correction learning.
- Release shared history-audio leases in the correct SQL order, preserving audio
  still used by another operation.
- Bound the entire recognition command, including lock/write/flush/read, and
  invalidate the child process on timeout or cancellation.
- Distinguish missing, ready and corrupt caches; offer explicit repair rather than
  repeatedly trying to load invalid weights. Preserve our separate engine-download
  flow and English user-facing messages.
- Accept explicit captured providers for assistant-key IPC/backend operations.
- Port applicable regression tests, formal companions, replay fixtures, provenance
  checks and CI wiring. Adapt registry anchors to the code actually present here.

The upstream assistant credential editor and its UI-specific tests/models are
not applicable: SpeakFlow's redesigned SettingsPage no longer contains that
editor. Restoring an old settings screen would undo our deliberate divergence.
The credential storage/snapshot contracts and explicit-provider API tests remain.

### Ported: provider endpoint resolution (`8e97604`, partial)

Source: [`8e9760428a0f0546b22c6a5a3b51bacea074ed07`](https://github.com/sypsyp97/light-whisper/commit/8e9760428a0f0546b22c6a5a3b51bacea074ed07).

Resolve assistant, selection, validation and screen-vision endpoints directly for
their provider instead of mutating a copy of the active polish configuration.
This avoids carrying the wrong active-provider overrides into an independent
role. Port the endpoint-resolution regression matrix; preserve our existing
provider URLs, keyring identity, OAuth and updater settings.

The rest of this large commit remains deferred below. In particular, this partial
port does not migrate engines or add custom decision-model configuration fields.

### Evaluated separately: shared Qwen/R2T2 engine and packaging

Source: `8e97604` and the
[v1.7.7 release](https://github.com/sypsyp97/light-whisper/releases/tag/v1.7.7).

1. **GPU support — retain Vulkan for now.** Upstream removes Vulkan and the old
   transcribe.cpp provider, leaving CPU and NVIDIA CUDA. The development PC has
   an NVIDIA GeForce RTX 4070 with 12 GB VRAM, so it is compatible with CUDA, but
   that does not justify removing acceleration for non-NVIDIA users. Keep the
   current engine unless a separate, explicit compatibility decision approves
   this migration.
2. **Model migration/performance — promising, not validated here.** Qwen requires
   a different approximately 1.15 GB GGUF package instead of our approximately
   850 MB package. Upstream preserves old caches, so disk use can grow by the
   full new download. Published tests on an RTX 4070 SUPER report lower Qwen
   inference times on two Chinese fixtures; R2T2 full-model improvements are much
   smaller, and broad recognition-quality parity is not established. Those are
   upstream measurements, not end-to-end SpeakFlow benchmarks. Before adoption,
   compare English/multilingual recordings, long audio and last-word completeness,
   model switching, first-caption/final latency, CPU fallback and real CUDA
   inference against our current packaged engine. Do not change the displayed
   model size until the actual model migration ships.
3. **Installer size — keep our split packaging.** Upstream's installer is
   651,970,761 bytes (~652 MB). SpeakFlow 1.0.5 has a 28,125,883-byte installer
   (~28 MB) plus a 654,843,260-byte engine downloaded only for local recognition;
   models are additional in both comparisons. Our initial local-engine download
   totals about 683 MB before models, so this is not a claim that total local-ASR
   setup is smaller. The split keeps cloud-only installs and app-only updates
   small. Runtime dependency pruning may be useful in a separate engine build,
   but it is not a reason to restore a bundled 652 MB installer or drop Vulkan.

Verdict: retain the current ASR runtime, model package and split distribution.
Evaluate a shared-runtime candidate separately only with measured compatibility
and recognition results. No native migration or model re-download is triggered
by this port.

### Deferred or skipped commits/features

- `8e97604`: configurable decision-model API URL/model and other engine-related
  work remain feature candidates, not required reliability fixes. The existing
  decision settings component is not mounted in our redesigned SettingsPage;
  adding configuration should be a separate UX/API task, not restoration of
  upstream's old settings screen.
- [`1158104`](https://github.com/sypsyp97/light-whisper/commit/11581047802fa8a8ef735fe97fc0acd3fe817f81): the 1.15 GB model label belongs to
  the deferred new model; applying it alone would misdescribe our current download.
- `6ef1dfbb318002f1578ac31e5f82b16312dabbc2` and
  `fec2bbc9e058c8f9e6667307f1430e8eb1918822`: upstream release/version commits;
  keep SpeakFlow's own version, signed updater public key and release process.

## Local verification

Imported upstream evidence is historical provenance, not validation of this
modified checkout. Local results are recorded below. Live microphone/focus
behavior, packaged GPU inference and the deferred engine's performance require
separate checks.

### Results

- Frontend: 533 tests passed with `vitest run --exclude src/perf/** --maxWorkers=2`;
  the separate fluidity test passed. ESLint, TypeScript/Vite build and whitespace
  checks passed. Earlier full-parallel runs hit existing 5-second UI timeouts
  under concurrent compilation; the bounded-worker run passed all test files.
- Rust: 714 all-target tests passed in isolated data directories; 11 integration/
  fixture cases were ignored and the one CI-excluded hotword timing test was
  filtered out. Formatting and all-target Clippy (`-D warnings`, two build jobs)
  passed. A first Clippy attempt hit an MSVC object timestamp error; its retry
  succeeded without source changes.
- Python resources: 127 tests passed. Engine-build atomicity: 20 passed; release
  CI: 5 passed; updater manifest: 1 passed. Two PowerShell fail-fast regressions
  were platform-skipped.
- Formal: 126 commands / 18 events / 16 lifecycle contracts / 11 protocols passed
  inventory and request-snapshot checks. Six ported TLC models passed; all 13
  fault controls and the separate repair reachability witness produced their
  expected violations. Provenance/portability: 8 passed; wiring/isolation: 4
  passed; verifier regressions: 11 passed. Both microphone source-mutation checks
  passed their baseline and rejected the deliberately broken variant.
- Pi's actual skill loader discovers `upstream-check` with no diagnostics. The
  dedicated upstream review ref fetch was exercised; origin remains SpeakFlow.
- Not rerun: the entire pre-existing TLC/Lean suite. No installer was rebuilt,
  installed or released, and no changes were pushed.

### Test-isolation incident

The initial imported Rust fixture run still used `LIGHT_WHISPER_DATA_DIR`, which
SpeakFlow ignores. Three child fixtures therefore reached the default installed
application data directory; history tests raced in the same database and a
retention test may have deleted older saved history. Its pre-run contents were
not backed up, so prior history loss cannot be determined or reversed from the
available evidence. The user was notified during this task.

All three child overrides now use `SPEAK_FLOW_DATA_DIR`. A source regression
checks those exact fixtures, the local skill requires a side-effect/isolation
audit before imported tests run, and the successful Rust rerun used a fresh
temporary `SPEAK_FLOW_DATA_DIR` for the entire suite. Never point test commands
at installed-app history. This correction does not restore any earlier data.
