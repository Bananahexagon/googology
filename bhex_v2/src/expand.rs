use crate::{compare::lt, types::AST};

pub fn nth(body: AST, n: AST) -> AST {
    match body.clone() {
        AST::Zero => AST::Zero,
        AST::Add(l, r) => {
            if *r == AST::Zero {
                *l
            } else {
                AST::Add(l, nth(*r, n).to_box())
            }
        }
        AST::Psi(a, b) => {
            if dom(&b) == AST::Zero {
                if dom(&a) == AST::Zero || dom(&a) == AST::one() {
                    n
                } else {
                    AST::Psi(nth(*a, n).to_box(), AST::Zero.to_box())
                }
            } else if dom(&b) == AST::one() {
                if n.is_successor() {
                    AST::Add(
                        nth(body.clone(), nth(n, AST::Zero)).to_box(),
                        nth(body, AST::one()).to_box(),
                    )
                } else {
                    AST::Psi(a.to_box(), nth(*b, AST::Zero).to_box())
                }
            } else {
                if lt(&dom(&b), &body) {
                    AST::Psi(a.to_box(), nth(*b, n).to_box())
                } else if let AST::Psi(c, _) = dom(&b) {
                    match (n.to_number(), nth(body, nth(n, AST::Zero))) {
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
        AST::Add(_, r) => dom(r),
        AST::Psi(l, r) => {
            if dom(r) == AST::Zero {
                if dom(l) == AST::Zero || dom(l) == AST::one() {
                    ast.clone()
                } else {
                    dom(r)
                }
            } else if dom(r) == AST::one() {
                AST::omega()
            } else {
                if lt(&dom(r), ast) {
                    dom(r)
                } else {
                    AST::omega()
                }
            }
        }
    }
}
