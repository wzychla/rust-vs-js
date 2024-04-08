# A tiny Rust2WASM vs Javascript performance contest

## Javascript

Javascript code is in the <a href='mandel_js.html'>mandel_js.html</a> file. 

## Rust2WASM

Rust source is in <a href='src/lib.rs'>src/lib.rs</a> file. Rust HTML host is in the <a href='mandel_rust.html'>mandel_rust.html</a>.

The Rust2WASM docs are <a href='https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm'>here</a>.

### Compilation

```
wasm-pack build --target web --release
```

