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
            let d = dom(&a).0;
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
                if lt(&d, &s) {
                    AST::Psi(nth(*a, t).to_box())
                } else if d == AST::card()
                    || (if let AST::Psi(b) = a.clone().t_and_pt().1 {
                        dom(&b).0 == AST::card()
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
                        let (al, (il, (iil, iir))) = {
                            let (al, ar) = a.clone().t_and_pt();
                            let ari = if let AST::Psi(i) = ar {
                                i
                            } else {
                                unreachable!()
                            };
                            let (il, ir) = ari.t_and_pt();
                            let iri = if let AST::Psi(i) = ir {
                                i
                            } else {
                                unreachable!()
                            };
                            let (iil, iir) = iri.t_and_pt();
                            (al, (il, (iil, iir)))
                        };
                        println!(
                            "({:?}+p({:?}+p({:?}+{})))",
                            //a.to_string(),
                            al.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                            il.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                            iil.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                            iir.to_string()
                        );
                        let v = [al, il, iil];
                        if let Some(i) = v.iter().position(|v| !v.is_empty()) {
                            let mut r = iir.clone();
                            for (j, l) in v.into_iter().enumerate().rev() {
                                let mut l = l;
                                let n = if i == j {
                                    unimplemented!()
                                } else {
                                    r
                                };
                                if n != AST::Zero {
                                    l.push_back(n);
                                }
                                r = AST::Psi(AST::q_to_add(l).to_box());
                            }
                            r
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
