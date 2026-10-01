//! Oracles.
//!
//! Vendored from `crates/fuzzing/src/oracles.rs` at the commit this branch is pinned to, trimmed
//! to what `make_api_calls` needs. The full file unconditionally declares `mod diff_v8;` (only
//! `#[cfg]`-gated on target arch, not on any feature), so compiling the real `wasmtime-fuzzing`
//! crate at all pulls in the `v8` crate and its build-time fetch of a prebuilt V8 binary — see
//! `mayhem/fuzz/Cargo.toml`. This copy drops `diff_v8`/`diff_spec`/`diff_wasmi`/`diff_wasmtime`/
//! `engine`/`stacks` (differential-testing oracles, unused by `api_calls`) and keeps `dummy`
//! (imports for instantiation), `StoreLimits`, `Timeout`, `log_wasm`, `instantiate_with_dummy`
//! and `make_api_calls` verbatim.

pub mod dummy;

use crate::generators;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::time::Duration;
use wasmtime::*;

static CNT: AtomicUsize = AtomicUsize::new(0);

/// Logs a wasm file to the filesystem to make it easy to figure out what wasm
/// was used when debugging.
pub fn log_wasm(wasm: &[u8]) {
    super::init_fuzzing();

    if !log::log_enabled!(log::Level::Debug) {
        return;
    }

    let i = CNT.fetch_add(1, SeqCst);
    let name = format!("testcase{}.wasm", i);
    std::fs::write(&name, wasm).expect("failed to write wasm file");
    log::debug!("wrote wasm file to `{}`", name);
    let wat = format!("testcase{}.wat", i);
    match wasmprinter::print_bytes(wasm) {
        Ok(s) => std::fs::write(&wat, s).expect("failed to write wat file"),
        // If wasmprinter failed remove a `*.wat` file, if any, to avoid
        // confusing a preexisting one with this wasm which failed to get
        // printed.
        Err(_) => drop(std::fs::remove_file(&wat)),
    }
}

/// The `T` in `Store<T>` for fuzzing stores, used to limit resource
/// consumption during fuzzing.
#[derive(Clone)]
pub struct StoreLimits(Rc<LimitsState>);

struct LimitsState {
    /// Remaining memory, in bytes, left to allocate
    remaining_memory: Cell<usize>,
    /// Whether or not an allocation request has been denied
    oom: Cell<bool>,
}

impl StoreLimits {
    /// Creates the default set of limits for all fuzzing stores.
    pub fn new() -> StoreLimits {
        StoreLimits(Rc::new(LimitsState {
            // Limits tables/memories within a store to at most 1gb for now to
            // exercise some larger address but not overflow various limits.
            remaining_memory: Cell::new(1 << 30),
            oom: Cell::new(false),
        }))
    }

    fn alloc(&mut self, amt: usize) -> bool {
        match self.0.remaining_memory.get().checked_sub(amt) {
            Some(mem) => {
                self.0.remaining_memory.set(mem);
                true
            }
            None => {
                self.0.oom.set(true);
                false
            }
        }
    }
}

impl ResourceLimiter for StoreLimits {
    fn memory_growing(&mut self, current: usize, desired: usize, _maximum: Option<usize>) -> bool {
        self.alloc(desired - current)
    }

    fn table_growing(&mut self, current: u32, desired: u32, _maximum: Option<u32>) -> bool {
        let delta = (desired - current) as usize * std::mem::size_of::<usize>();
        self.alloc(delta)
    }
}

/// Methods of timing out execution of a WebAssembly module
#[derive(Clone, Debug)]
pub enum Timeout {
    /// No timeout is used, it should be guaranteed via some other means that
    /// the input does not infinite loop.
    None,
    /// Fuel-based timeouts are used where the specified fuel is all that the
    /// provided wasm module is allowed to consume.
    Fuel(u64),
    /// An epoch-interruption-based timeout is used with a sleeping
    /// thread bumping the epoch counter after the specified duration.
    Epoch(Duration),
}

