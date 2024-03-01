use crate::{compare::lt, types::AST};

pub fn is_ot(t: &AST) -> bool {
    match t {
        AST::Zero => true,
        AST::Add(a, b) => lt(&a, &b) && is_ot(&a) && is_ot(b),
        AST::Psi(a) => if lt(a, &AST::aleph()) {
            is_ot(a) && lt(a, t)
        } else {
            is_ut(a) || lt(a.t_and_pt_ref().1,t)
        },
        _ => false,
    }
}

pub fn is_ut(t: &AST) -> bool {
    match t {
        AST::Omega(a) => is_ot(a) || is_ut(a),
        AST::Add(a, b) => lt(&a, &b) && is_ut(&a) && is_ut(b),
        _ => false,
    }
}
