use backend::create_wasm;
use clap::Parser;
use ir::{cell_zero, inst_combine, opt_simple_loops, parse, scan_opt};
use ir::{Inst, IR};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

mod backend;
mod ir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    bf_source: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    #[arg(short, long)]
    loop_opt: bool,

    #[arg(short, long)]
    scan_opt: bool,

    #[arg(short, long)]
    cell_zero_opt: bool,

    #[arg(short, long)]
    print_ir: bool,
}

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
    let cli = Cli::parse();
    let program: String = fs::read_to_string(cli.bf_source)?;

    let mut ir = parse(&program);
    ir = inst_combine(&ir);
    if cli.cell_zero_opt {
        ir = cell_zero(&ir);
    }
    if cli.loop_opt {
        ir = opt_simple_loops(&ir);
    }
    if cli.scan_opt {
        ir = scan_opt(&ir);
    }

    if cli.print_ir {
        print_ir(&ir);
    }

    let wasm_bytes = create_wasm(&ir);

    fs::write(cli.output, wasm_bytes)?;
    Ok(())
}
