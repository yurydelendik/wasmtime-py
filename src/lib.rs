use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use pyo3::wrap_pyfunction;

use crate::instance::{Instance, InstanceContext};
use crate::module::Module;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod function;
mod instance;
mod module;
mod value;

#[pyclass]
pub struct InstantiateResultObject {
    instance: Py<Instance>,
    module: Py<Module>,
}

#[pymethods]
impl InstantiateResultObject {
    #[getter(instance)]
    fn get_instance(&self) -> PyResult<Py<Instance>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.instance.clone_ref(py))
    }

    #[getter(module)]
    fn get_module(&self) -> PyResult<Py<Module>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self.module.clone_ref(py))
    }
}

#[pyfunction]
pub fn instantiate(
    py: Python,
    buffer_source: &PyBytes,
    _import_obj: Py<PyDict>,
) -> PyResult<Py<InstantiateResultObject>> {
    let wasm_data = buffer_source.as_bytes();

    let generate_debug_info = false;

    let isa = {
        let isa_builder =
            cranelift_native::builder().expect("host machine is not a supported target");
        let flag_builder = cranelift_codegen::settings::builder();
        isa_builder.finish(cranelift_codegen::settings::Flags::new(flag_builder))
    };
    let mut compiler = wasmtime_jit::Compiler::new(isa);
    let mut resolver = wasmtime_jit::NullResolver {};
    let global_exports = Rc::new(RefCell::new(HashMap::new()));
    let mut module = wasmtime_jit::CompiledModule::new(
        &mut compiler,
        wasm_data,
        &mut resolver,
        global_exports,
        generate_debug_info,
    )
    .expect("compiled");

    let mut context = wasmtime_jit::Context::new(Box::new(compiler));
    context.set_debug_info(generate_debug_info);
    let _global_exports = context.get_global_exports();

    let instance = module.instantiate().expect("instance");

    let module = Py::new(py, Module { module })?;
    let context = Rc::new(RefCell::new(InstanceContext {
        jit_context: context,
        instance,
    }));
    let instance = Py::new(py, Instance { context })?;

    Py::new(py, InstantiateResultObject { instance, module })
}

#[pymodule]
fn wasmtime_py(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Instance>()?;
    m.add_class::<Module>()?;
    m.add_class::<InstantiateResultObject>()?;
    m.add_wrapped(wrap_pyfunction!(instantiate))?;
    Ok(())
}
