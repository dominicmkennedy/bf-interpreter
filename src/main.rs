use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::{fs, usize};
use wasm_encoder::{
    BlockType, CodeSection, ExportKind, ExportSection, Function, FunctionSection, ImportSection,
    Instruction, MemArg, MemorySection, MemoryType, Module, TypeSection, ValType,
};

fn null_mem_arg() -> MemArg {
    MemArg {
        offset: 0,
        align: 0,
        memory_index: 0,
    }
}

fn print(f: &mut Function, js_log: u32) {
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::Call(js_log));
}

fn add(f: &mut Function, ct: usize) {
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::I32Const(ct as i32));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::I32Store8(null_mem_arg()));
}

fn sub(f: &mut Function, ct: usize) {
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::I32Const(ct as i32));
    f.instruction(&Instruction::I32Sub);
    f.instruction(&Instruction::I32Store8(null_mem_arg()));
}

// TODO opt for [>] and [<]
// fn scan(f: &mut Function) {
//     f.instruction(&Instruction::LocalGet(0));
//     f.instruction(&Instruction::V128Load(null_mem_arg()));
//     f.instruction(&Instruction::I8x16ExtractLaneU(0));
//     f.instruction(&Instruction::I32Eqz);
// }

fn set_0(f: &mut Function) {
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Const(0));
    f.instruction(&Instruction::I32Store8(null_mem_arg()));
}

fn dp_r(f: &mut Function, ct: usize) {
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Const(ct as i32));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(0));
}

fn dp_l(f: &mut Function, ct: usize) {
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Const(ct as i32));
    f.instruction(&Instruction::I32Sub);
    f.instruction(&Instruction::LocalSet(0));
}

fn loop_start(f: &mut Function) {
    f.instruction(&Instruction::Block(BlockType::Empty));
    f.instruction(&Instruction::Loop(BlockType::Empty));
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::I32Eqz);
    f.instruction(&Instruction::BrIf(1));
}

fn loop_end(f: &mut Function) {
    f.instruction(&Instruction::Br(0));
    f.instruction(&Instruction::End);
    f.instruction(&Instruction::End);
}

fn get_inner_loops(ir: &Vec<(Inst, usize)>) -> HashMap<usize, usize> {
    let mut inner_loops: HashMap<usize, usize> = HashMap::new();
    let mut top_paren: Option<usize> = None;
    for (idx, (ins, _)) in ir.iter().enumerate() {
        match ins {
            Inst::LoopStart => top_paren = Some(idx),
            Inst::LoopEnd => match top_paren {
                None => (),
                Some(x) => {
                    inner_loops.insert(x, idx);
                    top_paren = None;
                }
            },
            _ => (),
        }
    }

    inner_loops
}

fn is_simple(ir: &Vec<(Inst, usize)>, start: usize, end: usize) -> bool {
    let loop_ins = &ir[start + 1..end];
    let mut ret = true;

    // TODO should be resiliant to the fact that i/o may be folded at some point
    if loop_ins.contains(&(Inst::In, 1)) || loop_ins.contains(&(Inst::Out, 1)) {
        ret = false;
    }

    let mut ptr_change: i32 = 0;
    let mut loop_ptr_changed = false;
    for (ins, ct) in loop_ins {
        if *ins == Inst::Right {
            ptr_change += *ct as i32;
        }
        if *ins == Inst::Left {
            ptr_change -= *ct as i32;
        }
        // TODO this may break if the ir gets more complicated
        if (*ins == Inst::Add || *ins == Inst::Sub) && ptr_change == 0 {
            match loop_ptr_changed {
                true => {
                    ret = false;
                }
                false => {
                    loop_ptr_changed = true;
                }
            }
        }
    }

    if ptr_change != 0 {
        ret = false;
    }

    ret
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, PartialOrd, Ord)]
enum Inst {
    Add,
    Sub,
    Left,
    Right,
    In,
    Out,
    LoopStart,
    LoopEnd,
    Zero,
    Nop,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let program: String = fs::read_to_string(args[1].clone())?;

