mod ir;
use backend::create_wasm;
use ir::{cell_zero, inst_combine, opt_simple_loops, parse, scan_opt};
mod backend;

// TODO make a function for displaying the IR
// TODO make the optimizations optional

// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn compile(
    program: &str,
    do_cell_zero_opt: bool,
    do_simple_loop_opt: bool,
    do_scan_opt: bool,
) -> Vec<u8> {
    let mut ir = parse(&program.to_string());
    ir = inst_combine(&ir);
    if do_cell_zero_opt {
        ir = cell_zero(&ir);
    }
    if do_simple_loop_opt {
        ir = opt_simple_loops(&ir);
    }
    if do_scan_opt {
        ir = scan_opt(&ir);
    }
    create_wasm(&ir)
}
