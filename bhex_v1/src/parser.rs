use peg;

use crate::types::*;
use crate::util::Either;

peg::parser! {
pub grammar parser() for str {

rule _ = [' ' | '\t' | '\r']*

rule space() =[' ' | '\n' | '\t' | '\r']*

pub rule codes() -> Vec<Either<AST, (AST, AST)>>
    = space() c: code() ** ( _ "\n"+ _ ) space() { c }

rule code() -> Either<AST, (AST, AST)>
    = n: nth() { Either::Right(n) }
    / a: ast() { Either::Left(a) }

rule nth() -> (AST, AST)
    = b: ast() "[" _ i: ast() _ "]" { (b, i) }

rule ast() -> AST
    = _ v: add() _ { v }
    / _ v: pt()  _ { v }

rule pt() -> AST
    = _ v: integer()  _ { v }
    / _ v: omega()    _ { v }
    / _ v: aleph()    _ { v }
    / _ v: psi()      _ { v }
    / _ v: mahlo_f()  _ { v }
    / _ v: mahlo()    _ { v }

rule integer() -> AST
    = n: $(['0'] / (['1'..='9']['0'..='9']*)) {
        AST::from_int( n.parse::<u32>().unwrap() )
    }

rule omega() -> AST
    = "w" { AST::omega() }

rule aleph() -> AST
    = "O" { AST::aleph() }

rule mahlo() -> AST
    = "O" { AST::Mahlo(AST::one().to_box()) }

rule psi() -> AST
    = "p" _ "(" _ a: ast() _ ")" { AST::Psi(a.to_box()) }

rule mahlo_f() -> AST
    = "p" _ "(" _ a: ast() _ ")" { AST::Mahlo(a.to_box()) }

rule add() -> AST
    = l: pt() _ "+" _ r: ast() { AST::Add(l.to_box(), r.to_box()) }

}
}
