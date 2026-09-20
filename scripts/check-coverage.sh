#!/usr/bin/env bash
# The coverage gate: fails unless both stacks keep 100% line coverage of the
# hand-written code (Rust functions as well). Provider-neutral, so any CI can run
# it as a single step; needs cargo-llvm-cov, cargo-nextest and flutter on PATH.
#
# The exclusions are documented where they are applied:
# - Rust: generated FRB code and the test_support crate (see the regex below).
# - Flutter: generated code and lib/main.dart (see gui_application/flutter/tool/coverage.sh).
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== Rust"
cargo llvm-cov nextest --workspace \
  --ignore-filename-regex 'frb_generated\.rs|test_support/' \
  --fail-under-lines 100 --fail-under-functions 100 --summary-only

echo "== Flutter"
flutter_coverage() { (cd gui_application/flutter && tool/coverage.sh --fail-under 100); }
if ! flutter_coverage; then
  # The Flutter measurement has been seen to drop a single const-constructor line
  # in a run that a re-run did not reproduce. A real gap is deterministic and
  # fails both times, so measure once more before failing.
  echo "Flutter coverage was below 100%; measuring once more in case the measurement was flaky" >&2
  flutter_coverage
fi
