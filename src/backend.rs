use wasm_encoder::{
    BlockType, CodeSection, ExportKind, ExportSection, Function, FunctionSection, ImportSection,
    Instruction, MemArg, MemorySection, MemoryType, Module, TypeSection, ValType,
};

use crate::ir::{IR, Inst};

pub fn create_wasm(ir: &IR) -> Vec<u8> {
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
            Inst::Add => add(&mut f, *ct),
            Inst::Sub => sub(&mut f, *ct),
            Inst::Right => dp_r(&mut f, *ct),
            Inst::Left => dp_l(&mut f, *ct),
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

    wasm_bytes
}

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
