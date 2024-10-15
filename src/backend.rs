use wasm_encoder::{
    BlockType, CodeSection, ExportKind, ExportSection, Function, FunctionSection, ImportSection,
    Instruction, MemArg, MemorySection, MemoryType, Module, TypeSection, ValType,
};

use crate::ir::{Inst, IR};

const DP: u32 = 0;

pub fn create_wasm(ir: &IR) -> Vec<u8> {
    let mut module = Module::new();

    // Encode the type section.
    let mut types = TypeSection::new();

    types.function([ValType::I32], []);
    let js_write = types.len() - 1;

    types.function([], [ValType::I32]);
    let js_read = types.len() - 1;

    types.function([ValType::I32, ValType::I32], []);
    let js_debug_terminate = types.len() - 1;

    types.function([], []);
    let bf_main = types.len() - 1;

    module.section(&types);

    let mut imports = ImportSection::new();
    imports.import("env", "write", wasm_encoder::EntityType::Function(js_write));
    imports.import("env", "read", wasm_encoder::EntityType::Function(js_read));
    imports.import(
        "env",
        "debug_terminate",
        wasm_encoder::EntityType::Function(js_debug_terminate),
    );
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
    let locals = vec![(1, ValType::I32), (1, ValType::I32)];
    let mut f = Function::new(locals);

    f.instruction(&Instruction::I32Const(16));
    f.instruction(&Instruction::LocalSet(DP));

    for ins in ir {
        match ins {
            Inst::Add(ct) => add(&mut f, *ct),
            Inst::Sub(ct) => sub(&mut f, *ct),
            Inst::AddFrom(ct, off) => add_from(&mut f, *ct, *off),
            Inst::SubFrom(ct, off) => sub_from(&mut f, *ct, *off),
            Inst::Right(ct) => dp_r(&mut f, *ct),
            Inst::Left(ct) => dp_l(&mut f, *ct),
            Inst::LoopStart => loop_start(&mut f),
            Inst::LoopEnd => loop_end(&mut f),
            Inst::Zero(off) => set_0(&mut f, *off),
            Inst::Out => print(&mut f, js_write),
            Inst::In => read(&mut f, js_read),
            Inst::SimpleLoopStart(off) => simple_loop_start(&mut f, *off),
            Inst::SimpleLoopEnd => simple_loop_end(&mut f),
            Inst::Scan(stride) => scan(&mut f, *stride),
        }
    }

    add_debug_termination(&mut f, js_debug_terminate);

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

// TODO https://rsms.me/wasm-intro#addressing-memory
fn null_mem_arg() -> MemArg {
    MemArg {
        offset: 0,
        align: 0,
        memory_index: 0,
    }
}

fn add_debug_termination(f: &mut Function, js_debug_terminate: u32) {
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::Call(js_debug_terminate));
}

fn read(f: &mut Function, js_read: u32) {
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::Call(js_read));
    f.instruction(&Instruction::I32Store8(null_mem_arg()));
}

fn print(f: &mut Function, js_write: u32) {
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::Call(js_write));
}

fn add_or_sub(f: &mut Function, ct: usize, i: &Instruction) {
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::I32Const(ct as i32));
    f.instruction(i);
    f.instruction(&Instruction::I32Store8(null_mem_arg()));
}

fn add(f: &mut Function, ct: usize) {
    add_or_sub(f, ct, &Instruction::I32Add);
}

fn sub(f: &mut Function, ct: usize) {
    add_or_sub(f, ct, &Instruction::I32Sub);
}

fn add_or_sub_from(f: &mut Function, ct: usize, off: i32, i: &Instruction) {
    // get offset number address
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Const(off));
    f.instruction(&Instruction::I32Add);
    // get offset number
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Const(off));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    // get loop ct val
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    // mul loop number by count
    if ct != 1 {
        f.instruction(&Instruction::I32Const(ct as i32));
        f.instruction(&Instruction::I32Mul);
    }
    // add/sub offset number and mul'd loop ct
    f.instruction(i);
    // store new num at offset addr
    f.instruction(&Instruction::I32Store8(null_mem_arg()));
}

fn add_from(f: &mut Function, ct: usize, off: i32) {
    add_or_sub_from(f, ct, off, &Instruction::I32Add)
}

fn sub_from(f: &mut Function, ct: usize, off: i32) {
    add_or_sub_from(f, ct, off, &Instruction::I32Sub)
}

fn scan(f: &mut Function, stride: i32) {
    if stride > 0 {
        for_scan(f, stride);
    } else {
        rev_scan(f, stride);
    }
}

