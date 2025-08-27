# `hyperlight-wasm` http example

> [!WARNING]  
> This proof of concept is a work in progress.
> The current state of this example fails at runtime.

This is a minimal example of a
[Hyperlight-Wasm](https://github.com/hyperlight-dev/hyperlight-wasm)
host application. It implements just enough of the `wasi:http` api
to run the [echo sample_wasi_http_rust
server](https://github.com/bytecodealliance/sample-wasi-http-rust).

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install), including the `x86_64-unknown-none` target (which may be installed via e.g. `rustup target add x86_64-unknown-none`)
2. `clang`
3. [`just`](https://github.com/casey/just) (optional, but recommended)

If you want to follow the manual build instructions, you will also need:
4. [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)
5. If you are fetching the sample binary from an OCI registry,
   [`wkg`](https://crates.io/crates/wkg/0.10.0).
6. [`hyperlight-wasm-aot`](https://github.com/hyperlight-dev/hyperlight-wasm) from [this commit](https://github.com/jprendes/hyperlight-wasm/tree/134d8fc35)

## Simply setup

### Building

```sh
just build
```

### Running

```sh
just run
```

From another terminal, you can then test the server:

```sh
curl http://localhost:3000/
curl -w'\n' -d "hola mundo" http://127.0.0.1:3000/echo
curl -I -H "x-language: spanish" http://127.0.0.1:3000/echo-headers
```

## Manual setup

### Building

Compile the WIT and set the environment variables used when building
(both the host and the guest):

```sh
wasm-tools component wit hyperlight.wit -w -o hyperlight-world.wasm
```

Build:
```
cargo build
```

### Running

Get an `sample_wasi_http_rust.wasm` from [the sample
repo](https://github.com/bytecodealliance/sample-wasi-http-rust), either
by building it or by fetching it from the OCI registry
(`wkg oci pull ghcr.io/bytecodealliance/sample-wasi-http-rust/sample-wasi-http-rust:latest -o sample_wasi_http_rust.wasm`).

AOT compile it:

```sh
cargo install hyperlight-wasm-aot --git https://github.com/jprendes/hyperlight-wasm.git --rev 134d8fc35
hyperlight-wasm-aot compile --component sample_wasi_http_rust.wasm sample_wasi_http_rust.bin
```

You can then run the server:

```sh
cargo run -- sample_wasi_http_rust.bin
```
