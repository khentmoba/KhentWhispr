# Async ownership port: correspondence and limits

Ported from Light Whisper commit
[`c43606b`](https://github.com/sypsyp97/light-whisper/commit/c43606ba565b4185c392bfe09e2f15005afe4b6e).
The upstream author's original measurements and proof logs are available in
[the pinned upstream evidence](https://github.com/sypsyp97/light-whisper/blob/c43606ba565b4185c392bfe09e2f15005afe4b6e/formal/evidence/async-ownership.md).
Those measurements are not validation of this modified SpeakFlow checkout.
See [the integration ledger](../../docs/upstream.md) for local results.

## Ported seams

Six finite companion models supplement the existing contracts:

| Model | Actual implementation/regression seam |
| --- | --- |
| MicrophoneAsyncOwnership | `useMicrophoneSettings.test.tsx`: out-of-order test/start promises and intent retirement. |
| MicrophoneEffectOwners | The same hook tests: two effect instances, shared native-operation queue and retired cleanup. |
| HistoryLeaseInterleavings | `history_service.rs::async_ownership_replay_tests`: public SQLite get/delete/release/cleanup operations and real temporary WAVs. |
| OutputDrainOwnership | `output_tests.rs::async_ownership_replay_tests`: actual AppState, dispatch mutex, pending sessions and target/recovery decisions; native paste is injected. |
| IpcCommandDeadline | `ipc_recovery_tests.rs::async_ownership_replay_tests`: actual command timeout, child stdin/flush/read, cancellation and replacement process. |
| ModelIntegrityRepair | `test_async_ownership_replay.py`, cache integrity regressions, `useModelStatus.test.tsx` and `StatusIndicator.test.tsx`: corrupt-cache detection, explicit repair and readiness. |

The history companion covers two independent holders sharing audio. Releasing a
lease deletes an existing count-one row **before** decrementing greater counts;
UPDATE-then-DELETE could retire the other holder's protection in one transaction.

Pending dictations retain their own session, original target and exact text.
Unknown/changed targets and native paste failures offer a copy action instead of
blindly pasting into a different application. Deduplication covers pending IDs,
not a global exactly-once delivery promise. SpeakFlow's opt-in external correction
learning remains attached only to successful targeted pastes.

A single IPC timeout covers lock acquisition, write, flush and reply. Dropping a
cancelled command invalidates the process instead of reusing partial JSON frames.
Model repair treats present invalid completion metadata as authoritative; absent
metadata retains legacy compatibility. SpeakFlow additionally keeps a missing
separately downloaded engine in the download flow, even with valid model files.

## Deliberate exclusions

SpeakFlow no longer has the upstream assistant-key editor. Its dirty-draft/read
model, page tests and mutation fixtures are not ported or claimed as coverage.
The relevant backend and IPC safeguards accept an explicit captured provider;
`src/api/__tests__/tauri.test.ts` checks that boundary. Existing credential storage
and authentication snapshot contracts remain registered.

These are bounded state/protocol models plus reviewed implementation tests, not
compiled refinement proofs. Windows focus is sampled, native input success does
not prove a foreign application consumed the text, and SQLite/keyring/CPAL/OS
internals, live GPU inference and linguistic accuracy remain external assumptions.

## Reproduction

```sh
python scripts/generate_async_ownership_traces.py --check
python scripts/test_async_ownership_traces.py
python scripts/test_async_output_wiring.py
python scripts/check_formal.py --inventory-only
pnpm exec vitest run src/hooks/__tests__/useMicrophoneSettings.test.tsx src/hooks/__tests__/useModelStatus.test.tsx src/hooks/__tests__/useRecordingOutputRecovery.test.tsx src/components/StatusIndicator.test.tsx src/api/__tests__/tauri.test.ts
cargo test --manifest-path src-tauri/Cargo.toml --lib async_ownership_replay_tests -- --nocapture
python -m unittest discover -s src-tauri/resources -p test_async_ownership_replay.py
```

The trace generator checks the pinned TLC 1.7.4 checksum and extracts real
counterexample State blocks. JSON fixtures bind model/config/log hashes and the
exact event sequence; editing events independently fails provenance validation.
Hashed `.tla`, `.cfg` and `.tlc.txt` files use LF on Windows and Linux. The repair
reachability witness deliberately violates `RepairPathNotReached` to produce a
repair schedule; it is not a safety proof or a broken implementation variant.

Full TLC/Lean commands and proof boundaries are in [the formal guide](../README.md).
The optional `run_async_replay_mutations.py` checks the two applicable microphone
mutations in isolated snapshots; it never mutates live sources.