    let mut ir: Vec<(Inst, usize)> = vec![];
    let mut idx = 0;
    // TODO do a dumb translation from text into IR
    // then add a step to that does the forward scans
    // parsing and inst folding
    while idx < program.len() {
        match program.chars().nth(idx).unwrap() {
            '+' => {
                let m = Regex::new(r"\++").unwrap().find(&program[idx..]).unwrap();
                ir.push((Inst::Add, m.len()));
                idx += m.len() - 1;
            }
            '-' => {
                let m = Regex::new(r"-+").unwrap().find(&program[idx..]).unwrap();
                ir.push((Inst::Sub, m.len()));
                idx += m.len() - 1;
            }
            '>' => {
                let m = Regex::new(r">+").unwrap().find(&program[idx..]).unwrap();
                ir.push((Inst::Right, m.len()));
                idx += m.len() - 1;
            }
            '<' => {
                let m = Regex::new(r"<+").unwrap().find(&program[idx..]).unwrap();
                ir.push((Inst::Left, m.len()));
                idx += m.len() - 1;
            }
            '[' => ir.push((Inst::LoopStart, 1)),
            ']' => ir.push((Inst::LoopEnd, 1)),
            '.' => ir.push((Inst::Out, 1)),
            ',' => ir.push((Inst::In, 1)),
            _ => (),
        }

        idx += 1;
    }

    // cell zeroing opt
    for (idx, window) in ir.to_vec().windows(3).enumerate() {
        if let [(i0, _), (i1, _), (i2, _)] = window {
            if *i0 == Inst::LoopStart
                && (*i1 == Inst::Add || *i1 == Inst::Sub)
                && *i2 == Inst::LoopEnd
            {
                ir[idx] = (Inst::Zero, 1);
                ir[idx + 1] = (Inst::Nop, 1);
                ir[idx + 2] = (Inst::Nop, 1);
            }
        }
    }

    // let inner_loops = get_inner_loops(&ir);
    // for (start, end) in inner_loops {
    //     let simple = is_simple(&ir, start, end);
    //     let loop_ins = &ir[start + 1..end];
    //     println!("{}:", simple);
    //     for (ins, ct) in loop_ins {
    //         println!("\t{:?}: {}", ins, ct);
    //     }
    // }

    let mut module = Module::new();

    // Encode the type section.
    let mut types = TypeSection::new();
    types.function([ValType::I32], []);
    let js_log = types.len() - 1;
    types.function([], []);
    let bf_main = types.len() - 1;
    module.section(&types);

    let mut imports = ImportSection::new();
    imports.import("env", "log", wasm_encoder::EntityType::Function(js_log));
    module.section(&imports);

    // Encode the function section.
    let mut functions = FunctionSection::new();
    functions.function(bf_main);
    module.section(&functions);

    let mut memories = MemorySection::new();
    memories.memory(MemoryType {
        minimum: 1,
        maximum: None,
        memory64: false,
        shared: false,
        page_size_log2: None,
    });
    module.section(&memories);

    // Encode the export section.
    let mut exports = ExportSection::new();
    exports.export("main", ExportKind::Func, bf_main);
    module.section(&exports);

    // Encode the code section.
    let mut codes = CodeSection::new();
    let locals = vec![(1, ValType::I32)];
    let mut f = Function::new(locals);

    for (ins, ct) in ir {
        match ins {
            Inst::Add => add(&mut f, ct),
            Inst::Sub => sub(&mut f, ct),
            Inst::Right => dp_r(&mut f, ct),
            Inst::Left => dp_l(&mut f, ct),
            Inst::LoopStart => loop_start(&mut f),
            Inst::LoopEnd => loop_end(&mut f),
            Inst::Zero => set_0(&mut f),
            Inst::Nop => (),
            Inst::Out => print(&mut f, js_log),
            Inst::In => assert!(false),
        }
    }

    f.instruction(&Instruction::End);
    codes.function(&f);
    module.section(&codes);

    let wasm_bytes = module.finish();
    match wasmparser::validate(&wasm_bytes) {
        Ok(_) => (),
        Err(e) => {
            println!("\n\nERROR:\n{}\n", e);
            assert!(false);
        }
    }

    fs::write("rust_prog.wasm", wasm_bytes)?;
    Ok(())
}
