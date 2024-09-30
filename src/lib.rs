mod ir;
use backend::create_wasm;
use ir::{cell_zero, inst_combine, opt_simple_loops, parse};
mod backend;

// TODO make a function for displaying the IR
// TODO make the optimizations optional

// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn compile(program: &str) -> Vec<u8> {
    let mut ir = parse(&program.to_string());
    ir = inst_combine(&ir);
    ir = cell_zero(&ir);
    ir = opt_simple_loops(&ir);
    create_wasm(&ir)
}

// TODO just for testing
// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet(x: &str) -> String {
    format!("Hello world {}", x)
}
