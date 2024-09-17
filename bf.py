from sys import argv

tape = [0] * 10000
program = open(argv[1], "r").read()
pairs = {}
stack = []
pc = 0
dp = 0
for indx, inst in enumerate(program):
    if inst == "[":
        stack.append(indx)
    if inst == "]":
        pairs[indx] = stack.pop()
        pairs[pairs[indx]] = indx
while pc < len(program):
    match program[pc]:
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
        case "]":
            pc = pairs[pc] if tape[dp] else pc
    pc += 1
