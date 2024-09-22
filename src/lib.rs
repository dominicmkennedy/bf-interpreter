mod ir;
use backend::create_wasm;
use ir::parse;
mod backend;

// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn parse_prog(program: &str) {
    let ir = parse(&program.to_string());
    let wasm_prog = create_wasm(&ir);
}

// TODO make a function for displaying the IR

// TODO just for testing
// #[cfg(target_arch = "wasm32")]
pub fn greet(x: &str) -> String {
    format!("Hello world {}", x)
}
