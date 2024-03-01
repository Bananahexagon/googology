mod compare;
mod normal;
mod parser;
mod types;
mod util;

use compare::lt;
use normal::is_ot;
use util::Either;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub unsafe fn expand(code: &str) -> String {
    let res = {
        let expanded = main(code);
        if let Ok(ast) = expanded {
            ast
        } else {
            return expanded.err().unwrap();
        }
    };
    let mut r = String::new();
    for s in res {
        r.push_str(&format!("{}\n", s))
    }
    r
}

fn main(code: &str) -> Result<Vec<String>, String> {
    if let Ok(asts) = parser::parser::codes(code) {
        let mut r = Vec::new();
        for ast in asts {
            r.push(match ast {
                Either::Left(a) => a.to_string(),
                Either::Right(Either::Left(a)) => if is_ot(&a) { "OK" } else { "ERR" }.to_string(),
                Either::Right(Either::Right((a, b))) => lt(&a, &b).to_string(),
            })
        }
        Ok(r)
    } else {
        Err("parse error".to_string())
    }
}
