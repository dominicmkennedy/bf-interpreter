#[derive(PartialEq, Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Inst {
    // SimpleLoop(Vec<Inst>, usize),
    Add(usize), // TODO maybe add offset, or range
    Sub(usize), // TODO maybe add offset, or range
    AddFrom(usize, i32),
    SubFrom(usize, i32),
    Left(usize),
    Right(usize),
    In,
    Out,
    LoopStart,
    LoopEnd,
    SimpleLoopStart(i32),
    SimpleLoopEnd,
    Zero(i32),
    Nop, // TODO try and remove
}

pub type IR = Vec<Inst>;

pub fn parse(program: &String) -> IR {
    let mut ir: IR = vec![];

    for ins in program.chars() {
        match ins {
            '+' => ir.push(Inst::Add(1)),
            '-' => ir.push(Inst::Sub(1)),
            '>' => ir.push(Inst::Right(1)),
            '<' => ir.push(Inst::Left(1)),
            '[' => ir.push(Inst::LoopStart),
            ']' => ir.push(Inst::LoopEnd),
            '.' => ir.push(Inst::Out),
            ',' => ir.push(Inst::In),
            _ => (),
        }
    }

    ir
}

fn forward_scan(ir: &[Inst], ins: Inst) -> usize {
    let mut ct = 0;
    for i in ir {
        if *i == ins {
            ct += 1;
        } else {
            break;
        }
    }

    ct
}

pub fn inst_combine(ir: &IR) -> IR {
    let mut new_ir: IR = vec![];
    let mut idx = 0;
    while idx < ir.len() {
        let ins = ir[idx];
        match ins {
            Inst::Add(_) => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push(Inst::Add(ct));
                idx += ct;
            }
            Inst::Sub(_) => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push(Inst::Sub(ct));
                idx += ct;
            }
            Inst::Right(_) => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push(Inst::Right(ct));
                idx += ct;
            }
            Inst::Left(_) => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push(Inst::Left(ct));
                idx += ct;
            }
            _ => {
                new_ir.push(ins);
                idx += 1;
            }
        }
    }

    new_ir
}

fn single_loop_opt(ir: &IR) -> IR {
    let mut dp: i32 = 0;
    let mut new_ir: IR = vec![Inst::SimpleLoopStart(0)];
    for i in &ir[1..] {
        match i {
            Inst::Right(ct) => dp += *ct as i32,
            Inst::Left(ct) => dp -= *ct as i32,
            Inst::Add(ct) => new_ir.push(Inst::AddFrom(*ct, dp)),
            Inst::Sub(ct) => new_ir.push(Inst::SubFrom(*ct, dp)),
            Inst::Zero(_) => new_ir.push(Inst::Zero(dp)),
            Inst::Nop => (),
            _ => {
                println!("{:?}", i);
                assert!(false)
            }
        }
    }

    new_ir.push(Inst::Zero(0));
    new_ir.push(Inst::SimpleLoopEnd);
    new_ir
}

pub fn opt_simple_loops(ir: &IR) -> IR {
    let mut new_ir = ir.clone();
    let mut offset: i32 = 0;
    let inner_loops = get_inner_loops(&new_ir);
    for (start, end) in inner_loops {
        let start_off: i32 = start as i32 + offset;
        let end_off: i32 = end as i32 + offset;
        let simple = is_simple(&new_ir, start_off as usize, end_off as usize);
        let loop_ins = &new_ir[start_off as usize + 1..end_off as usize];
        if simple {
            let new_loop_ins = single_loop_opt(&loop_ins.to_vec());
            offset += new_loop_ins.len() as i32 - (end_off - start_off) - 1;
            new_ir = [
                &new_ir[0..start_off as usize],
                &new_loop_ins,
                &new_ir[end_off as usize + 1..],
            ]
            .concat();
        }
    }

    new_ir
}

