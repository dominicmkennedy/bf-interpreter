use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Inst {
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

pub type IR = Vec<(Inst, usize)>;

pub fn parse(program: &String) -> IR {
    let mut ir: IR = vec![];

    for ins in program.chars() {
        match ins {
            '+' => ir.push((Inst::Add, 1)),
            '-' => ir.push((Inst::Sub, 1)),
            '>' => ir.push((Inst::Right, 1)),
            '<' => ir.push((Inst::Left, 1)),
            '[' => ir.push((Inst::LoopStart, 1)),
            ']' => ir.push((Inst::LoopEnd, 1)),
            '.' => ir.push((Inst::Out, 1)),
            ',' => ir.push((Inst::In, 1)),
            _ => (),
        }
    }

    ir
}

fn forward_scan(ir: &[(Inst, usize)], ins: Inst) -> usize {
    let mut ct = 0;
    for (i, _) in ir {
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
        let (ins, _) = ir[idx];
        match ins {
            Inst::Add => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push((ins, ct));
                idx += ct;
            }
            Inst::Sub => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push((ins, ct));
                idx += ct;
            }
            Inst::Right => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push((ins, ct));
                idx += ct;
            }
            Inst::Left => {
                let ct = forward_scan(&ir[idx..], ins);
                new_ir.push((ins, ct));
                idx += ct;
            }
            _ => {
                new_ir.push((ins, 1));
                idx += 1;
            }
        }
    }

    new_ir
}

pub fn cell_zero(ir: &mut IR) {
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
}

pub fn get_inner_loops(ir: &IR) -> HashMap<usize, usize> {
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

pub fn is_simple(ir: &IR, start: usize, end: usize) -> bool {
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
