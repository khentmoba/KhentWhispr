# Qwen3-ASR backend comparison — 2026-10-22

Vulkan was faster than CUDA for Qwen3-ASR 1.7B Q8_0 on this RTX 4070.
The server now accepts a model-specific backend preference so this result
can be used without assuming the same winner on every GPU or model.
The automatic backend order remains CUDA, Vulkan, CPU on NVIDIA systems.

## Runtime and workload

The source baseline is `94ef4f6876db4e692bf1934e2bbde6e3f3f39eb0` plus the
two-thread FireRedVAD change in [the first experiment](local-asr-performance.md).
Both backends used the same installed CUDA-provider build: transcribe.cpp
0.1.3, commit `a94e021`, public header hash `86b16dd97ad1cb58`. No dependencies
or weights were installed. The native bundle was loaded read-only.

Hardware: RTX 4070 12 GB, driver 610.88, Ryzen 7 5700X (16 logical cores),
Windows. Q8_0 weights, F16 KV cache, 32,768-token context, two VAD threads and
native default inference threads (`n_threads=0`) were fixed. Inputs were the
same 5-, 11- and 33-second JFK fixtures and two-second silence from the first
experiment. They are English fixtures, not an accuracy corpus.

Five trials per backend per input alternated backend order. Each trial used
a fresh process and warmed all inputs before measuring VAD plus decoding.
An earlier same-process comparison showed Vulkan degrading after CUDA ran.
Those contaminated timings were discarded. GPU allocations or other backend
state surviving model closure are a suspected cause, not a validated native
runtime defect. The benchmark now isolates multi-backend comparisons in
subprocesses automatically.

## Inference results

Median milliseconds, followed by min–max ranges:

- 5 s: CUDA 125.70 (125.23–127.89); Vulkan 109.57 (108.52–109.74).
- 11 s: CUDA 253.90 (253.45–258.80); Vulkan 216.60 (215.43–217.37).
- 33 s: CUDA 694.93 (690.65–700.62); Vulkan 570.78 (570.26–573.53).
- Silence: CUDA 5.11 (4.97–5.31); Vulkan 5.16 (5.10–5.58). No meaningful gain.

All 40 measured requests completed without errors. Output text, detected
language and VAD boundaries matched exactly across both backends and all runs.

Native timing medians for the 11-second input locate most of the difference
in text decoding:

- Mel/features: CUDA 5.52 ms; Vulkan 5.25 ms.
- Audio encoder: CUDA 13.16 ms; Vulkan 11.98 ms.
- Text decoder: CUDA 209.93 ms; Vulkan 171.93 ms.

These use the runtime's existing counters. Its `load_ms` field describes model
loading and must not be added to per-request latency. Model loading, warmup,
file decoding, IPC, recording and UI are outside these request timings.

## Server transport verification

A separate check launched the updated Python server and sent PCM16 base64
through its actual stdin/stdout JSON protocol. Five fresh processes per backend
each warmed all inputs and measured each once. This includes request JSON
serialization, pipes, server dispatch, inline-audio decoding, VAD, inference
and parsing the response. Recording, Rust-side base64 encoding, desktop UI and
process/model startup are excluded. Every response's actual backend was checked.

- 5 s: CUDA 130.67 ms (126.28–135.83) → Vulkan 110.86 ms (109.81–113.48),
  **15.16% lower latency**.
- 11 s: CUDA 262.54 ms (256.42–274.57) → Vulkan 221.22 ms (219.75–251.52),
  **15.74% lower latency**.
- 33 s: CUDA 727.72 ms (709.86–782.61) → Vulkan 586.73 ms (583.13–603.61),
  **19.37% lower latency**.

All 40 measured server requests completed with matching text, languages,
speech segment counts and request IDs. Silence remained empty. Median overhead
beyond VAD and inference was 3.89 ms for the 11-second Vulkan input. Text
decoding remains the primary bottleneck, rather than transport.

## Rejected thread tuning

Five alternating CUDA runs per configuration tested default, one and four
native inference threads on all four inputs. For the 11-second clip:

- Default: 254.30 ms (252.73–263.18).
- One: 275.43 ms (274.26–284.39), slower.
- Four: 257.08 ms (252.28–260.14), within default variation.

No inference-thread change was retained. No additional context, quantization,
sampling or decoding changes were introduced.

## Selecting a measured backend

For an engine rebuilt from this source, set this variable before launching
the app or engine, after fully exiting any existing app instance:

```powershell
$env:SPEAK_FLOW_ASR_BACKEND_QWEN3_ASR_1_7B = "vulkan"
uv run python src-tauri/resources/engine.py serve --engine qwen3-asr-1.7b
```

Supported preferences are `auto`, `cuda`, `vulkan`, `cpu`, ignoring case and
surrounding whitespace. `auto` preserves the device-dependent default order.
The model-specific variable overrides the optional global
`SPEAK_FLOW_ASR_BACKEND`. Model suffixes uppercase the engine ID and replace
hyphens and dots with underscores. These preferences apply to models served
by `qwen3_asr_server.py`, not R2T2 or cloud providers.

The preference is tried first, then existing fallbacks without duplicates.
Check the reported `backend` to verify the actual selection. Invalid preferences
produce a clear initialization error. No persistent environment settings or
installed app files were changed; the installed engine needs a rebuild to
read these new preferences.

The loader now also owns a model immediately after construction, so a failed
session closes it before another backend is tried. A regression test reproduced
the missing cleanup before the fix and passed afterward.

## Reproduce and verify

With a CUDA-capable native provider available:

```powershell
uv run python scripts/benchmark_local_asr.py --engine qwen3-asr-1.7b --backend vulkan cuda --vad-threads 2 --wav tmp/model-perf/jfk-short.wav --wav tmp/model-perf/jfk.wav --wav tmp/model-perf/jfk-long.wav --wav tmp/model-perf/silence.wav --repetitions 5 --output tmp/model-perf/round2-qwen17-isolated.json
```

To use an existing compatible Windows bundle without installing another
provider, set `TRANSCRIBE_LIBRARY` to its `_native/transcribe.dll` and pass
`--dll-directory` pointing to `_internal`, containing the CUDA runtime DLLs.
The binding checks native version and ABI at import. Confirm the bundle's
contract matches the binding header before using this override.

Use `--backend cuda --n-threads 0 1 4` for thread tuning. JSON reports include
individual samples, text, stage timings, source/input hashes, native commit and
library path. This run's reports remain under local
`tmp/model-perf/round2-qwen17-{isolated,threads}.json`; the pipe check and scratch
harness are `tmp/model-perf/{round2-server.json,benchmark_server.py}`.

All **155** resource tests passed, including preference precedence, default
fallback, invalid-preference and failed-session cleanup checks:

```powershell
uv run python -m unittest discover -s src-tauri/resources -p 'test_*.py'
git diff --check
```

These results support Vulkan on this machine and fixtures. They do not
establish a universal backend winner, an accuracy improvement, or a full
desktop/microphone latency improvement.
