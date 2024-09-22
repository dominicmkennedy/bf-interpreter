use backend::create_wasm;
use ir::{cell_zero, inst_combine, opt_simple_loops, parse};
use std::error::Error;
use std::fs;
use std::env;
mod backend;
mod ir;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let program: String = fs::read_to_string(args[1].clone())?;

    let mut ir = parse(&program);
    ir = inst_combine(&ir);
    cell_zero(&mut ir);
    ir = opt_simple_loops(&ir);

    // let mut new_ir: Vec<Inst> = Vec::new();
    // for ((start, end), new_loop) in replacements {
    //     let air = [&ir[0..start], &new_loop, &ir[end + 1..]].concat();
    // }

    // for i in ir.to_vec() {
    //     println!("{:?}", i);
    // }

    let wasm_bytes = create_wasm(&ir);

    fs::write("rust_prog.wasm", wasm_bytes)?;
    Ok(())
}
