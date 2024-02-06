use crate::{compare::lt, types::AST};

pub fn nth(s: AST, t: AST) -> AST {
    match s.clone() {
        AST::Zero => AST::Zero,
        AST::Add(a, b) => {
            if *b == AST::Zero {
                *a
            } else {
                AST::Add(a, nth(*b, t).to_box())
            }
        }
        AST::Psi(a, b) => {
            if dom(&b) == AST::Zero {
                if dom(&a) == AST::Zero || dom(&a) == AST::one() {
                    t
                } else {
                    AST::Psi(nth(*a, t).to_box(), AST::Zero.to_box())
                }
            } else if dom(&b) == AST::one() {
                if t.is_successor() {
                    AST::Add(
                        nth(s.clone(), nth(t, AST::Zero)).to_box(),
                        nth(s, AST::one()).to_box(),
                    )
                } else {
                    AST::Psi(a.to_box(), nth(*b, AST::Zero).to_box())
                }
            } else {
                if lt(&dom(&b), &s) {
                    AST::Psi(a.to_box(), nth(*b, t).to_box())
                } else if let AST::Psi(c, _) = dom(&b) {
                    match (t.to_number(), nth(s, nth(t, AST::Zero))) {
                        (Some(m), AST::Psi(d, g)) if m != 0 && a == d => AST::Psi(
                            a,
                            nth(
                                *b,
                                AST::Psi(nth(*c, AST::Zero).to_box(), g),
                            )
                            .to_box(),
                        ),
                        _ => AST::Psi(
                            a,
                            nth(
                                *b,
                                AST::Psi(nth(*c, AST::Zero).to_box(), AST::Zero.to_box()),
                            )
                            .to_box(),
                        ),
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }
}

fn dom(ast: &AST) -> AST {
    match ast {
        AST::Zero => AST::Zero,
        AST::Add(_, b) => dom(b),
        AST::Psi(a, b) => {
            if dom(b) == AST::Zero {
                if dom(a) == AST::Zero || dom(a) == AST::one() {
                    ast.clone()
                } else {
                    dom(b)
                }
            } else if dom(b) == AST::one() {
                AST::omega()
            } else {
                if lt(&dom(b), ast) {
                    dom(b)
                } else {
                    AST::omega()
                }
            }
        }
    }
}
