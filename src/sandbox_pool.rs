use anyhow::Context;

use crate::CliOptions;
use crate::WasiImpl;
use crate::wasi_impl;
use crate::wasi_impl::bindings::RootSandbox;
use hyperlight_wasm::LoadedWasmSandbox;

/// The initial number of sandboxes to create in the pool.
#[cfg(feature = "gdb")]
const INITIAL_POOL_SIZE: usize = 1; // Only one sandbox to avoid port conflicts with gdb.
#[cfg(not(feature = "gdb"))]
const INITIAL_POOL_SIZE: usize = 10;

/// Sandbox pool state shared across all requests.
#[derive(Default)]
pub struct SandboxPool {
    options: Option<CliOptions>,
    pool: Vec<RootSandbox<WasiImpl, LoadedWasmSandbox>>,
}

impl SandboxPool {
    pub fn from_options(&mut self, options: CliOptions) -> anyhow::Result<()> {
        let mut pool = Vec::with_capacity(INITIAL_POOL_SIZE);
        for _ in 0..INITIAL_POOL_SIZE {
            let sandbox = build_sandbox(&options)?;
            pool.push(sandbox);
        }
        println!("Initialized sandbox pool with {} sandboxes", pool.len());
        self.options = Some(options);
        self.pool = pool;
        Ok(())
    }

    pub fn get_sandbox(&mut self) -> anyhow::Result<RootSandbox<WasiImpl, LoadedWasmSandbox>> {
        if let Some(sandbox) = self.pool.pop() {
            Ok(sandbox)
        } else {
            println!("Sandbox pool is empty, building a new sandbox");
            let options = self
                .options
                .as_ref()
                .context("SandboxPool not initialized with options")?;
            build_sandbox(options)
        }
    }

    pub fn return_sandbox(&mut self, sandbox: RootSandbox<WasiImpl, LoadedWasmSandbox>) {
        self.pool.push(sandbox);
    }
}

fn builder(wasm_size: u64) -> hyperlight_wasm::SandboxBuilder {
    #[cfg(not(feature = "gdb"))]
    let builder = hyperlight_wasm::SandboxBuilder::new()
        .with_guest_heap_size((1.2 * wasm_size as f64) as u64);

    // When debugging with gdb, we need to a big chunk of memory to avoid out-of-memory
    #[cfg(feature = "gdb")]
    let builder = hyperlight_wasm::SandboxBuilder::new()
        .with_debugging_enabled(8080)
        .with_guest_heap_size(6 * wasm_size);

    builder
        .with_guest_stack_size(10 * 1024)
        .with_guest_input_buffer_size(10 * 1024)
        .with_guest_output_buffer_size(10 * 1024)
        .with_function_definition_size(20 * 1024)
}

/// Parse `--env KEY=VALUE` entries into (key, value) pairs.
fn parse_envs(envs: &[String]) -> Vec<(String, String)> {
    envs.iter()
        .filter_map(|e| {
            let (k, v) = e.split_once('=')?;
            Some((k.to_string(), v.to_string()))
        })
        .collect()
}

fn build_sandbox(options: &CliOptions) -> anyhow::Result<RootSandbox<WasiImpl, LoadedWasmSandbox>> {
    let wasm_size = std::fs::metadata(&options.wasm_file)
        .with_context(|| format!("cannot stat wasm file '{}'", options.wasm_file))?
        .len();

    let builder = builder(wasm_size);
    let mut sb = builder.build().context("failed to build sandbox")?;

    let mut state = WasiImpl::new();
    state.env_vars = parse_envs(&options.envs);

    let rt = wasi_impl::bindings::register_host_functions(&mut sb, state);

    let sb = sb.load_runtime().context("failed to load runtime")?;
    let sb = sb
        .load_module(&options.wasm_file)
        .with_context(|| format!("failed to load module '{}'", options.wasm_file))?;

    Ok(RootSandbox { sb, rt })
}
