use crate::types::AST;

pub fn le(s: &AST, t: &AST) -> bool {
    s == t || lt(s, t)
}

pub fn lt(s: &AST, t: &AST) -> bool {
    if let AST::Zero = t {
        false
    } else if let AST::Zero = s {
        true
    } else if let AST::Add(a, b) = s {
        if let AST::Add(c, d) = t {
            (a == c && lt(b, d)) || lt(a, c)
        } else if let AST::Psi(_) = t {
            lt(a, t)
        } else if let AST::Card(_) = t {
            lt(a, t)
        } else {
            unreachable!()
        }
    } else if let AST::Psi(a) = s {
        if let AST::Add(b, _) = t {
            le(s, b)
        } else if let AST::Psi(b) = t {
            lt(a, b)
        } else if let AST::Card(_) = t {
            true
        } else {
            unreachable!()
        }
    } else if let AST::Card(a) = s {
        if let AST::Add(b, _) = t {
            le(s, b)
        } else if let AST::Psi(_) = t {
            false
        } else if let AST::Card(b) = t {
            lt(a, b)
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}
