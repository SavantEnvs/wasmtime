// Lifts a pair of enum values out of linear memory through `#[derive(Lift)]`: the first two input bytes
// are the discriminants a component writes to memory and returns as `tuple<e, e>`. A 256-case enum has
// every u8 as a valid discriminant, so lifting it must always succeed and give back the same bytes; the
// 255-case enum must accept exactly the bytes below 255.
#![no_main]

use libfuzzer_sys::{fuzz_target, Corpus};
use std::sync::OnceLock;
use wasmtime::component::Component;
use wasmtime::Engine;

#[path = "../src/cases.rs"]
mod cases;

struct Setup {
    engine: Engine,
    e255: Component,
    e256: Component,
}

fn setup() -> &'static Setup {
    static SETUP: OnceLock<Setup> = OnceLock::new();
    SETUP.get_or_init(|| {
        let engine = cases::engine();
        let e255 = cases::component(&engine, 255);
        let e256 = cases::component(&engine, 256);
        Setup { engine, e255, e256 }
    })
}

fuzz_target!(|data: &[u8]| -> Corpus {
    if data.len() < 2 {
        return Corpus::Reject;
    }
    let (a, b) = (data[0], data[1]);
    let s = setup();

    match cases::lift_e256(&s.engine, &s.e256, a, b) {
        Ok((x, y)) => assert_eq!((x as u8, y as u8), (a, b)),
        Err(e) => panic!("lifting discriminants ({a}, {b}) of a 256-case enum failed: {e:#}"),
    }

    match cases::lift_e255(&s.engine, &s.e255, a, b) {
        Ok((x, y)) => {
            assert!(a < 255 && b < 255);
            assert_eq!((x as u8, y as u8), (a, b));
        }
        Err(e) => assert!(a >= 255 || b >= 255, "lifting ({a}, {b}) of a 255-case enum failed: {e:#}"),
    }
    Corpus::Keep
});
