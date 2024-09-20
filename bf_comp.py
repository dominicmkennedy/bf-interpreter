from sys import argv

f_name = argv[1]
raw_prog = open(f_name, "r").read()
program: list[str] = [x for x in raw_prog if x in "+-><[].,"]

prog_header = """
(module
  (import "env" "log" (func $log (param i32)))
  (memory $mem 100)
  (func (export "main")
    (local $dp i32)
    (local.set $dp (i32.const 0))
"""

prog_footer = """
  )
)
"""


with open("prog.wat", "w") as f:
    f.write(prog_header)

    for idx, ins in enumerate(program):
        wasm_ins = ""
        match ins:
            case ">":
                wasm_ins = """
                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))
                """
            case "<":
                wasm_ins = """
                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))
                """
            case "+":
                wasm_ins = """
                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))
                """
            case "-":
                wasm_ins = """
                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))
                """
            case ".":
                wasm_ins = """
                (call $log (i32.load8_u (local.get $dp)))
                """
            case "[":
                wasm_ins = """
                (block $break
                  (loop $loop
                    (br_if
                      $break
                      (i32.eqz
                        (i32.load8_u (local.get $dp))))
                """
            case "]":
                wasm_ins = """
                br $loop))
                """

        f.write(wasm_ins)

    f.write(prog_footer)
