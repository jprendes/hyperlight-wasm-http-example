
install-wkg:
    #!/bin/bash
    set -euox pipefail
    if ! command -v wkg > /dev/null; then
        cargo install wkg
    fi

get-component: install-wkg
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f sample_wasi_http_rust.wasm ]]; then
        wkg oci pull \
            ghcr.io/bytecodealliance/sample-wasi-http-rust/sample-wasi-http-rust:latest \
            -o sample_wasi_http_rust.wasm
    fi

install-hyperlight-wasm-aot:
    #!/bin/bash
    set -euox pipefail
    if ! command -v hyperlight-wasm-aot > /dev/null; then
        cargo install hyperlight-wasm-aot
    fi

aot-component: get-component install-hyperlight-wasm-aot
    #!/bin/bash
    set -euox pipefail
    hyperlight-wasm-aot compile --component sample_wasi_http_rust.wasm sample_wasi_http_rust.bin

install-wasm-tools:
    #!/bin/bash
    set -euox pipefail
    if ! command -v wasm-tools > /dev/null; then
        cargo install wasm-tools
    fi

make-wit-world: install-wasm-tools
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f hyperlight-world.wasm ]]; then
        wasm-tools component wit hyperlight.wit -w -o hyperlight-world.wasm
    fi

generate-bindings: make-wit-world
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f {{ justfile_directory() }}/src/bindings.rs ]]; then
        export WIT_WORLD={{ justfile_directory() }}/hyperlight-world.wasm
        export HYPERLIGHT_COMPONENT_MACRO_DEBUG={{ justfile_directory() }}/src/bindings.rs
        cargo build --features=use-macro || true
        git apply --no-index {{ justfile_directory() }}/bindings.patch
    fi

build: generate-bindings
    #!/bin/bash
    set -euox pipefail
    export WIT_WORLD={{ justfile_directory() }}/hyperlight-world.wasm
    cargo build

run: build aot-component
    #!/bin/bash
    set -euox pipefail
    export WIT_WORLD={{ justfile_directory() }}/hyperlight-world.wasm
    cargo run -- sample_wasi_http_rust.bin
