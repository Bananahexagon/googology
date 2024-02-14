use peg;

use crate::types::*;
use crate::util::Either;

peg::parser! {
pub grammar parser() for str {

rule _ = [' ' | '\t' | '\r']*

rule space() =[' ' | '\n' | '\t' | '\r']*

pub rule codes() -> Vec<Either<AST, Either<AST, (AST, AST)>>>
    = space() c: code() ** ( _ "\n"+ _ ) space() { c }

rule code() -> Either<AST, Either<AST, (AST, AST)>>
    = n: nth() { Either::Right(Either::Right(n)) }
    / d: dom() { Either::Right(Either::Left(d)) }
    / a: ast() { Either::Left(a) }

rule nth() -> (AST, AST)
    = b: ast() "[" _ i: ast() _ "]" { (b, i) }

rule dom() -> AST
    = v: ast() _ "dom" { v }

pub rule ast() -> AST
    = _ v: add() _ { v }
    / _ v: pt()  _ { v }

rule pt() -> AST
    = _ v: integer()  _ { v }
    / _ v: omega()    _ { v }
    / _ v: aleph()    _ { v }
    / _ v: mahlo()    _ { v }
    / _ v: card()     _ { v }
    / _ v: psi()      _ { v }
    / _ v: card_f()   _ { v }

rule integer() -> AST
    = n: $(['0'] / (['1'..='9']['0'..='9']*)) {
        AST::from_int( n.parse::<u32>().unwrap() )
    }

rule omega() -> AST
    = "w" { AST::omega() }

rule aleph() -> AST
    = "W" { AST::aleph() }

rule mahlo() -> AST
    = "M" { AST::mahlo() }

rule psi() -> AST
    = "p" _ "(" _ a: ast() _ ")" { AST::Psi(a.to_box()) }

rule card_f() -> AST
    = "c" _ "(" _ a: ast() _ ")" { AST::Card(a.to_box()) }

rule card() -> AST
    = "C" { AST::Card(AST::Zero.to_box()) }

rule add() -> AST
    = l: pt() _ "+" _ r: ast() { AST::Add(l.to_box(), r.to_box()) }

}
}
