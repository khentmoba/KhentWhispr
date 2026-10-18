"""Exercise the real release CI gate with a local GitHub CLI mock."""

import os
import re
import shutil
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CANDIDATE = "a" * 40


class ReleaseCiTests(unittest.TestCase):
    def run_gate(self, formal: str, head: str = CANDIDATE) -> subprocess.CompletedProcess:
        source = (ROOT / "scripts/release.sh").read_text(encoding="utf-8")
        function = re.search(r"^wait_for_ci\(\) \{\n.*?^\}", source, re.M | re.S)
        self.assertIsNotNone(function)
        bash = (Path(os.environ.get("ProgramFiles", "C:/Program Files")) / "Git/bin/bash.exe"
                if os.name == "nt" else shutil.which("bash"))
        self.assertTrue(bash and Path(bash).is_file(), "Git Bash (Windows) or Bash is required")
        # Load only this read-only function. The release script's main/tag/upload
        # paths are never sourced or invoked; every gh call is handled locally.
        harness = r'''
set -euo pipefail
CI_WORKFLOW=ci.yml
CI_DISCOVERY_TIMEOUT_SECONDS=5
fail() { printf '%s\n' "$*" >&2; exit 1; }
gh() {
    case "$*" in
        'run list '*) printf '123\n' ;;
        'run watch '*) return 0 ;;
        *'--json headSha '*) printf '%s\n' "$TEST_RUN_HEAD" ;;
        *'--json conclusion '*) printf 'success\n' ;;
        *'--json jobs '*'Application formal contracts'*)
            if [[ "$TEST_FORMAL_RESULT" != missing ]]; then printf '%s\n' "$TEST_FORMAL_RESULT"; fi ;;
        *'--json jobs '*) printf 'success\n' ;;
        *) printf 'Unexpected gh invocation: %s\n' "$*" >&2; return 99 ;;
    esac
}
'''
        environment = dict(os.environ, TEST_FORMAL_RESULT=formal, TEST_RUN_HEAD=head)
        return subprocess.run([str(bash), "-c", harness + function[0] + '\nwait_for_ci "$1"',
                               "release-ci-test", CANDIDATE], env=environment,
                              text=True, encoding="utf-8", capture_output=True, timeout=15)

    def test_rust_cache_cannot_restore_paths_from_a_renamed_repository(self):
        workflow = (ROOT / ".github/workflows/ci.yml").read_text(encoding="utf-8")
        cache = workflow.split("      - name: Cache Rust dependencies and build outputs", 1)[1]
        cache = cache.split("      - name:", 1)[0]
        keys = [line.strip() for line in cache.splitlines()
                if line.strip().startswith(("key: rust-", "rust-"))]
        self.assertEqual(len(keys), 2)
        for key in keys:
            self.assertIn("rust-${{ github.repository }}-", key)

    def test_all_four_jobs_pass(self):
        result = self.run_gate("success")
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_missing_formal_job_blocks_release(self):
        result = self.run_gate("missing")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Application formal contracts", result.stderr)

    def test_skipped_formal_job_blocks_release(self):
        result = self.run_gate("skipped")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Application formal contracts", result.stderr)

    def test_failed_formal_job_blocks_release(self):
        result = self.run_gate("failure")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Application formal contracts", result.stderr)

    def test_different_commit_blocks_release(self):
        result = self.run_gate("success", "b" * 40)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("headSha", result.stderr)


if __name__ == "__main__":
    unittest.main()
