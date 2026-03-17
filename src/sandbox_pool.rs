use crate::CliOptions;
use crate::WasiImpl;
use crate::wasi_impl;
use crate::wasi_impl::bindings::RootSandbox;
use hyperlight_wasm::LoadedWasmSandbox;

const INITIAL_POOL_SIZE: usize = 10;

/// Sandbox pool state shared across all requests.
#[derive(Default)]
pub struct SandboxPool {
    options: Option<CliOptions>,
    pool: Vec<RootSandbox<WasiImpl, LoadedWasmSandbox>>,
}

impl SandboxPool {
    pub fn from_options(&mut self, options: CliOptions) {
        let mut pool = Vec::with_capacity(INITIAL_POOL_SIZE);
        for _ in 0..INITIAL_POOL_SIZE {
            let sandbox = build_sandbox(&options);
            pool.push(sandbox);
        }
        println!("Initialized sandbox pool with {} sandboxes", pool.len());
        self.options = Some(options);
        self.pool = pool;
    }

    pub fn get_sandbox(&mut self) -> RootSandbox<WasiImpl, LoadedWasmSandbox> {
        if let Some(sandbox) = self.pool.pop() {
            sandbox
        } else {
            println!("Sandbox pool is empty, building a new sandbox");
            build_sandbox(
                &self
                    .options
                    .as_ref()
                    .expect("SandboxPool not initialized with options"),
            )
        }
    }

    pub fn return_sandbox(&mut self, sandbox: RootSandbox<WasiImpl, LoadedWasmSandbox>) {
        self.pool.push(sandbox);
    }
}

fn builder(wasm_size: u64) -> hyperlight_wasm::SandboxBuilder {
    let builder = hyperlight_wasm::SandboxBuilder::new()
        .with_guest_heap_size((1.2 * wasm_size as f64) as u64);

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

fn build_sandbox(options: &CliOptions) -> RootSandbox<WasiImpl, LoadedWasmSandbox> {
    let wasm_size = std::fs::metadata(&options.wasm_file)
        .unwrap_or_else(|e| panic!("cannot stat wasm file '{}': {e}", options.wasm_file))
        .len();

    let builder = builder(wasm_size);
    let mut sb = builder.build().unwrap();

    let mut state = WasiImpl::new();
    state.env_vars = parse_envs(&options.envs);

    let rt = wasi_impl::bindings::register_host_functions(&mut sb, state);

    let sb = sb.load_runtime().unwrap();
    let sb = sb.load_module(&options.wasm_file).unwrap();

    RootSandbox { sb, rt }
}
