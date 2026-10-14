#!/usr/bin/env bash
#
# mayhem/build.sh — build wasmtime's Mayhem fuzz target as a sanitized libFuzzer binary
# (OSS-Fuzz Rust path: cargo-fuzz + ASan via RUSTFLAGS), plus a small dynamically-linked
# KAT probe that mayhem/test.sh RUNS as the behavioral oracle.
#
# Runs inside the commit image (RUST mayhem/Dockerfile) as `mayhem` in /mayhem. The Rust
# toolchain + cargo registry live at $CARGO_HOME=/opt/toolchains/rust/cargo (pinned by the
# Dockerfile ENV — absolute, $HOME-independent).
#
# WHAT is fuzzed: cranelift-isle's lexer, parser and compiler (mayhem/fuzz, compile), on the
# cranelift-isle in this checkout. Both additive crates (mayhem/fuzz, mayhem/kat) are standalone workspaces
# path-depending on cranelift/isle/isle; upstream source is never touched.
#
# AIR-GAPPED CONTRACT (SPEC §6.5): the PATCH tier re-runs THIS script OFFLINE.
#   - This FIRST build (online) resolves deps + writes the two Cargo.lock files and
#     populates the cargo registry under $CARGO_HOME.
#   - The PATCH re-run resolves from that cache (CARGO_NET_OFFLINE=true is exported by the
#     runtime), so we do NOT hard-code `--offline` here and we never run generate-lockfile
#     / cargo update (those hit the network index).
set -euo pipefail

# clang rejects SOURCE_DATE_EPOCH='' — must be unset or a valid integer.
[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH

: "${MAYHEM_JOBS:=$(nproc)}"
# cargo-fuzz has no --jobs flag; cargo reads parallelism from CARGO_BUILD_JOBS.
export CARGO_BUILD_JOBS="$MAYHEM_JOBS"

cd "$SRC"

# Sanitizers (§6.1): the base provides clang $SANITIZER_FLAGS (ASan+UBSan, halting).
# rustc can't consume those clang flags, but we honor the KNOB: when $SANITIZER_FLAGS is
# non-empty we instrument the Rust build with ASan (the OSS-Fuzz Rust path); an explicit
# empty `--build-arg SANITIZER_FLAGS=` yields an un-sanitized build. Referencing
# $SANITIZER_FLAGS keeps the fuzzed code sanitized by default.
RUST_SAN=""
if [ -n "${SANITIZER_FLAGS:-}" ]; then
  RUST_SAN="-Zsanitizer=address"
fi

# Debug info (§6.2 item 10): the produced fuzz binary MUST carry DWARF < 4 (Mayhem triage
# can't read DWARF >= 4). rustc nightly defaults to DWARF-5, so pin -Zdwarf-version=3 for
# Rust code; the libfuzzer-sys cc shim is compiled by clang (DWARF-5 default), so pin its
# DWARF via CFLAGS/CXXFLAGS. $RUST_DEBUG_FLAGS threads any extra base pins.
export RUSTFLAGS="${RUSTFLAGS:-} ${RUST_DEBUG_FLAGS:-} --cfg fuzzing ${RUST_SAN} -Zdwarf-version=3 -Cdebuginfo=1 -Cforce-frame-pointers"
export CFLAGS="${CFLAGS:-} -gdwarf-3"
export CXXFLAGS="${CXXFLAGS:-} -gdwarf-3"

# The bundled ASan runtime archive that `-Zsanitizer=address` links is precompiled with
# clang (DWARF-5) and ships with full debug info, which would otherwise land a DWARF-5
# compile unit as the final binary's FIRST CU and fail the DWARF < 4 gate. Strip debug
# info from that runtime archive (a toolchain artifact, NOT project code). Idempotent:
# re-running --strip-debug on an already-stripped archive is a no-op, so the offline
# PATCH re-run stays clean.
if [ -n "${RUST_SAN}" ]; then
  RT_LIB_DIR="$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/lib"
  for asan in "$RT_LIB_DIR"/librustc-*_rt.asan.a; do
    [ -f "$asan" ] || continue
    if [ -w "$asan" ]; then
      objcopy --strip-debug "$asan" "$asan.stripped" && mv "$asan.stripped" "$asan"
      echo "stripped debug info from bundled ASan runtime: $asan"
    fi
  done
fi

FUZZ_DIR="mayhem/fuzz"
TRIPLE="x86_64-unknown-linux-gnu"

# Discover every target from the crate's fuzz_targets/ dir (one binary per target).
FUZZ_TARGETS=()
for f in "$FUZZ_DIR"/fuzz_targets/*.rs; do
  FUZZ_TARGETS+=("$(basename "${f%.*}")")
done
[ "${#FUZZ_TARGETS[@]}" -gt 0 ] || { echo "ERROR: no fuzz targets under $FUZZ_DIR/fuzz_targets/" >&2; exit 1; }

echo "=== cargo fuzz build (image nightly, ASan via RUSTFLAGS) ==="
echo "RUSTFLAGS=$RUSTFLAGS"
echo "targets: ${FUZZ_TARGETS[*]}"

# Use the image's DEFAULT toolchain (the Dockerfile's pinned nightly).
for t in "${FUZZ_TARGETS[@]}"; do
  echo "--- building fuzz target: $t ---"
  cargo fuzz build --fuzz-dir "$FUZZ_DIR" -O --debug-assertions "$t"
  bin="$SRC/$FUZZ_DIR/target/$TRIPLE/release/$t"
  [ -x "$bin" ] || { echo "ERROR: expected fuzz binary not found at $bin" >&2; exit 1; }
  cp "$bin" "/mayhem/$t"
  echo "built /mayhem/$t"
done

# ── KAT probe (behavioral oracle) — clean, NON-sanitized, dynamically linked ──────────
# Built without the fuzz/DWARF/ASan RUSTFLAGS (its own honest build). Rust binaries are
# dynamically linked by default, so the gate's LD_PRELOAD sabotage shim reaches it.
echo "=== building KAT probe (mayhem/kat) ==="
env -u RUSTFLAGS -u CFLAGS -u CXXFLAGS \
  cargo build --release --manifest-path mayhem/kat/Cargo.toml
KAT_BIN="$SRC/mayhem/kat/target/release/kat_probe"
[ -x "$KAT_BIN" ] || { echo "ERROR: KAT probe not found at $KAT_BIN" >&2; exit 1; }
cp "$KAT_BIN" /mayhem/kat_probe
# Regression guard (SPEC §6.3): the oracle only detects sabotage if the probe is dynamic.
file /mayhem/kat_probe | grep -q 'dynamically linked' \
  || { echo "ERROR: KAT probe is not dynamically linked — oracle would be sabotage-immune" >&2; exit 1; }
echo "built /mayhem/kat_probe (dynamically linked)"

echo "build.sh complete"
