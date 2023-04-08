export { asm };

// const response = await fetch("./target/wasm32-unknown-unknown/release/rs_wasm_02.wasm");
const response = await fetch("./main.wasm");
const bytes = await response.arrayBuffer();
const instance = await WebAssembly.instantiate(bytes);
const asm = instance.instance.exports;
