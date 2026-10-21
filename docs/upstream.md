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
- Last reviewed upstream tip: `af52448667788ddb74ae9a9a4c4cd6f5fc9c9a09` (after v1.7.8;
  reviewed 2026-10-21). The previous checkpoint was `fec2bbc9e058c8f9e6667307f1430e8eb1918822` (v1.7.7).
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

### Review 2026-10-21: `fec2bbc..af52448`

Range: 16 commits, 14 non-merge. Every non-merge commit has a disposition below.
Upstream and SpeakFlow share no git merge base (SpeakFlow's history starts at
`bf07f14`), so ports are applied by hand to the working tree, never by merge.

| Commit | Subject | Disposition | Reason and evidence |
|:--|:--|:--|:--|
| [`145bf29`](https://github.com/sypsyp97/light-whisper/commit/145bf297cca2562b65fcea6cde3ef8c3922b8d9d) | fix(security): upgrade all KaTeX dependency paths | Ported | `katex` 0.18.2 with a pnpm override so `rehype-katex` no longer pulls 0.16.47; ported `MarkdownContentSecurity.test.tsx`. The upstream formal evidence note was not copied because `formal/` artifacts are hashed. |
| [`40f2dcb`](https://github.com/sypsyp97/light-whisper/commit/40f2dcb561a9f2616c70dc526a193f976169d93e) | refactor(app): harden settings and streamline runtime paths | Partly ported | Ported: `engine.json` writers (engine, endpoints, models directory, GPU idle) now run one locked read-modify-write, with the upstream concurrency tests renamed to `SPEAK_FLOW_*`. Deferred: `build_engine.py`, `release.sh` and the atomicity test changes (our `build_engine.py` already differs from the ledger baseline, and the engine archive was not rebuilt here); `app_state`, profile and `lib.rs` changes; the `qwen_hotword_service` refactor was skipped as behavior-neutral without evidence. |
| [`47064e2`](https://github.com/sypsyp97/light-whisper/commit/47064e24525b36b4b539d2c95cd6ab9d316931ab) | perf(asr): coalesce R2T2 final acoustic context | Evaluate separately | The Python change in `r2t2_native.py` is gated on native ABI 5 and the `confucius4_r2t2.coalesced_finish` option. It needs the patched audio.cpp runtime rebuilt; the benefit is unmeasured here. |
| [`c64c7f1`](https://github.com/sypsyp97/light-whisper/commit/c64c7f1) | perf(asr): reuse exact streaming prefixes and log-mel work | Evaluate separately | Native C++ patch and runtime rebuild required. Not verified here. |
| [`507ed50`](https://github.com/sypsyp97/light-whisper/commit/507ed50) | perf(asr): verify temporal drafts with scalar weight reuse | Evaluate separately | Native C++ patch and runtime rebuild required. Not verified here. |
| [`56d37cf`](https://github.com/sypsyp97/light-whisper/commit/56d37cf) | Refactor LLM model defaults and update API documentation | Evaluate separately (maintainer decision) | Changes the default Alibaba ASR model to `qwen-audio-3.1-asr-flash` and adds a new DashScope request body and response parser for `qwen-audio-3.*-asr-flash`. This changes a cloud default and the API contract, and it cannot be checked against DashScope here. The LLM default changes are not reviewed. |
| [`22a4128`](https://github.com/sypsyp97/light-whisper/commit/22a4128) | feat(llm): add OpenRouter provider and fix reasoning controls | Evaluate separately | Optional LLM provider feature outside ASR. |
| [`f644f3e`](https://github.com/sypsyp97/light-whisper/commit/f644f3e) | chore(jev): change default OpenRouter decisions model | Deferred | Depends on `22a4128`. |
| [`c651fda`](https://github.com/sypsyp97/light-whisper/commit/c651fda) | fix(llm): remove fixed output caps and harden provider fallback | Evaluate separately | LLM transport change; needs provider-level evidence. |
| [`7651eba`](https://github.com/sypsyp97/light-whisper/commit/7651eba) | test(llm): simplify the streaming expectation | Deferred | Test-only follow-up to `c651fda`. |
| [`f83acfd`](https://github.com/sypsyp97/light-whisper/commit/f83acfd) | fix(llm): discover DeepSeek image capabilities from metadata | Evaluate separately | LLM capability discovery outside ASR. |
| [`af52448`](https://github.com/sypsyp97/light-whisper/commit/af52448) | Add profile save notifications and related tests | Evaluate separately | UI feature that also touches history and profile code. |
| [`1e8cc10`](https://github.com/sypsyp97/light-whisper/commit/1e8cc10) | fix(windows): use full-resolution default window icon | Skipped | Upstream branding asset; SpeakFlow keeps its own icons. |
| [`69497ea`](https://github.com/sypsyp97/light-whisper/commit/69497ea) | chore(release): bump version to 1.7.8 | Skipped | SpeakFlow keeps its own version, signed updater key and release process. |

### SpeakFlow engine changes made in the same pass (not upstream)

- Qwen3-ASR 1.7B is selectable and downloadable again, through transcribe.cpp
  (`handy-computer/Qwen3-ASR-1.7B-gguf`, Q8_0, pinned revision). This reverses the
  v1.6 migration that mapped saved 1.7B selections to R2T2. Saved 1.7B selections
  are honored again; R2T2 caches are unaffected.
- Parakeet Unified EN 0.6B and Parakeet TDT 0.6B v2 are added as English-only local
  models (Q8_0, pinned revisions, SHA-256 checked by the production downloader). Both
  run through the shared transcribe.cpp server, so they use the same interim and
  final path as Qwen. Their license terms are recorded in `THIRD_PARTY_NOTICES.md`.
  NVIDIA's model card puts Parakeet Unified under the NVIDIA Open Model License, which
  conflicts with the transcribe.cpp model note (CC-BY-4.0). The NVIDIA card is the
  source the notices follow. Maintainer review is still needed before redistribution.
- Canary-Qwen 2.5B is added as an English-only local model (`handy-computer/canary-qwen-2.5b-gguf`,
  Q8_0, pinned revision `a4480da`, SHA-256 checked by the production downloader). It runs
  through the same shared transcribe.cpp server. NVIDIA's card and the GGUF metadata both
  state CC-BY-4.0.
- The local engine list is held in `LOCAL_ASR_ENGINES` in `paths.rs`. Contract tests in
  `test_local_engine_contract.py` keep it in step with the Python registry, the bundled
  entry point, the Rust readiness table and the TypeScript options.

## Local verification

Imported upstream evidence is historical provenance, not validation of this
modified checkout. Local results are recorded below. Live microphone/focus
behavior, packaged GPU inference and the deferred engine's performance require
separate checks.

### Results 2026-10-21

- Frontend: full `vitest run --exclude src/perf/** --maxWorkers=2`: 56 files and 455
  tests passed. `pnpm perf:fluidity`, `tsc --noEmit`, `pnpm lint`, `pnpm build` and
  `pnpm check:bundle` passed. `pnpm install --frozen-lockfile` passed.
  `pnpm audit --prod --audit-level high` reported no known vulnerabilities. The ported
  KaTeX security test passed (3 tests).
- Rust: `cargo test --all-targets --locked --no-fail-fast` with `SPEAK_FLOW_DATA_DIR` set
  to a fresh temporary directory, skipping only the documented hotword timing test:
  library 715 passed, 0 failed, 11 ignored; contract and archive-policy integration
  tests passed. `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`
  passed. One IPC replay timing test failed once in a run that overlapped the full
  frontend suite. It passed when rerun alone and in the next full run, so the failure
  is treated as load-sensitive.
- Python: `unittest discover` on `src-tauri/resources` passed 139 tests. `compileall`
  passed. `test_build_engine_atomicity` passed 26 tests, `test_release_ci` passed 6, and
  `test_ci_fail_fast` skipped its 2 platform-specific tests. `check_formal.py
  --inventory-only` passed.
- Real local recognition, on Windows with an RTX 4070 and a Ryzen 7 5700X, using the
  production downloader with SHA-256 checks and the 11-second JFK sample: every
  transcript matched the reference sentence. Vulkan warm calls were about 60-90 ms
  for both Parakeet models, 220-320 ms for Qwen3-ASR 1.7B, and 120-210 ms for
  Qwen3-ASR 0.6B. CPU-only calls were about 0.70 s for each Parakeet model and about
  1.65 s for Qwen3-ASR 0.6B. The CUDA provider was not installed, so CUDA was not
  exercised. The first Parakeet Unified call after writing its 731 MB file took about
  11 s once and did not recur in fresh processes.
- Not verified here: microphone capture, the packaged app and its UI, live speech in
  the interim loop, long recordings, CUDA inference, and regenerated GPU evidence in
  `formal/evidence` for the new engines.

### Results (previous checkpoint)

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
