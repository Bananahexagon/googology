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
    = a: ast() _ "<" _ b: ast() { Either::Right(Either::Right((a, b))) }
    / a: ast() _ "is" { Either::Right(Either::Left(a)) }
    / a: ast() { Either::Left(a) }

pub rule ast() -> AST
    = _ v: add() _ { v }
    / _ v: pt()  _ { v }

rule pt() -> AST
    = _ v: integer()  _ { v }
    / _ v: omega()    _ { v }
    / _ v: aleph()    _ { v }
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

rule psi() -> AST
    = "p" _ "(" _ a: ast() _ ")" { AST::Psi(a.to_box()) }

rule card_f() -> AST
    = "o" _ "(" _ a: ast() _ ")" { AST::Omega(a.to_box()) }

rule add() -> AST
    = l: pt() _ "+" _ r: ast() { AST::Add(l.to_box(), r.to_box()) }

}
}
