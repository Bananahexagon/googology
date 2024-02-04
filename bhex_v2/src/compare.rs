use crate::{expand::nth, types::AST};

pub fn le(l: AST, r: AST) -> bool {
    l == r || lt(l, r)
}

pub fn lt(l: AST, r: AST) -> bool {
    if r == AST::Zero {
        false
    } else if l == AST::Zero {
        true
    } else if let AST::Add(a, b) = l {
        if let AST::Add(c, d) = r {
            (*a == *c && lt(*b, *d)) || lt(*a, *c)
        } else if let AST::Psi(_, _) = r {
            lt(*a, r)
        } else if let AST::Nth(c, d) = r {
            lt(AST::Add(a, b), nth(*c, *d))
        } else {
            unreachable!()
        }
    } else {
        unimplemented!()
    }
}
