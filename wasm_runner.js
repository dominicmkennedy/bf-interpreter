const fs = require('fs');

let inputBuffer = Buffer.alloc(1)

function getChar() {
  fs.readSync(0, inputBuffer, 0, 1)
  return inputBuffer[0]
}

const imports = {
  env: {
    log: x => process.stdout.write(String.fromCharCode(x)),
    read: () => getChar(),
  }
};

const wasmBuffer = fs.readFileSync('./rust_prog.wasm')
WebAssembly.instantiate(wasmBuffer, imports).then(
  results => {
    console.time("main")
    results.instance.exports.main();
    console.log("\n")
    console.timeEnd("main")
  }
)
