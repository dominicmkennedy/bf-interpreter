const fs = require('fs');

const imports = {
  'env': { log: x => process.stdout.write(String.fromCharCode(x)) }
};

const wasmBuffer = fs.readFileSync('./prog.wasm')
WebAssembly.instantiate(wasmBuffer, imports).then(
  results => {
    results.instance.exports.main();
  }
)
