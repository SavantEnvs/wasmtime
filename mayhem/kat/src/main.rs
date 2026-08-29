// Known-answer-test probe for the Mayhem behavioral oracle (SPEC §6.3).
//
// Runs wasmparser over a FIXED, hand-assembled WebAssembly module and prints values it
// COMPUTES from the parse + validation. mayhem/test.sh asserts the exact output.
//
// The embedded module (a valid Wasm 1.0 module) is, byte for byte:
//   00 61 73 6d 01 00 00 00   ; "\0asm" magic + version 1
//   01 05 01 60 00 01 7f      ; Type section:     1 type  = () -> i32
//   03 02 01 00               ; Function section: 1 func, using type #0
//   0a 06 01 04 00 41 2a 0b   ; Code section:     1 body  = (i32.const 42)
// It defines exactly ONE type and ONE function, and MUST validate cleanly. The probe
// therefore prints, deterministically:  KAT wasmparser valid=1 types=1 funcs=1 const=42
//
// Every printed field is derived from wasmparser output, so neutering wasmparser (or the
// whole probe, via the sabotage shim) changes/removes the line and fails the oracle.

const MODULE: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // magic + version
    0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f, // type section: () -> i32
    0x03, 0x02, 0x01, 0x00, // function section: func 0 : type 0
    0x0a, 0x06, 0x01, 0x04, 0x00, 0x41, 0x2a, 0x0b, // code: (i32.const 42)
];

fn main() {
    // 1) Full validation MUST succeed for this fixed, valid module.
    let valid = wasmparser::validate(MODULE).is_ok();

    // 2) Count declared types + functions via the streaming parser (public API).
    let mut types = 0u32;
    let mut funcs = 0u32;
    for payload in wasmparser::Parser::new(0).parse_all(MODULE) {
        match payload.expect("KAT module must parse") {
            wasmparser::Payload::TypeSection(s) => types = s.count(),
            wasmparser::Payload::FunctionSection(s) => funcs = s.count(),
            _ => {}
        }
    }

    // 3) Decode the single operator in the function body and read the i32 constant,
    //    proving the operator reader actually decodes the code section.
    let mut konst: i32 = -1;
    for payload in wasmparser::Parser::new(0).parse_all(MODULE) {
        if let wasmparser::Payload::CodeSectionEntry(body) =
            payload.expect("KAT module must parse")
        {
            let mut reader = body.get_operators_reader().expect("operators reader");
            while !reader.eof() {
                if let wasmparser::Operator::I32Const { value } =
                    reader.read().expect("operator")
                {
                    konst = value;
                }
            }
        }
    }

    println!(
        "KAT wasmparser valid={} types={} funcs={} const={}",
        if valid { 1 } else { 0 },
        types,
        funcs,
        konst
    );
}
