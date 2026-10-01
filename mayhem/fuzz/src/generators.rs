//! Test case generators.
//!
//! Vendored from `crates/fuzzing/src/generators.rs` at the commit this branch is pinned to,
//! trimmed to the modules `generators::api::ApiCalls` actually needs. See `mayhem/fuzz/Cargo.toml`
//! for why: upstream's own `fuzz/` crate pulls in `wasmtime-fuzzing`, which unconditionally
//! depends on the `v8` crate (via `oracles::diff_v8`, a private module `oracles.rs` declares
//! regardless of which binary is built) and, through it, a network fetch of a prebuilt V8
//! binary at build time — incompatible with the air-gapped PATCH rebuild (SPEC §6.5).

pub mod api;
mod codegen_settings;
mod config;
mod instance_allocation_strategy;
mod memory;
mod module;
mod pooling_config;

pub use codegen_settings::CodegenSettings;
pub use config::{Config, WasmtimeConfig};
pub use instance_allocation_strategy::InstanceAllocationStrategy;
pub use memory::{MemoryConfig, NormalMemoryConfig, UnalignedMemory, UnalignedMemoryCreator};
pub use module::ModuleConfig;
pub use pooling_config::PoolingAllocationConfig;
