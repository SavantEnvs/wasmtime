// Additive in-process libFuzzer harness for the WebAssembly binary parser/validator
// (`wasmparser`, the exact 0.258.0 version wasmtime pins and consumes).
//
// The fuzzer hands us arbitrary bytes; we drive them through the two public entry
// points of the parser/validator surface:
//   1. the streaming `Parser` over every top-level payload (structural decode), and
//   2. the full `Validator` (`wasmparser::validate`) which recursively validates the
//      type/function/code sections including every operator in every function body.
//
// No disk I/O and no reliance on any file path (SPEC §3): all input comes from the
// fuzzer. Errors are the normal, expected result on most inputs and are ignored; a
// PANIC / ASan report inside wasmparser on any input is a genuine finding, so nothing
// is guarded. Upstream source is untouched — this crate only CALLS wasmparser.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // (1) Exercise the streaming parser over arbitrary bytes. Drain every payload so
    // section readers are actually decoded, not just constructed lazily.
    for payload in wasmparser::Parser::new(0).parse_all(data) {
        if payload.is_err() {
            break;
        }
    }

    // (2) Exercise the full validator (default WasmFeatures) — the deep path that walks
    // function bodies and operators. Ignore the Result: an invalid module is a normal,
    // expected outcome, not a bug.
    let _ = wasmparser::validate(data);
});
