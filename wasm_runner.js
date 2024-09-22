const fs = require('fs');

const imports = {
  env: { log: x => process.stdout.write(String.fromCharCode(x)) }
  // env: { log: x => console.log(x) }
};

const wasmBuffer = fs.readFileSync('./rust_prog.wasm')
WebAssembly.instantiate(wasmBuffer, imports).then(
  results => {
    console.time("main")
    results.instance.exports.main();
    console.timeEnd("main")
  }
)
