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
                } else if let AST::Psi(_) = d {
                    //崩壊ゾーン
                    if let Some(AST::Psi(g)) = if t.is_non_zero() {
                        Some(nth(s, nth(t.clone(), AST::Zero)))
                    } else {
                        None
                    } {
                        if let (al, Some((il, ir))) = {
                            let (al, ar) = if let Some((l, r)) = a.clone().t_and_pt() {
                                (l, r)
                            } else {
                                (VecDeque::new(), *a.clone())
                            };
                            let ari = if let AST::Psi(i) = ar {
                                i
                            } else {
                                unreachable!()
                            };
                            let il_ir = ari.clone().t_and_pt();
                            (
                                al,
                                if let Some((_, ir)) = &il_ir {
                                    if dom(&ir) == AST::mahlo() {
                                        il_ir
                                    } else {
                                        None
                                    }
                                } else {
                                    if dom(&ari) == AST::mahlo() && *ari != AST::mahlo() {
                                        Some((VecDeque::new(), *ari))
                                    } else {
                                        None
                                    }
                                },
                            )
                        } {
                            let r = {
                                let mut il = il;
                                let u = nth(ir, *g);
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
                } else {
                    //結局正則基数になるパターン
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

pub fn dom(ast: &AST) -> AST {
    match ast {
        AST::Zero => AST::Zero,
        AST::Add(_, b) => dom(b),
        AST::Psi(a) => {
            if a == &AST::Zero.to_box() || dom(a) == AST::mahlo() {
                ast.clone()
            } else if dom(&a) == AST::one() {
                AST::omega()
            } else if lt(&dom(&a), ast) {
                dom(a)
            } else {
                AST::omega()
            }
        }
        AST::Mahlo(a) => {
            if a == &AST::Zero.to_box() {
                AST::mahlo()
            } else if dom(&a) == AST::one() {
                AST::omega()
            } else {
                dom(a)
            }
        }
    }
}
