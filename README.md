# bf-interpreter

## Python

Uses: Python3

To run: `python bf.py program.bf`

## Uiua

To install Uiua: `cargo install uiua`

To run: `uiua bf.ua`

## BF to WASM compiler

First run `python bf_comp.py program.b`. This will create `prog.wat`.

Then run `wat2wasm prog.wat`. This will convert the WAT code to a WASM file.

Finaly run `node wasm_runner.js`. This will execute the `prog.wasm` file.
