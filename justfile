
install-wkg:
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f {{ justfile_directory() }}/target/bin/wkg ]]; then
        cargo install wkg \
            --root {{ justfile_directory() }}/target
    fi

get-component: install-wkg
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f sample_wasi_http_rust.wasm ]]; then
        {{ justfile_directory() }}/target/bin/wkg oci pull \
            ghcr.io/bytecodealliance/sample-wasi-http-rust/sample-wasi-http-rust:latest \
            -o sample_wasi_http_rust.wasm
    fi

install-hyperlight-wasm-aot:
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f {{ justfile_directory() }}/target/bin/hyperlight-wasm-aot ]]; then
        cargo install hyperlight-wasm-aot \
            --git https://github.com/jprendes/hyperlight-wasm.git \
            --rev 134d8fc355ef842ace918777a758349342241c9d \
            --root {{ justfile_directory() }}/target
    fi

aot-component: get-component install-hyperlight-wasm-aot
    #!/bin/bash
    set -euox pipefail
    {{ justfile_directory() }}/target/bin/hyperlight-wasm-aot compile --component sample_wasi_http_rust.wasm sample_wasi_http_rust.bin

install-wasm-tools:
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f {{ justfile_directory() }}/target/bin/wasm-tools ]]; then
        cargo install wasm-tools \
            --root {{ justfile_directory() }}/target
    fi

make-wit-world: install-wasm-tools
    #!/bin/bash
    set -euox pipefail
    if ! [[ -f hyperlight-world.wasm ]]; then
        {{ justfile_directory() }}/target/bin/wasm-tools component wit hyperlight.wit -w -o hyperlight-world.wasm
    fi

build: make-wit-world
    #!/bin/bash
    set -euox pipefail
    export WIT_WORLD={{ justfile_directory() }}/hyperlight-world.wasm
    cargo build

run: build aot-component
    #!/bin/bash
    set -euox pipefail
    export WIT_WORLD={{ justfile_directory() }}/hyperlight-world.wasm
    cargo run -- sample_wasi_http_rust.bin
