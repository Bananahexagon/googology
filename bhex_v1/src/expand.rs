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
                } else if let AST::Psi(b) = d {
                    //崩壊ゾーン
                    if dom(&b) == AST::mahlo() {
                        let mut r = None;
                        if t.is_non_zero() {
                            match nth(s, nth(t, AST::Zero)) {
                                AST::Psi(g) => {
                                    let ar = if let Some((_, r)) = a.clone().t_and_pt() {
                                        r
                                    } else {
                                        *a.clone()
                                    };
                                    let ai = if let AST::Psi(u) = ar {
                                        *u
                                    } else {
                                        unreachable!()
                                    };
                                    r = Some(if let Some((c, _)) = ai.clone().t_and_pt() {
                                        AST::Psi(AST::Psi(
                                            AST::Add(
                                                c.to_box(),
                                                AST::Psi(nth(*b, *g).to_box()).to_box(),
                                            )
                                            .to_box(),
                                        ).to_box())
                                    } else {
                                        AST::Psi(AST::Psi(AST::Psi(nth(*b, *g).to_box()).to_box()).to_box())
                                    })
                                }
                                _ => (),
                            }
                        }
                        r.unwrap_or(AST::Psi(nth(*a, AST::Zero).to_box()))
                    } else {
                        let mut r = None;
                        if t.is_non_zero() {
                            match nth(s, nth(t, AST::Zero)) {
                                AST::Psi(g) => {
                                    r = Some(AST::Psi(nth(*a.clone(), AST::Psi(g)).to_box()))
                                }
                                _ => (),
                            }
                        }
                        r.unwrap_or(AST::Psi(nth(*a, AST::Zero).to_box()))
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
