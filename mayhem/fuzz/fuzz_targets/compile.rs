#![no_main]

use libfuzzer_sys::fuzz_target;
use std::default::Default;

// Same pipeline as cranelift/isle/fuzz/fuzz_targets/compile.rs: lex, parse, compile.
fuzz_target!(|s: &str| {
    let lexer = match cranelift_isle::lexer::Lexer::from_str(s, "fuzz-input.isle") {
        Ok(l) => l,
        Err(_) => return,
    };
    let defs = match cranelift_isle::parser::parse(lexer) {
        Ok(d) => d,
        Err(_) => return,
    };
    let code = match cranelift_isle::compile::compile(&defs, &Default::default()) {
        Ok(c) => c,
        Err(_) => return,
    };
    drop(code);
});
