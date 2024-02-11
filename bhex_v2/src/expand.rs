use std::collections::VecDeque;

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
                if lt(&dom(&a), &s) {
                    AST::Psi(nth(*a, t).to_box())
                } else if d == AST::card()
                    || (if let AST::Psi(b) = a.clone().t_and_pt().1 {
                        dom(&b) == AST::card()
                    } else {
                        false
                    })
                {
                    t
                } else {
                    if let Some(AST::Psi(g)) = if t.is_non_zero() {
                        Some(nth(s, nth(t.clone(), AST::Zero)))
                    } else {
                        None
                    } {
                        if let (al, Some((il, ir))) = {
                            let (al, ar) = a.clone().t_and_pt();
                            let ari = if let AST::Psi(i) = ar {
                                i
                            } else {
                                unreachable!()
                            };
                            let il_ir = ari.clone().t_and_pt();
                            (
                                al,
                                if dom(&il_ir.1) == AST::mahlo() {
                                    Some(il_ir)
                                } else {
                                    None
                                },
                            )
                        } {
                            let r = {
                                let mut il = il;
                                let u = nth(ir.clone(), *g);
                                if u != AST::Zero {
                                    il.push_back(u);
                                }
                                AST::q_to_add(il)
                            };
                            let mut al = al;
                            al.push_back(r);
                            AST::Psi(AST::Psi(AST::q_to_add(al).to_box()).to_box())
                        } else {
                            AST::Psi(nth(*a, AST::Psi(g)).to_box())
                        }
                    } else {
                        AST::Psi(nth(*a, AST::Zero).to_box())
                    }
                }
            }
        }
        AST::Card(a) => {
            if *a == AST::Zero {
                t
            } else if dom(&a) == AST::one() {
                AST::omega()
            } else {
                AST::Card(nth(*a, t).to_box())
            }
        }
    }
}

pub fn dom(s: &AST) -> AST {
    match s {
        AST::Zero => AST::Zero,
        AST::Add(_, b) => dom(b),
        AST::Psi(a) => {
            if **a == AST::Zero
                || dom(a) == AST::card()
                || if let AST::Psi(b) = a.t_and_pt().1 {
                    dom(b) == AST::card()
                } else {
                    false
                }
            {
                s.clone()
            } else if dom(a) == AST::one() {
                AST::omega()
            } else if lt(&dom(&a), s) {
                dom(a)
            } else {
                AST::omega()
            }
        }
        AST::Card(a) => {
            if **a == AST::Zero {
                AST::card()
            } else if dom(&a) == AST::one() {
                AST::omega()
            } else {
                dom(a)
            }
        }
    }
}
