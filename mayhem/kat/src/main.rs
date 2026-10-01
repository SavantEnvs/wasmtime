// Known-answer-test probe for the Mayhem behavioral oracle (SPEC §6.3).
//
// Compiles + instantiates + calls a FIXED, hand-assembled WebAssembly module through this
// commit's own `wasmtime` engine and prints a value it COMPUTES from running it. mayhem/test.sh
// asserts the exact output.
//
// The embedded module (a valid Wasm 1.0 module) is, byte for byte:
//   00 61 73 6d 01 00 00 00            ; "\0asm" magic + version 1
//   01 05 01 60 00 01 7f               ; Type section:     1 type  = () -> i32
//   03 02 01 00                        ; Function section: 1 func, using type #0
//   07 0a 01 06 'answer' 00 00         ; Export section:   func #0 exported as "answer"
//   0a 06 01 04 00 41 2a 0b            ; Code section:     1 body  = (i32.const 42)
// It MUST compile, instantiate with no imports, and calling "answer" MUST return 42. The probe
// therefore prints, deterministically: KAT wasmtime exports=1 answer=42
//
// Every printed field is derived from running the module through wasmtime's own compiler +
// runtime, so neutering the engine (or the whole probe, via the sabotage shim) changes/removes
// the line and fails the oracle.

const MODULE: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // magic + version
    0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f, // type section: () -> i32
    0x03, 0x02, 0x01, 0x00, // function section: func 0 : type 0
    0x07, 0x0a, 0x01, 0x06, b'a', b'n', b's', b'w', b'e', b'r', 0x00, 0x00, // export "answer"
    0x0a, 0x06, 0x01, 0x04, 0x00, 0x41, 0x2a, 0x0b, // code: (i32.const 42)
];

fn main() {
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::new(&engine, MODULE).expect("KAT module must compile");
    let mut store = wasmtime::Store::new(&engine, ());
    let instance =
        wasmtime::Instance::new(&mut store, &module, &[]).expect("KAT module must instantiate");

    let exports = module.exports().count();

    let answer = instance
        .get_typed_func::<(), i32>(&mut store, "answer")
        .expect("KAT module must export \"answer\"")
        .call(&mut store, ())
        .expect("KAT call must succeed");

    println!("KAT wasmtime exports={} answer={}", exports, answer);
}
