use backend::create_wasm;
use ir::cell_zero;
use std::env;
use std::error::Error;
use std::fs;
mod ir;
use crate::ir::{inst_combine, parse};
mod backend;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let program: String = fs::read_to_string(args[1].clone())?;

    let mut ir = parse(&program);
    ir = inst_combine(&ir);
    cell_zero(&mut ir);

    // let inner_loops = get_inner_loops(&ir);
    // for (start, end) in inner_loops {
    //     let simple = is_simple(&ir, start, end);
    //     let loop_ins = &ir[start + 1..end];
    //     println!("{}:", simple);
    //     for (ins, ct) in loop_ins {
    //         println!("\t{:?}: {}", ins, ct);
    //     }
    // }

    let wasm_bytes = create_wasm(&ir);

    fs::write("rust_prog.wasm", wasm_bytes)?;
    Ok(())
}