// TODO this opt would work better if it was preceded by a cannonicolizer
// TODO this code is also kind of a disaster
// >>+>>+<<<< should be >>+<<>>>>+<<<<
//pub fn offset_arth(ir: &mut IR) {
//    for (idx, window) in ir.to_vec().windows(3).enumerate() {
//        if let [i0, i1, i2] = window {
//            match i1 {
//                Inst::Add(ct1, 0) | Inst::Sub(ct1, 0) => match i0 {
//                    Inst::Right(ct0) => match i2 {
//                        Inst::Left(ct2) => {
//                            if ct0 == ct2 {
//                                match i1 {
//                                    Inst::Add(_, _) => ir[idx] = Inst::Add(*ct1, *ct0 as i32),
//                                    Inst::Sub(_, _) => ir[idx] = Inst::Sub(*ct1, *ct0 as i32),
//                                    _ => (),
//                                }
//                                ir[idx + 1] = Inst::Nop;
//                                ir[idx + 2] = Inst::Nop;
//                            }
//                        }
//                        _ => (),
//                    },
//                    Inst::Left(ct0) => match i2 {
//                        Inst::Right(ct2) => {
//                            if ct0 == ct2 {
//                                match i1 {
//                                    Inst::Add(_, _) => ir[idx] = Inst::Add(*ct1, -(*ct0 as i32)),
//                                    Inst::Sub(_, _) => ir[idx] = Inst::Sub(*ct1, -(*ct0 as i32)),
//                                    _ => (),
//                                }
//                                ir[idx + 1] = Inst::Nop;
//                                ir[idx + 2] = Inst::Nop;
//                            }
//                        }
//                        _ => (),
//                    },
//                    _ => (),
//                },
//                _ => (),
//            }
//        }
//    }
//}

pub fn cell_zero(ir: &mut IR) {
    for (idx, window) in ir.to_vec().windows(3).enumerate() {
        if let [i0, i1, i2] = window {
            if *i0 == Inst::LoopStart && *i2 == Inst::LoopEnd {
                match i1 {
                    Inst::Add(_) | Inst::Sub(_) => {
                        ir[idx] = Inst::Zero(0);
                        ir[idx + 1] = Inst::Nop;
                        ir[idx + 2] = Inst::Nop;
                    }
                    _ => (),
                }
            }
        }
    }
}

pub fn get_inner_loops(ir: &IR) -> Vec<(usize, usize)> {
    let mut inner_loops: Vec<(usize, usize)> = Vec::new();
    let mut top_paren: Option<usize> = None;
    for (idx, ins) in ir.iter().enumerate() {
        match ins {
            Inst::LoopStart => top_paren = Some(idx),
            Inst::LoopEnd => match top_paren {
                None => (),
                Some(x) => {
                    inner_loops.push((x, idx));
                    top_paren = None;
                }
            },
            _ => (),
        }
    }

    inner_loops
}

pub fn is_simple(ir: &IR, start: usize, end: usize) -> bool {
    let loop_ins = &ir[start + 1..end];
    let mut ret = true;

    if loop_ins.contains(&Inst::In) || loop_ins.contains(&Inst::Out) {
        ret = false;
    }

    // TODO making sure the loops are extremely simple just for now
    // this should be removed later
    match loop_ins[0] {
        Inst::Sub(1) => (),
        _ => ret = false,
    }

    let mut ptr_change: i32 = 0;
    let mut loop_ptr_changed = false;
    for ins in loop_ins {
        match ins {
            Inst::Right(ct) => ptr_change += *ct as i32,
            Inst::Left(ct) => ptr_change -= *ct as i32,
            _ => (),
        }
        // TODO this may break if the ir gets more complicated
        match ins {
            Inst::Add(_) | Inst::Sub(_) => {
                if ptr_change == 0 {
                    match loop_ptr_changed {
                        true => ret = false,
                        false => loop_ptr_changed = true,
                    }
                }
            }
            _ => (),
        }
    }

    if ptr_change != 0 {
        ret = false;
    }

    ret
}
