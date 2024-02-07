use crate::{compare::lt, types::AST};

pub fn nth(s: AST, t: AST) -> AST {
    unimplemented!()
}

fn dom(ast: &AST) -> AST {
    match ast {
        AST::Zero => AST::Zero,
        AST::Add(_, b) => dom(b),
        AST::Psi(a) => {
            if a == &AST::Zero.to_box() || a == &AST::one().to_box() || a == &AST::mahlo().to_box() {
                ast.clone()
            } else {
                dom(a)
            }
        },
        AST::Mahlo(a) => {
            if a == &AST::Zero.to_box() {
                AST::mahlo()
            } else if a == &AST::one().to_box() {
                AST::omega()
            } else {
                dom(a)
            }
        }
    }
}
