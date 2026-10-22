# Local ASR performance experiment — 2026-10-22

The shared FireRedVAD runtime now uses at most two CPU inference threads by
default, with one thread on single-core machines or when CPU count is unknown.
Inter-op parallelism remains one. This affects Qwen3-ASR, Parakeet,
Canary-Qwen, Cohere Transcribe, and the VAD used by R2T2. Only the two Qwen
models below were measured through speech detection and decoding.

## Environment and workload

- Source baseline: `94ef4f6876db4e692bf1934e2bbde6e3f3f39eb0`; the candidate
  changes only the VAD thread budget. Benchmark configurations explicitly select
  one or two threads against the same source and model weights.
- Windows, RTX 4070 (12 GB), 16 logical CPU cores; transcribe-cpp 0.1.3,
  ONNX Runtime 1.24.1, NumPy 1.26.4. The existing development environment had
  the Vulkan native provider, so these are Vulkan results, not CUDA results.
- Published native runtime wheel, Q8_0 weights, F16 KV cache, 32,768-token
  context. Model weights remain resident. One session is active at a time.
- Official [JFK sample](https://github.com/handy-computer/transcribe.cpp/blob/v0.1.3/samples/jfk.wav):
  11 seconds, 16 kHz mono PCM16, SHA-256
  `59dfb9a4acb36fe2a2affc14bacbee2920ff435cb13cc314a08c13f66ba7860e`.
  Additional workloads are its first five seconds, three concatenated copies
  (33 seconds), and two seconds of digital silence. These are English fixtures,
  not an independent multilingual accuracy corpus.
- Ten samples per configuration per workload, alternating configuration order.
  Each session warms all clips before measurement. Timing includes speech
  detection, trimming and decoding. It excludes loading, warmup, audio file
  decoding, IPC, recording and UI. No profilers run inside the reported timings.

## Results

Times below are median milliseconds with min–max ranges in parentheses.

Qwen3-ASR 1.7B Q8_0:

- 5 s: 113.82 (112.87–115.49) → 109.85 (109.27–111.51), **3.48% lower latency**.
- 11 s: 227.13 (225.96–240.54) → 217.99 (217.27–220.04), **4.02% lower latency**.
- 33 s: 607.12 (602.58–638.24) → 581.01 (576.22–598.59), **4.30% lower latency**.
- Silence: 6.55 (6.39–7.00) → 5.34 (5.04–5.58); decoding is skipped.

Qwen3-ASR 0.6B Q8_0:

- 5 s: 72.41 (70.97–74.80) → 68.77 (67.15–69.87), **5.03% lower latency**.
- 11 s: 139.76 (137.83–143.45) → 130.44 (128.55–132.31), **6.66% lower latency**.
- 33 s: 366.04 (362.62–386.49) → 340.50 (336.85–352.73), **6.98% lower latency**.
- Silence: 6.56 (6.51–7.04) → 5.15 (4.99–8.24); decoding is skipped.

VAD alone on the 11-second clip fell from 33.49 to 24.16 ms in the 1.7B
comparison. All 160 measured requests completed without errors. Every
transcript, detected language and VAD boundary matched the corresponding
one-thread baseline. Silence produced no transcript. A separate ten-run probe
also produced bit-identical VAD probabilities for one, two and four threads.

The tradeoff is a second CPU inference thread during VAD. Four threads were
faster in the isolated probe, but were not selected: two threads gave a clear
benefit with half that thread budget and less variation in that probe.

## Rejected context experiment

Reducing Qwen's context from 32,768 to 4,096 tokens reduced the runtime's
reported maximum KV budget from 3,758,096,384 to 469,762,048 bytes. That was
**not an observed allocation saving**: five alternating runs showed essentially
the same GPU memory usage, and 1.7B latency was 231.42 vs 231.43 ms. Smaller
contexts also reduce maximum input duration. The application retains its
original context and F16 cache; no performance claim relies on this experiment.
The runtime's [input-limit documentation](https://github.com/handy-computer/transcribe.cpp/blob/v0.1.3/docs/models/qwen3-asr.md)
describes that context/duration relationship.

## Reproduce and verify

With the project dependencies and selected model already available:

```powershell
uv run python scripts/benchmark_local_asr.py --engine qwen3-asr-1.7b --backend vulkan --wav tmp/model-perf/jfk-short.wav --wav tmp/model-perf/jfk.wav --wav tmp/model-perf/jfk-long.wav --wav tmp/model-perf/silence.wav --repetitions 10 --output tmp/model-perf/qwen17-verified.json
```

Repeat with `--engine qwen3-asr-0.6b` and another output filename. The default
comparison is `--vad-threads 1 2`. To investigate context independently use
`--contexts 32768 4096 --vad-threads 1`. The JSON records all timings, output
text, speech boundaries, context limits, input/source hashes and environment.
GPU memory snapshots are whole-device readings and may include other apps.
A transcript/language/boundary mismatch makes the benchmark exit unsuccessfully
after saving the report. The original run reports remain in local
`tmp/model-perf/qwen{17,06}-verified.json`.

```powershell
uv run python -m unittest discover -s src-tauri/resources -p 'test_*.py'
git diff --check
```

All 151 resource tests passed. New checks cover CPU-count fallback and
probability/boundary preservation using the actual bundled ONNX model.
No model weights, quantization, speech thresholds or transcript correction
rules changed. Accuracy improvement, CUDA speedup, other model families'
full decoding speed, and full desktop application latency remain unmeasured.

Follow-up: [CUDA/Vulkan comparison, decoder timings and server transport](local-asr-backends.md).
