use crate::{compare::lt, types::AST};

pub fn nth(s: AST, t: AST) -> AST {
    match s.clone() {
        AST::Zero => AST::Zero,
        AST::Add(a, b) => {
            if *b == AST::Zero {
                *a
            } else {
                let r = nth(*b, t);
                if r == AST::Zero {
                    *a
                } else {
                    AST::Add(a, r.to_box())
                }
            }
        }
        AST::Psi(a) => {
            let d = dom(&a);
            if d == AST::Zero {
                t
            } else if d == AST::one() {
                if t.is_successor() {
                    AST::Add(
                        nth(s.clone(), nth(t, AST::Zero)).to_box(),
                        nth(s, AST::one()).to_box(),
                    )
                } else {
                    AST::Psi(nth(*a, AST::Zero).to_box())
                }
            } else {
                if lt(&a, &s) {
                    AST::Psi(nth(*a, t).to_box())
                } else if let AST::Psi(_) = d {
                    unimplemented!()
                } else {
                    t
                }
            }
        }
        AST::Mahlo(a) => {
            let d = dom(&a);
            if d == AST::Zero {
                t
            } else if d == AST::one() {
                if t.is_successor() {
                    AST::Add(
                        nth(s.clone(), nth(t, AST::Zero)).to_box(),
                        nth(s, AST::one()).to_box(),
                    )
                } else {
                    AST::Mahlo(nth(*a, AST::Zero).to_box())
                }
            } else {
                AST::Mahlo(nth(*a, t).to_box())
            }
        }
    }
}

fn dom(ast: &AST) -> AST {
    match ast {
        AST::Zero => AST::Zero,
        AST::Add(_, b) => dom(b),
        AST::Psi(a) => {
            if a == &AST::Zero.to_box() || a == &AST::one().to_box() || a == &AST::mahlo().to_box()
            {
                ast.clone()
            } else {
                dom(a)
            }
        }
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
