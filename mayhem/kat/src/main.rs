// Lifts (7, 254) and (0, 130) through the 255-case enum's `#[derive(Lift)]` and checks that 255 is
// rejected; works the same on every wasmtime that has the derive. The prints are what test.sh asserts.
#[path = "../../fuzz/src/cases.rs"]
mod cases;

fn main() {
    let engine = cases::engine();
    let component = cases::component(&engine, 255);
    for (a, b) in [(7u8, 254u8), (0, 130)] {
        let (x, y) = cases::lift_e255(&engine, &component, a, b).expect("in-range discriminants lift");
        println!("KAT lift255 {a},{b} -> {},{}", x as u8, y as u8);
    }
    match cases::lift_e255(&engine, &component, 255, 1) {
        Ok(_) => println!("KAT lift255 255,1 -> accepted"),
        Err(e) => println!("KAT lift255 255,1 -> {}", e.root_cause()),
    }
}
