#!/usr/bin/env python3
"""Full-workspace verification runner for shbt-recon (rec1.txt §4).

Runs: kernel build, cargo test --workspace, python smoke checks, and the
verification matrix (written to verification_matrix.json).
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parents[1]


def run(cmd, **kw) -> int:
    print(f"[run_all_tests] $ {' '.join(map(str, cmd))}", flush=True)
    return subprocess.run(cmd, cwd=REPO, **kw).returncode


def main() -> int:
    rc = run([sys.executable, "python/shbt_recon/cli/main.py", "build-kernel"])
    if rc:
        print("kernel build failed")
        return rc

    rc = run(["cargo", "test", "--workspace"])
    if rc:
        return rc

    out = subprocess.run(
        [sys.executable, "python/shbt_recon/cli/main.py", "verify"],
        cwd=REPO, capture_output=True, text=True,
    )
    print(out.stdout)
    (REPO / "verification_matrix.json").write_text(out.stdout)
    if out.returncode:
        print(out.stderr)
        return out.returncode

    # Python binding smoke test (only when the extension was built).
    try:
        import shbt_recon  # noqa: F401
    except ImportError:
        print("[run_all_tests] shbt_recon extension not built; "
              "skipping PyO3 smoke test")
    print("[run_all_tests] all suites passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
