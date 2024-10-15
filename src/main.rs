use backend::create_wasm;
use ir::{cell_zero, inst_combine, opt_simple_loops, parse, scan_opt};
use ir::{Inst, IR};
use std::env;
use std::error::Error;
use std::fs;
mod backend;
mod ir;

// TODO make this look like John's IR output
fn print_ir(ir: &IR) -> () {
    let mut loop_nest = 0;
    for i in ir.to_vec() {
        if i == Inst::LoopEnd {
            loop_nest -= 1;
        }
        for _ in 0..loop_nest {
            print!("\t");
        }
        println!("{:?}", i);
        if i == Inst::LoopStart {
            loop_nest += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let program: String = fs::read_to_string(args[1].clone())?;

    let mut ir = parse(&program);
    ir = inst_combine(&ir);
    ir = cell_zero(&ir);
    ir = opt_simple_loops(&ir);
    ir = scan_opt(&ir);

    // print_ir(&ir);

    let wasm_bytes = create_wasm(&ir);

    fs::write("rust_prog.wasm", wasm_bytes)?;
    Ok(())
}
