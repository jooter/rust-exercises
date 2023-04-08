# A quick simple WASM with JS module
I had some trouble loading WebAssembly (wasm) in a concise way. Now I think I found it. To help others who may encounter the same issue, I have created a simple demo that outlines all the necessary steps.

## Install
Install rust if you haven't. And then
```shell
rustup target add wasm32-unknown-unknown
```

## Create project
```shell
cargo new --lib a-quick-wasm
```
and then go to a-quick-wasm directory or open the directory in your IDE

## Config
In Cargo.toml add a line in lib section
```toml
[lib]
crate-type = ["cdylib"] # for wasm
```
## Export Rust functions
Tag function with
**no_mangle**

Declare with 
**pub extern "C"**
```rust
#[no_mangle]
pub extern "C"
fn add(left: usize, right: usize) -> usize {
    left + right
}

```

## Build
```shell
cargo build --release --target wasm32-unknown-unknown
```
## Or config alias for build

add .cargo/config with lines
```toml
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
```
and then
```shell
cargo wasm
```

## Load WASM by JS module
```js
// main.js
export { asm };

const response = await fetch("./target/wasm32-unknown-unknown/release/a_quick_wasm.wasm");
const bytes = await response.arrayBuffer();
const instance = await WebAssembly.instantiate(bytes);
const asm = instance.instance.exports;
```
Only in JS module header, await is allowed outside of async

## Import WASM module
import wasm module into html or other JS module
```html
  <body>
    <script type="module">
      import { asm } from "./main.js";
      console.log(asm.add(40 + 2));
    </script>
  </body>
```

## Serve statice files
```shell
python -m http.server
# and then open http://localhost:8000 in browser
# and then open browser console
```

## Notes and options
wasm-gc will shrink wasm size greatly
```shell
cargo install wasm-gc
```
These steps have been tested in windows with chrome. They should work in Linux and other browsers.

Thanks for the wonderful community of wasm with rusty.