TARGET_DIR := justfile_directory() + "/target"
BIN_DIR := TARGET_DIR + "/bin"

default: run

install-wkg:
    ! test -f {{ BIN_DIR }}/wkg || \
    cargo install wkg \
        --root {{ TARGET_DIR }}

get-component: install-wkg
    ! test -f sample_wasi_http_rust.wasm || \
    {{ BIN_DIR }}/wkg oci pull \
        ghcr.io/bytecodealliance/sample-wasi-http-rust/sample-wasi-http-rust:latest \
        -o sample_wasi_http_rust.wasm

install-hyperlight-wasm-aot:
    ! test -f {{ BIN_DIR }}/hyperlight-wasm-aot || \
    cargo install hyperlight-wasm-aot \
        --git https://github.com/jprendes/hyperlight-wasm.git \
        --rev 134d8fc355ef842ace918777a758349342241c9d \
        --root {{ TARGET_DIR }}

aot-component: get-component install-hyperlight-wasm-aot
    {{ BIN_DIR }}/hyperlight-wasm-aot compile --component sample_wasi_http_rust.wasm sample_wasi_http_rust.bin

install-wasm-tools:
    ! test -f {{ BIN_DIR }}/wasm-tools || \
    cargo install wasm-tools \
        --root {{ TARGET_DIR }}

make-wit-world: install-wasm-tools
    ! test -f hyperlight-world.wasm || \
    {{ BIN_DIR }}/wasm-tools component wit hyperlight.wit -w -o hyperlight-world.wasm

build: make-wit-world
    cargo build

run: build aot-component
    cargo run -- sample_wasi_http_rust.bin
