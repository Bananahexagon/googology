use peg;

use crate::types::*;

peg::parser! {
pub grammar parser() for str {

rule _ =  [' ' | '\t' | '\r' | '\n']*

pub rule ast() -> AST
    = _ v: add() _ { v }
    / _ v: pt()  _ { v }
    / _ v: nth() _ { v }

rule pt() -> AST
    = _ v: zero()  _ { v }
    / _ v: one()   _ { v }
    / _ v: omega() _ { v }
    / _ v: psi()   _ { v }

rule zero() -> AST
    = "0" { AST::Zero }

rule one() -> AST
    = "1" { AST::one() }

rule omega() -> AST
    = "w" { AST::Psi(AST::Zero.to_box(), AST::one().to_box()) }

rule psi() -> AST
    = "p" _ "(" _ l: ast() _ "," _ r: ast() _ ")" { AST::Psi(l.to_box(), r.to_box()) }

rule add() -> AST
    = l: pt() _ "+" _ r: ast() { AST::Add(l.to_box(), r.to_box()) }

rule nth() -> AST
    = b: pt() "[" i: ast() "]" { AST::Nth(b.to_box(), i.to_box()) }

}
}
