# A tiny Rust2WASM vs Javascript performance contest

Is Rust faster than Javascript?  What a question, it is. 

Actually however, it's not that one language is just *faster* than the other. 
It depends on what you try to do and how you do it. 

Javascript just runs in a browser on a highly optimized engine. Rust can be compiled to WASM that
also runs in a browser but on its own engine. 

The two can be compared. It's not then about Rust vs Javascript but rather about  
the Javascript and WASM engines. Assuming you do same/similar things in both engines, you can 
compare them in terms of performance/memory usage.

Note also that both contest implementations are subjects of further improvement. 

## Running tests

Both playgrounds can be run by navigating to their respective `*.html` files.

## Javascript

Javascript code is in the <a href='mandel_js.html'>mandel_js.html</a> file. 

### Compilation

No need to compile anything, Javascript just works.

## Rust2WASM

Rust source is in <a href='src/lib.rs'>src/lib.rs</a> file. Rust HTML host is in the <a href='mandel_rust.html'>mandel_rust.html</a>.
The Rust source has to be compiled in advance, the compilation produces the `*.wasm` binary and also a 
`*.js` bridge to it.

The Rust2WASM docs are <a href='https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm'>here</a>.

### Compilation

<ol>

<li><a href='https://www.rust-lang.org/tools/install'>Install Rust</a>
<li>Install <a href='https://github.com/rustwasm/wasm-pack'>wasm-pack</a>

```
cargo install wasm-pack
```

<li>use wasm-pack to compile and build the wasm/js

```
wasm-pack build --target web --release
```
</ol>

## gpu.js

Yet another version, just to show that moving the computation to GPU (using <a href="https://gpu.rocks/#/">gpu.js</a>) boosts the performance considerably. 
Check the source here, <a href='mandel_js_gpu.html'>mandel_js_gpu.html</a>.

Thanks to <a href="https://github.com/zsacul">Łukasz Piwowar</a> for pointing out this possibility and porting the code.
