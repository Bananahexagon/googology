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
            } else if d.1 == 0 || lt(&d.0, &s) {
                AST::Psi(nth(*a, t).to_box())
            } else if d.1 == 3 {
                if let Some(AST::Psi(g)) = if t.is_non_zero() {
                    Some(nth(s, nth(t.clone(), AST::Zero)))
                } else {
                    None
                } {
                    let (p, (mut l, mut il, mut iil, iir)) = {
                        let (l, r) = a.clone().t_and_pt();
                        let ri = if let AST::Psi(i) = r { i } else { panic!() };
                        let (il, ir) = ri.t_and_pt();
                        let iri = if let AST::Psi(i) = ir.clone() {
                            i
                        } else {
                            panic!()
                        };
                        let (iil, iir) = iri.t_and_pt();
                        if dom(&iir).0 == AST::card() && !(iil.is_empty() && iir == AST::card()) {
                            (2, (l, il, iil, iir))
                        } else if dom(&ir).0 == AST::mahlo()
                            && !(il.is_empty() && ir == AST::mahlo())
                        {
                            (1, (l, il, iil, iir))
                        } else {
                            (0, (l, il, iil, iir))
                        }
                    };
                    
                    if p == 2 {
                        let (_gl, h) = if *g != AST::Zero {
                            let (gl, gr) = g.t_and_pt();
                            if let AST::Psi(h) = gr {
                                (gl, h)
                            } else {
                                unimplemented!()
                            }
                        } else {
                            (VecDeque::new(), AST::Zero.to_box())
                        };
                        let ir = AST::Psi(
                            {
                                let u = nth(iir, h.t_and_pt().1);
                                if u != AST::Zero {
                                    iil.push_back(u);
                                }
                                AST::q_to_add(iil)
                            }
                            .to_box(),
                        );
                        let r = AST::Psi(
                            {
                                il.push_back(ir);
                                AST::q_to_add(il)
                            }
                            .to_box(),
                        );
                        l.push_back(r);
                        AST::Psi(AST::q_to_add(l).to_box())
                    } else if p == 1 {
                        let ir = AST::Psi(
                            {
                                iil.push_back(iir);
                                AST::q_to_add(iil)
                            }
                            .to_box(),
                        );
                        let r = AST::Psi(
                            {
                                let u = nth(ir, *g);
                                if u != AST::Zero {
                                    il.push_back(u);
                                }
                                AST::q_to_add(il)
                            }
                            .to_box(),
                        );
                        l.push_back(r);
                        AST::Psi(AST::q_to_add(l).to_box())
                    } else if p == 0 {
                        AST::Psi(nth(*a, AST::Psi(g)).to_box())
                    } else {
                        unimplemented!()
                    }
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
