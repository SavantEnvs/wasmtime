#!/usr/bin/env bash
#
# mayhem/test.sh — behavioral oracle for the wasmtime (cranelift-isle) Mayhem integration.
#
# Runs the KAT probe (built by mayhem/build.sh: mayhem/kat, a dynamically-linked binary), which
# parses and compiles ISLE rules, and asserts the EXACT values it prints. This is a known-answer test through a dynamically linked binary, so it
# FAILS when cranelift-isle (or the whole probe, under the gate's LD_PRELOAD sabotage shim) is neutered to
# a no-op — the anti-reward-hacking contract (SPEC §6.3). bash + coreutils (grep) are whitelisted by
# the shim, so the comparison runs where sabotage cannot hide.
#
# The probe is UNCONDITIONAL: a missing binary is a FAILURE, never a skip. Emits a CTRF
# summary; exit 0 iff failed==0. Does NOT build (build.sh did).
set -uo pipefail
[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH
cd "$SRC"

# emit_ctrf <tool> <passed> <failed> [skipped] [pending] [other]
emit_ctrf() {
  local tool="$1" passed="$2" failed="$3" skipped="${4:-0}" pending="${5:-0}" other="${6:-0}"
  local tests=$(( passed + failed + skipped + pending + other ))
  cat > "${CTRF_REPORT:-$SRC/ctrf-report.json}" <<JSON
{
  "results": {
    "tool": { "name": "$tool" },
    "summary": {
      "tests": $tests,
      "passed": $passed,
      "failed": $failed,
      "pending": $pending,
      "skipped": $skipped,
      "other": $other
    }
  }
}
JSON
  printf 'CTRF {"results":{"tool":{"name":"%s"},"summary":{"tests":%d,"passed":%d,"failed":%d,"pending":%d,"skipped":%d,"other":%d}}}\n' \
    "$tool" "$tests" "$passed" "$failed" "$pending" "$skipped" "$other"
  [ "$failed" -eq 0 ]
}

PROBE=/mayhem/kat_probe
# Exact known answer (see mayhem/kat/src/main.rs): the two rules parse with priorities 7 and none, the
# program compiles to Rust containing `fn constructor_F`, and a truncated rule is rejected.
EXPECT='KAT rule prio Some(7)
KAT rule prio None
KAT codegen constructor_F true
KAT truncated rule rejected true'

passed=0; failed=0

if [ ! -x "$PROBE" ]; then
  echo "FAIL: KAT probe $PROBE missing or not executable (build.sh must produce it)" >&2
  emit_ctrf "cranelift-isle-kat" 0 1 0
  exit 1
fi

OUT="$("$PROBE" 2>/dev/null || true)"
echo "KAT probe output: [$OUT]"

# Assert the whole computed output, byte for byte, against the known answer.
if [ "$OUT" = "$EXPECT" ]; then
  echo "PASS: cranelift-isle parsed and compiled the KAT rules as expected"
  passed=$((passed+1))
else
  echo "FAIL: expected [$EXPECT] but got [$OUT]" >&2
  failed=$((failed+1))
fi

emit_ctrf "cranelift-isle-kat" "$passed" "$failed"
