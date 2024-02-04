use peg;

use crate::types::*;

peg::parser! {
pub grammar parser() for str {

rule _ =  [' ' | '\t' | '\r' | '\n']*

pub rule ast() -> AST
    = pt()
    / add()

rule pt() -> AST
    = psi()
    / zero()
    / one()
    / omega()

rule zero() -> AST
    = "0" { AST::zero() }

rule one() -> AST
    = "1" { AST::one() }

rule omega() -> AST
    = "w" { AST::Psi(AST::zero().to_box(), AST::one().to_box()) }

rule psi() -> AST
    = "p" _ "(" _ l: ast() _ "," _ r: ast() _ ")" { AST::Psi(l.to_box(), r.to_box()) }

rule add() -> AST
    = l: pt() _ r: ast() { AST::Add(l.to_box(), r.to_box()) }

}
}
