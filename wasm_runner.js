const fs = require('fs');

let inputBuffer = Buffer.alloc(1)

function getChar() {
  fs.readSync(0, inputBuffer, 0, 1)
  return inputBuffer[0]
}

const imports = {
  env: {
    debug_terminate: (cell_num, val) => console.log(`\nprogram terminated on cell: ${cell_num - 16} with value: ${val}`),
    write: x => process.stdout.write(String.fromCharCode(x)),
    read: () => getChar(),
  }
};

const wasmBuffer = fs.readFileSync('./rust_prog.wasm')
WebAssembly.instantiate(wasmBuffer, imports).then(
  results => {
    console.time("wasm-run-time")
    results.instance.exports.main();
    console.timeEnd("wasm-run-time")
  }
)
