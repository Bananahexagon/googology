use peg;

use crate::types::*;
use crate::util::Either;

peg::parser! {
pub grammar parser() for str {

rule _ = [' ' | '\t' | '\r']*

pub rule codes() -> Vec<Either<AST, (AST, AST)>>
    = c: code() ** ( _ "\n"+ _ ) { c }

rule code() -> Either<AST, (AST, AST)>
    = n: nth() { Either::Right(n)}
    / a: ast() { Either::Left(a)}

rule nth() -> (AST, AST)
    = b: pt() "[" _ i: ast() _ "]" { (b, i) }

rule ast() -> AST
    = _ v: add() _ { v }
    / _ v: pt()  _ { v }

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

}
}
