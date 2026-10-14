use cranelift_isle::{ast::Def, compile, lexer::Lexer, parser};

const SRC: &str = "(type u32 (primitive u32))
(decl F (u32) u32)
(rule 7 (F x) x)
(rule (F 0) 1)
";

fn main() {
    let defs = parser::parse(Lexer::from_str(SRC, "kat.isle").expect("lex")).expect("parse");
    for d in &defs.defs {
        if let Def::Rule(r) = d {
            println!("KAT rule prio {:?}", r.prio);
        }
    }
    let code = compile::compile(&defs, &Default::default()).expect("compile");
    println!("KAT codegen constructor_F {}", code.contains("fn constructor_F"));
    let bad = Lexer::from_str("(rule (", "kat.isle").map(parser::parse);
    println!("KAT truncated rule rejected {}", matches!(bad, Ok(Err(_))));
}
