//! Additive cargo-fuzz support crate for the Mayhem backport of wasmtime's `api_calls` target.
//!
//! See `mayhem/fuzz/Cargo.toml` for why this vendors a trimmed `generators`/`oracles` module pair
//! from `crates/fuzzing` instead of depending on the upstream `wasmtime-fuzzing` crate directly.

pub mod generators;
pub mod oracles;

/// One time start up initialization for fuzzing: enables `env_logger`.
pub(crate) fn init_fuzzing() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let _ = env_logger::try_init();
    })
}
