use crate::types::AST;

pub fn le(s: &AST, t: &AST) -> bool {
    s == t || lt(s, t)
}

pub fn lt(s: &AST, t: &AST) -> bool {
    if t == &AST::Zero {
        false
    } else if s == &AST::Zero {
        true
    } else if let AST::Add(a, b) = s {
        if let AST::Add(c, d) = t {
            (a == c && lt(b, d)) || lt(a, c)
        } else if let AST::Psi(_) = t {
            lt(a, t)
        } else if let AST::Mahlo(_) = t {
            lt(a, t)
        } else {
            unreachable!()
        }
    } else if let AST::Psi(a) = s {
        if let AST::Add(c, _) = t {
            le(s, c)
        } else if let AST::Psi(c) = t {
            lt(s, c)
        }else if let AST::Mahlo(_) = t {
            true
        } else {
            unreachable!()
        }
    } else if let AST::Mahlo(a) = s {
        if let AST::Add(c, _) = t {
            le(s, c)
        } else if let AST::Psi(_) = t {
            false
        }else if let AST::Mahlo(c) = t {
            lt(a, c)
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}
