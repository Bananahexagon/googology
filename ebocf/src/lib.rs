mod compare;
mod expand;
mod parser;
mod types;
mod util;

use expand::nth;
use types::AST;
use util::Either;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub unsafe fn expand(code: &str) -> String {
    let asts = {
        let expanded = main(code);
        if let Ok(ast) = expanded {
            ast
        } else {
            return expanded.err().unwrap();
        }
    };
    let mut r = String::new();
    for ast in asts {
        r.push_str(&format!("{}\n", ast.to_string()))
    }
    r
}

fn main(code: &str) -> Result<Vec<AST>, String> {
    if let Ok(asts) = parser::parser::codes(code) {
        let mut r = Vec::new();
        for ast in asts {
            r.push(match ast {
                Either::Left(a) => a,
                Either::Right((b, n)) => nth(b, n),
            })
        }
        Ok(r)
    } else {
        Err("parse error".to_string())
    }
}
