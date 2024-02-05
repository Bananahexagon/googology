mod parser;
mod types;
mod expand;
mod compare;
mod util;

use expand::nth;
use types::AST;
use util::Either;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub unsafe fn expand(code: &str) -> String {
    let ast = {
        let parsed = parse(code);
        if let Ok(ast) = parsed {
            ast
        } else {
            return parsed.err().unwrap();
        }
    };
    ast.to_json()
}

fn parse(code: &str) -> Result<AST, String> {
    if let Ok(ast) = parser::parser::code(code) {
        Ok(match ast {
            Either::Left(ast) => ast,
            Either::Right((b, n)) => nth(b, n)
        })
    } else {
        Err("parse error".to_string())
    }
}