fn rev_scan(f: &mut Function, stride: i32) {
    if stride != -1 && stride != -2 {
        assert!(false);
    }

    simple_loop_start(f, 0);

    // Set the dp back by one vector
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Const(-16));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(DP));

    f.instruction(&Instruction::Block(BlockType::Empty));
    f.instruction(&Instruction::Loop(BlockType::Empty));

    f.instruction(&Instruction::V128Const(0));
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::V128Load(null_mem_arg()));
    f.instruction(&Instruction::I8x16Eq);

    if stride == -2 {
        f.instruction(&Instruction::V128Const(0x00FF00FF00FF00FF00FF00FF00FF00FF));
        f.instruction(&Instruction::V128And);
    }

    f.instruction(&Instruction::I8x16Bitmask);
    f.instruction(&Instruction::I32Clz);
    f.instruction(&Instruction::I32Const(-16));
    f.instruction(&Instruction::I32Add);

    // if there is a value other than 16 then break
    f.instruction(&Instruction::LocalTee(1));
    f.instruction(&Instruction::I32Const(16));
    f.instruction(&Instruction::I32Ne);
    f.instruction(&Instruction::BrIf(1));

    // Sub 16 to data pointer
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Const(-16));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(DP));

    f.instruction(&Instruction::Br(0));
    f.instruction(&Instruction::End);
    f.instruction(&Instruction::End);

    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::LocalGet(1));
    f.instruction(&Instruction::I32Sub);
    f.instruction(&Instruction::I32Const(15));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(DP));

    simple_loop_end(f);
}

fn for_scan(f: &mut Function, stride: i32) {
    if stride != 1 && stride != 2 && stride != 4 {
        assert!(false);
    }

    simple_loop_start(f, 0);

    // TODO load the masks and the zero etc, outside the loop
    f.instruction(&Instruction::Block(BlockType::Empty));
    f.instruction(&Instruction::Loop(BlockType::Empty));

    f.instruction(&Instruction::V128Const(0));
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::V128Load(null_mem_arg()));
    f.instruction(&Instruction::I8x16Eq);

    if stride == 2 {
        f.instruction(&Instruction::V128Const(0x00FF00FF00FF00FF00FF00FF00FF00FF));
        f.instruction(&Instruction::V128And);
    }
    if stride == 4 {
        f.instruction(&Instruction::V128Const(0x000000FF000000FF000000FF000000FF));
        f.instruction(&Instruction::V128And);
    }

    f.instruction(&Instruction::I8x16Bitmask);
    f.instruction(&Instruction::I32Ctz);

    // if there is a value other than 32 then break
    f.instruction(&Instruction::LocalTee(1));
    f.instruction(&Instruction::I32Const(32));
    f.instruction(&Instruction::I32Ne);
    f.instruction(&Instruction::BrIf(1));

    // Add 16 to data pointer
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Const(16));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(DP));

    f.instruction(&Instruction::Br(0));
    f.instruction(&Instruction::End);
    f.instruction(&Instruction::End);

    f.instruction(&Instruction::LocalGet(1));
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(DP));

    simple_loop_end(f);
}

fn set_0(f: &mut Function, off: i32) {
    f.instruction(&Instruction::LocalGet(DP));
    if off != 0 {
        f.instruction(&Instruction::I32Const(off));
        f.instruction(&Instruction::I32Add);
    }
    f.instruction(&Instruction::I32Const(0));
    f.instruction(&Instruction::I32Store8(null_mem_arg()));
}

fn dp_r(f: &mut Function, ct: usize) {
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Const(ct as i32));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::LocalSet(DP));
}

fn dp_l(f: &mut Function, ct: usize) {
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Const(ct as i32));
    f.instruction(&Instruction::I32Sub);
    f.instruction(&Instruction::LocalSet(DP));
}

fn loop_start(f: &mut Function) {
    f.instruction(&Instruction::Block(BlockType::Empty));
    f.instruction(&Instruction::Loop(BlockType::Empty));
    f.instruction(&Instruction::LocalGet(DP));
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::I32Eqz);
    f.instruction(&Instruction::BrIf(1));
}

fn loop_end(f: &mut Function) {
    f.instruction(&Instruction::Br(0));
    f.instruction(&Instruction::End);
    f.instruction(&Instruction::End);
}

fn simple_loop_start(f: &mut Function, off: i32) {
    f.instruction(&Instruction::Block(BlockType::Empty));
    f.instruction(&Instruction::LocalGet(DP));
    if off != 0 {
        f.instruction(&Instruction::I32Const(off));
        f.instruction(&Instruction::I32Add);
    }
    f.instruction(&Instruction::I32Load8U(null_mem_arg()));
    f.instruction(&Instruction::I32Eqz);
    f.instruction(&Instruction::BrIf(0));
}

fn simple_loop_end(f: &mut Function) {
    f.instruction(&Instruction::End);
}
