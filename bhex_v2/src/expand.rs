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
            if d.0 == AST::Zero {
                t
            } else if d.0 == AST::one() {
                if t.is_successor() {
                    AST::Add(
                        nth(s.clone(), nth(t, AST::Zero)).to_box(),
                        nth(s, AST::one()).to_box(),
                    )
                } else {
                    AST::Psi(nth(*a, AST::Zero).to_box())
                }
            } else if d.1 == 0 {
                AST::Psi(nth(*a, t).to_box())
            } else if d.1 == 3 {
                if let Some(AST::Psi(g)) = if t.is_non_zero() {
                    Some(nth(s, nth(t.clone(), AST::Zero)))
                } else {
                    None
                } {
                    AST::Psi(nth(*a, AST::Psi(g)).to_box())
                } else {
                    AST::Psi(nth(*a, AST::Zero).to_box())
                }
            } else {
                t
            }
        }
        AST::Card(a) => {
            if *a == AST::Zero {
                t
            } else if dom(&a).0 == AST::one() {
                AST::omega()
            } else {
                AST::Card(nth(*a, t).to_box())
            }
        }
    }
}

pub fn dom(s: &AST) -> (AST, u32) {
    match s {
        AST::Zero => (AST::Zero, 0),
        AST::Add(_, b) => dom(b),
        AST::Psi(a) => {
            let d = dom(a);
            if **a == AST::Zero {
                (AST::one(), 0)
            } else if d.0 == AST::one() {
                (AST::omega(), 0)
            } else if lt(&d.0, s) {
                d
            } else if d.1 <= 2 {
                (s.clone(), d.1 + 1)
            } else {
                (AST::omega(), 0)
            }
        }
        AST::Card(a) => {
            if **a == AST::Zero {
                (s.clone(), 1)
            } else if dom(&a).0 == AST::one() {
                (AST::omega(), 0)
            } else {
                dom(a)
            }
        }
    }
}
