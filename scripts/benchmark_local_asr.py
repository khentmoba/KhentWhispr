"""Compare resident transcribe.cpp sessions on identical 16 kHz mono WAVs.

Example: python scripts/benchmark_local_asr.py --engine qwen3-asr-1.7b
    --wav tmp/jfk.wav --vad-threads 1 2 --output tmp/asr-benchmark.json
Timing includes VAD and decoding, excludes model loading, and follows a warmup
for each session. Configurations alternate order on successive repetitions.
"""

import argparse
import dataclasses
import hashlib
import itertools
import json
import os
import platform
from pathlib import Path
import statistics
import subprocess
import sys
import tempfile
import time

import soundfile as sf

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src-tauri/resources"))
from firered_vad import FireRedVad
from hf_cache_utils import TRANSCRIBE_CPP_ASR_MODELS, find_hf_snapshot_file


def gpu_memory_mib():
    try:
        result = subprocess.run(
            ["nvidia-smi", "--query-gpu=memory.used", "--format=csv,noheader,nounits"],
            capture_output=True, text=True, check=True, timeout=3,
            creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0),
        )
        return [int(value) for value in result.stdout.splitlines()]
    except (OSError, subprocess.SubprocessError, ValueError):
        return None


def save_report(report, output):
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    for key, by_clip in report["results"].items():
        for name, samples in by_clip.items():
            timings = [sample["total_ms"] for sample in samples]
            print(key, name, "median/min/max ms:", statistics.median(timings), min(timings), max(timings))
    baseline = next(iter(report["results"].values()))
    fields = ("text", "language", "vad_segments")
    if any(tuple(sample[field] for field in fields) !=
           tuple(baseline[name][0][field] for field in fields)
           for by_clip in report["results"].values() for name, samples in by_clip.items() for sample in samples):
        raise SystemExit("Transcript/language/audio changed; inspect the saved report before accepting a candidate")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine", choices=TRANSCRIBE_CPP_ASR_MODELS, required=True)
    parser.add_argument("--backend", nargs="+", default=["auto"], choices=["cuda", "vulkan", "cpu", "auto"])
    parser.add_argument("--wav", type=Path, action="append", required=True)
    parser.add_argument("--contexts", type=int, nargs="+", default=[32768])
    parser.add_argument("--vad-threads", type=int, nargs="+", default=[1, 2])
    parser.add_argument("--n-threads", type=int, nargs="+", default=[0])
    parser.add_argument("--dll-directory", type=Path, action="append", default=[])
    parser.add_argument("--repetitions", type=int, default=5)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--trial", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    if (args.repetitions < (1 if args.trial else 5)
            or any(value <= 0 for value in args.contexts + args.vad_threads)
            or any(value < 0 for value in args.n_threads)):
        parser.error("Use at least five repetitions and positive context sizes/thread counts")
    configurations = {f"{backend}-ctx{context}-vad{threads}-cpu{cpu_threads}":
                      (backend, context, threads, cpu_threads)
                      for backend, context, threads, cpu_threads in itertools.product(
                          args.backend, args.contexts, args.vad_threads, args.n_threads)}
    if len(args.backend) > 1 and not args.trial:
        # Switching from CUDA to Vulkan in one process distorted measured latency.
        report = None
        with tempfile.TemporaryDirectory(prefix="speakflow-asr-bench-") as scratch:
            for repetition in range(args.repetitions):
                order = list(configurations)
                if repetition % 2:
                    order.reverse()
                for key in order:
                    backend, context, threads, cpu_threads = configurations[key]
                    trial_output = Path(scratch) / "trial.json"
                    command = [sys.executable, str(Path(__file__).resolve()),
                               "--engine", args.engine, "--backend", backend,
                               "--contexts", str(context), "--vad-threads", str(threads),
                               "--n-threads", str(cpu_threads), "--repetitions", "1",
                               "--trial", "--output", str(trial_output)]
                    for path in args.wav:
                        command.extend(["--wav", str(path)])
                    for path in args.dll_directory:
                        command.extend(["--dll-directory", str(path)])
                    subprocess.run(command, check=True)
                    trial = json.loads(trial_output.read_text(encoding="utf-8"))
                    if report is None:
                        report = {**trial, "results": {}, "sessions": {},
                                  "configurations": configurations, "backend": args.backend,
                                  "trial_isolation": "process"}
                    report["contexts"].update(trial["contexts"])
                    for name, samples in trial["results"][key].items():
                        report["results"].setdefault(key, {}).setdefault(name, []).extend(samples)
                    report["sessions"].setdefault(key, []).extend(trial["sessions"][key])
                    print("Completed trial", repetition + 1, key, flush=True)
        save_report(report, args.output)
        return
    config = TRANSCRIBE_CPP_ASR_MODELS[args.engine]
    model_path = find_hf_snapshot_file(config["repo_id"], config["filename"])
    if not model_path:
        parser.error("The selected model must already be downloaded")
    clips = []
    for path in args.wav:
        audio, rate = sf.read(path, dtype="float32", always_2d=True)
        if rate != 16000 or audio.shape[1] != 1:
            parser.error(f"Expected 16 kHz mono audio: {path}")
        clips.append((str(path), audio[:, 0].copy(), hashlib.sha256(path.read_bytes()).hexdigest()))
    dll_handles = [os.add_dll_directory(str(path.resolve())) for path in args.dll_directory]
    import transcribe_cpp
    results = {key: {name: [] for name, _, _ in clips} for key in configurations}
    limits = {}
    sessions = {key: [] for key in configurations}
    for repetition in range(args.repetitions):
        order = list(configurations)
        if repetition % 2:
            order.reverse()
        for key in order:
            backend, context, threads, cpu_threads = configurations[key]
            with transcribe_cpp.Model(model_path, backend=backend) as model:
                vad = FireRedVad(intra_op_num_threads=threads)
                vad.warmup()
                setup_start = time.perf_counter()
                with model.session(kv_type="f16", n_ctx=context, n_threads=cpu_threads) as session:
                    setup_ms = (time.perf_counter() - setup_start) * 1000
                    limits[str(context)] = dataclasses.asdict(session.limits)
                    for _, audio, _ in clips:
                        chunks = vad.speech_timestamps(audio)
                        if chunks:
                            session.run(audio[chunks[0]["start"]:chunks[-1]["end"]], timestamps="none")
                    sessions[key].append(dict(setup_ms=setup_ms, gpu_memory_mib=gpu_memory_mib()))
                    for name, audio, _ in clips:
                        start = time.perf_counter()
                        chunks = vad.speech_timestamps(audio)
                        filtered = audio[chunks[0]["start"]:chunks[-1]["end"]] if chunks else audio[:0]
                        vad_ms = (time.perf_counter() - start) * 1000
                        decoded = session.run(filtered, timestamps="none") if len(filtered) else None
                        elapsed_ms = (time.perf_counter() - start) * 1000
                        sample = dict(total_ms=elapsed_ms, vad_ms=vad_ms,
                                      native_timings_ms=dataclasses.asdict(decoded.timings) if decoded else None,
                                      text=decoded.text.strip() if decoded else "",
                                      language=decoded.language if decoded else "unknown",
                                      speech_samples=len(filtered), vad_segments=chunks)
                        results[key][name].append(sample)
                        print(key, repetition, name, round(elapsed_ms, 2), sample["text"], flush=True)
    report = dict(engine=args.engine, backend=args.backend, model_path=model_path,
                  runtime_version=transcribe_cpp.__version__, native_commit=transcribe_cpp.native_commit(),
                  library_path=str(transcribe_cpp.library_path()), contexts=limits,
                  environment=dict(python=sys.version, platform=platform.platform(), cpu_count=os.cpu_count()),
                  source_sha256={path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                                 for path in [Path(__file__), Path(sys.modules[FireRedVad.__module__].__file__)]},
                  clips=[dict(path=name, samples=len(audio), sha256=digest) for name, audio, digest in clips],
                  configurations=configurations, sessions=sessions, results=results)
    save_report(report, args.output)


if __name__ == "__main__":
    main()