/// Create a Wasmtime [`Instance`] from a [`Module`] and fill in all imports
/// with dummy values (e.g., zeroed values, immediately-trapping functions).
/// Also, this function catches certain fuzz-related instantiation failures and
/// returns `None` instead of panicking.
pub fn instantiate_with_dummy(store: &mut Store<StoreLimits>, module: &Module) -> Option<Instance> {
    // Creation of imports can fail due to resource limit constraints, and then
    // instantiation can naturally fail for a number of reasons as well. Bundle
    // the two steps together to match on the error below.
    let instance =
        dummy::dummy_linker(store, module).and_then(|l| l.instantiate(&mut *store, module));

    let e = match instance {
        Ok(i) => return Some(i),
        Err(e) => e,
    };

    // If the instantiation hit OOM for some reason then that's ok, it's
    // expected that fuzz-generated programs try to allocate lots of
    // stuff.
    if store.data().0.oom.get() {
        log::debug!("failed to instantiate: OOM");
        return None;
    }

    // Allow traps which can happen normally with `unreachable` or a
    // timeout or such
    if let Some(trap) = e.downcast_ref::<Trap>() {
        log::debug!("failed to instantiate: {}", trap);
        return None;
    }

    let string = e.to_string();
    // Currently we instantiate with a `Linker` which can't instantiate
    // every single module under the sun due to using name-based resolution
    // rather than positional-based resolution
    if string.contains("incompatible import type") {
        log::debug!("failed to instantiate: {}", string);
        return None;
    }

    // Also allow failures to instantiate as a result of hitting instance limits
    if string.contains("maximum concurrent instance limit") {
        log::debug!("failed to instantiate: {}", string);
        return None;
    }

    // Everything else should be a bug in the fuzzer or a bug in wasmtime
    panic!("failed to instantiate: {:?}", e);
}

/// Invoke the given API calls.
pub fn make_api_calls(api: generators::api::ApiCalls) {
    use crate::generators::api::ApiCall;
    use std::collections::HashMap;

    let mut store: Option<Store<StoreLimits>> = None;
    let mut modules: HashMap<usize, Module> = Default::default();
    let mut instances: HashMap<usize, Instance> = Default::default();

    for call in api.calls {
        match call {
            ApiCall::StoreNew(config) => {
                log::trace!("creating store");
                assert!(store.is_none());
                store = Some(config.to_store());
            }

            ApiCall::ModuleNew { id, wasm } => {
                log::debug!("creating module: {}", id);
                log_wasm(&wasm);
                let module = match Module::new(store.as_ref().unwrap().engine(), &wasm) {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                let old = modules.insert(id, module);
                assert!(old.is_none());
            }

            ApiCall::ModuleDrop { id } => {
                log::trace!("dropping module: {}", id);
                drop(modules.remove(&id));
            }

            ApiCall::InstanceNew { id, module } => {
                log::trace!("instantiating module {} as {}", module, id);
                let module = match modules.get(&module) {
                    Some(m) => m,
                    None => continue,
                };

                let store = store.as_mut().unwrap();
                if let Some(instance) = instantiate_with_dummy(store, module) {
                    instances.insert(id, instance);
                }
            }

            ApiCall::InstanceDrop { id } => {
                log::trace!("dropping instance {}", id);
                drop(instances.remove(&id));
            }

            ApiCall::CallExportedFunc { instance, nth } => {
                log::trace!("calling instance export {} / {}", instance, nth);
                let instance = match instances.get(&instance) {
                    Some(i) => i,
                    None => {
                        // Note that we aren't guaranteed to instantiate valid
                        // modules, see comments in `InstanceNew` for details on
                        // that. But the API call generator can't know if
                        // instantiation failed, so we might not actually have
                        // this instance. When that's the case, just skip the
                        // API call and keep going.
                        continue;
                    }
                };
                let store = store.as_mut().unwrap();

                let funcs = instance
                    .exports(&mut *store)
                    .filter_map(|e| match e.into_extern() {
                        Extern::Func(f) => Some(f.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                if funcs.is_empty() {
                    continue;
                }

                let nth = nth % funcs.len();
                let f = &funcs[nth];
                let ty = f.ty(&store);
                let params = dummy::dummy_values(ty.params());
                let mut results = vec![Val::I32(0); ty.results().len()];
                let _ = f.call(store, &params, &mut results);
            }
        }
    }
}
