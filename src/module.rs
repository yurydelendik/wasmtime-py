use pyo3::prelude::*;

use wasmtime_jit::CompiledModule;

#[pyclass]
pub struct Module {
    pub module: CompiledModule,
}
