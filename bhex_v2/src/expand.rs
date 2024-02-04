use crate::types::AST;

pub fn main(ast: AST) -> AST {
    match ast {
        AST::Zero => AST::Zero,
        AST::Add(l, r) => AST::Add(main(*l).to_box(), main(*r).to_box()),
        AST::Psi(l, r) => AST::Psi(main(*l).to_box(), main(*r).to_box()),
        AST::Nth(b, i) => nth(*b, *i),
    }
}

pub fn nth(body: AST, nth: AST) -> AST {
    unimplemented!()
}
