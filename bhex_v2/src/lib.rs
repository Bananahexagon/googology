mod parser;
mod types;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub unsafe fn parse(name: &str) -> String {
    if let Ok(ast) = parser::parser::ast(name) {
        ast.to_json()
    } else {
        "parse error".to_string()
    }
}
