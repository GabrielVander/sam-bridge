#!/usr/bin/env bash
# Combined line-coverage report across Rust and Flutter, excluding generated code.
# Report-only: always exits 0. Needs cargo-llvm-cov, cargo-nextest and flutter on PATH.
#
# Exclusions:
# - Rust: generated FRB code and each crate's tests/support/ helper module (cargo
#   llvm-cov's report never includes tests/*.rs anyway, but the regex documents the
#   intent and stays correct if that changes).
# - Flutter: generated code (lib/rust/**, *.freezed.dart, *.g.dart) and lib/main.dart
#   (never imported by the coverage helper test below, so it never appears in the report).
set -euo pipefail
cd "$(dirname "$0")/.."

rust_lcov=$(mktemp)
trap 'rm -f "$rust_lcov"' EXIT

echo "== Flutter"
(
  cd gui_application/flutter
  helper=$(mktemp -d)/coverage_helper_test.dart
  trap 'rm -rf "$(dirname "$helper")"' EXIT

  # flutter test --coverage only reports files a test imports, so untested files would
  # silently vanish from the report. This helper test imports every hand-written file.
  # It lives outside the project (flutter test accepts a target file anywhere, resolving
  # package imports from the cwd's pubspec) so no generated file ever touches the working tree.
  package=$(sed -n 's/^name: *//p' pubspec.yaml | head -1)
  {
    find lib -name '*.dart' \
      ! -path 'lib/rust/*' ! -path 'lib/main.dart' ! -name '*.freezed.dart' ! -name '*.g.dart' | sort |
      awk -v pkg="$package" '{ sub(/^lib\//, ""); printf "import '\''package:%s/%s'\'' as i%d;\n", pkg, $0, NR }'
    echo "void main() {}"
  } >"$helper"

  flutter test --coverage test "$helper" >/dev/null
)

echo "== Rust"
cargo llvm-cov nextest --workspace \
  --ignore-filename-regex 'frb_generated\.rs|/tests/support/' \
  --lcov --output-path "$rust_lcov" \
  --status-level=fail --final-status-level=fail >/dev/null

echo "== Combined"
python3 - "$rust_lcov" gui_application/flutter/coverage/lcov.info <<'PY'
import re, sys

def totals(path, *, exclude=None):
    hit = total = 0
    for rec in open(path).read().split("end_of_record"):
        m = re.search(r"^SF:(.*)$", rec, re.M)
        if not m:
            continue
        if exclude and exclude(m.group(1)):
            continue
        total += int(re.search(r"^LF:(\d+)$", rec, re.M).group(1))
        hit += int(re.search(r"^LH:(\d+)$", rec, re.M).group(1))
    return hit, total

def flutter_excluded(path):
    return path.startswith("lib/rust/") or path.endswith((".freezed.dart", ".g.dart"))

rust_hit, rust_total = totals(sys.argv[1])
flutter_hit, flutter_total = totals(sys.argv[2], exclude=flutter_excluded)

def line(hit, total, label):
    pct = 100.0 if total == 0 else 100.0 * hit / total
    print(f"{pct:6.2f}%  {hit:4}/{total:<4}  {label}")

line(rust_hit, rust_total, "Rust")
line(flutter_hit, flutter_total, "Flutter")
line(rust_hit + flutter_hit, rust_total + flutter_total, "TOTAL")
PY
