use cpython::{PyResult, Python};
use std::cell::Cell;

use wasmtime_jit::CompiledModule;

py_class!(pub class Module |py| {
    data module: Cell<CompiledModule>;
});

impl Module {
    pub fn new(py: Python, compiled: Cell<CompiledModule>) -> PyResult<Self> {
        Module::create_instance(py, compiled)
    }
}
