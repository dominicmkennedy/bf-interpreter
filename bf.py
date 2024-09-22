from sys import argv
from typing import Any
import numpy as np

prof = False
if argv[1] == "-p":
    prof = True

f_name = argv[2] if prof else argv[1]
raw_prog = open(f_name, "r").read()
program: list[tuple[str, int]] = [(x, 0) for x in raw_prog if x in "+-><[].,"]


def make_pairs(program: list[tuple[str, int]]) -> tuple[dict[int, int], dict[int, int]]:
    queue: None | int = None
    stack: list[int] = []
    pairs: dict[int, int] = {}
    innermost: dict[int, int] = {}

    for idx, (ins, _) in enumerate(program):
        if ins == "[":
            queue = idx
            stack.append(idx)
        if ins == "]":
            if queue:
                innermost[queue] = idx
                queue = None
            pairs[idx] = stack.pop()
            pairs[pairs[idx]] = idx

    return pairs, innermost


tape = np.zeros(10_000_000, dtype=np.uint8)
pairs, innermost = make_pairs(program)
(pc, dp) = (0, 0)


def loop_ans(x: tuple[int, int]) -> tuple[bool, str]:
    loop_ins = "".join([x[0] for x in program[x[0] + 1 : x[1]]])

    # no I/O
    if "." in loop_ins or "," in loop_ins:
        return False, loop_ins

    ptr_change = 0
    loop_ptr_has_changed = False
    for ins in loop_ins:
        if ins == ">":
            ptr_change += 1
        if ins == "<":
            ptr_change -= 1
        if (ins == "+" or ins == "-") and ptr_change == 0:
            # only one way to zero out the loop
            if loop_ptr_has_changed:
                return False, loop_ins
            else:
                loop_ptr_has_changed = True

    # no net ptr movements
    if ptr_change != 0:
        return False, loop_ins

    return True, loop_ins


def inc_prof(x: tuple[Any, int]) -> tuple[Any, int]:
    return x[0], x[1] + 1


loops = {x[0]: (loop_ans(x), 0) for x in innermost.items()}

while pc < len(program):
    match program[pc][0]:
        case ">":
            dp += 1
        case "<":
            dp -= 1
        case "+":
            tape[dp] += 1
        case "-":
            tape[dp] -= 1
        case ".":
            print(chr(tape[dp]), end="")
        case "[":
            pc = pc if tape[dp] else pairs[pc]
            if pc in loops:
                loops[pc] = inc_prof(loops[pc])
        case "]":
            pc = pairs[pc] if tape[dp] else pc

    program[pc] = inc_prof(program[pc])
    pc += 1

print("Normal Termination")

if prof:
    for idx, (ins, ct) in enumerate(program):
        print(f"{idx:04}: {ins}: {ct}")

    print("Simple Loops:")
    print("Strt I|N Iter|Loop Ins")
    print("      |      |        ")
    simple_loops = sorted(
        [(x[0], x[1][1], x[1][0][1]) for x in loops.items() if x[1][0][0]],
        key=lambda x: x[1],
        reverse=True,
    )
    for x in simple_loops:
        print(f"{x[0]:05} |{x[1]:05} |{x[2]}")
    non_simple_loops = sorted(
        [(x[0], x[1][1], x[1][0][1]) for x in loops.items() if not x[1][0][0]],
        key=lambda x: x[1],
        reverse=True,
    )
    print("Non-simple loops")
    print("Strt I|N Iter|Loop Ins")
    print("      |      |        ")
    for x in non_simple_loops:
        print(f"{x[0]:05} |{x[1]:05} |{x[2]}")
