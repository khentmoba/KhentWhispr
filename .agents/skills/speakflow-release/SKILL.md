---
name: speakflow-release
description: Publish a SpeakFlow release with signed in-app updates. Use when asked to trigger a release, ship a version, or publish an app update.
---

# SpeakFlow release

Create an actual GitHub Release for `khentmoba/SpeakFlow`, not a release PR.
Creating or editing this skill is not authorization to publish.
Run commands from the repository root.

## 1. Establish the release candidate

Read `../../../docs/releasing.md`, `../../../scripts/release.sh` and
`../../../src-tauri/tauri.conf.json` relative to this skill directory. These are
the source of truth for release gates, signing and the updater endpoint.

- Inspect the working tree, branch, origin URL, local versions, remote tags and
  latest published release. Confirm origin is SpeakFlow's repository.
- Confirm an unused `X.Y.Z` version newer than both the source version and the
  latest published stable release. If the user supplied no version, propose a
  patch bump and ask for confirmation. Write concise English release notes from
  the changes since the previous release.
- Use a clean `main` checkout matching `origin/main`. If this thread is in a
  feature worktree, locate the existing main checkout and verify its state.
  Unmerged changes are a blocker: report what must reach main first rather than
  silently merging, resetting, stashing or dropping work.
- Check the script's required tools, GitHub authentication and signing-key
  availability without printing secret values. Preserve the existing updater key;
  generating a replacement would break trust for installed clients. If the key
  is unavailable, stop and ask the maintainer to provision it securely outside
  the repository.
- Rebuild the engine by default. Reuse only with evidence that the archive matches
  unchanged packaged Python code, native libraries and dependencies. If R2T2
  native inputs changed, follow `../../../docs/r2t2-native.md` first.

Done when the version, notes, candidate checkout and engine mode are established,
and the release can run without bypassing a preflight check.

## 2. Build and publish

Run the existing script with the confirmed version and notes:

```bash
bash scripts/release.sh <X.Y.Z> "<English release notes>" --rebuild-engine
```

Use `--reuse-engine` only when established in step 1. This script builds locally,
updates version metadata, validates the artifacts, pushes the candidate commit,
waits for that exact commit's required CI jobs, then publishes the tag and assets.
A tag push or workflow dispatch alone does not build or publish this app.

Stop on any failure and report the last successful stage, candidate SHA, CI URL
if available, and whether a commit, tag or release reached GitHub. Follow the
recovery section of `docs/releasing.md` before retrying; preserve published tags
and assets. Keep the CI and signing gates intact.

Done when the script succeeds and the published tag resolves to the checked
candidate commit.

## 3. Verify installed clients can discover the update

Inspect `gh release view <tag> --repo khentmoba/SpeakFlow --json url,tagName,isDraft,isPrerelease,assets`
and fetch the public endpoint configured in `plugins.updater.endpoints` with
cache bypass/retries as needed. Download the published assets to a temporary
directory rather than trusting only local build output.

Verify all of the following:

- The release is published, stable, and is the repository's latest release.
- Its assets include the versioned Windows x64 installer, matching `.sig`,
  `latest.json`, and `engine.tar.xz`.
- The public updater endpoint returns the confirmed version. Its
  `platforms.windows-x86_64.url` points to this release's installer and is
  downloadable; its signature equals the nonempty published `.sig` contents.
- The downloaded installer SHA-256 matches the locally validated installer.
- The configured updater public key and endpoint remain unchanged. Tauri updater
  signatures are distinct from Windows Authenticode; describe each accurately.

If the public feed is stale or an asset is missing, report publication as
incomplete for in-app updates. Do not replace assets without explicit approval.

Report the release URL, version/tag, candidate SHA, CI result and feed verification.
Distinguish these checks from a real installation test. Only claim end-to-end
update success after a maintainer-authorized test from an older compatible
installed build through Settings, including restart and version confirmation.
Publishing alone does not update the developer's installed app; versions through
1.0.1 need a one-time manual installation to gain the native updater.
