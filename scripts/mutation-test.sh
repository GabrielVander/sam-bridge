#!/usr/bin/env bash
# Mutation testing for the Rust crates: cargo-mutants changes the code in small
# ways (flips a comparison, swaps an operator, blanks a return value) and checks
# that some test fails. A mutant that survives is code that runs in the tests
# but is not actually verified by them.
#
# Usage: scripts/mutation-test.sh [crate...]   (default: every crate with logic)
# Needs cargo-mutants (`cargo install cargo-mutants`); exits non-zero if any
# mutant survives.
#
# It runs on a lean copy of the Rust sources because cargo-mutants copies the
# whole tree, and gui_application/flutter holds gigabytes of build output that
# fill /tmp. The generated bridge code is skipped: mutating it proves nothing.
set -euo pipefail
cd "$(dirname "$0")/.."

crates=("$@")
if [ "${#crates[@]}" -eq 0 ]; then
  crates=(authentication student sam credential_store gui_application)
fi

copy="$(mktemp -d)"
trap 'rm -rf "$copy"' EXIT
tar --exclude=./target --exclude=./.git --exclude=./gui_application/flutter \
  --exclude='./mutants.out*' -cf - . | tar -C "$copy" -xf -
cd "$copy"

status=0
for crate in "${crates[@]}"; do
  echo "== $crate"
  cargo mutants -p "$crate" --exclude 'gui_application/src/frb_generated.rs' \
    --no-shuffle --jobs 2 || status=1
done
exit "$status"
