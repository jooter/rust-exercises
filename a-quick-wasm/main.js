export { asm };

const response = await fetch("./target/wasm32-unknown-unknown/release/a_quick_wasm.wasm");
const bytes = await response.arrayBuffer();
const instance = await WebAssembly.instantiate(bytes);
const asm = instance.instance.exports;
