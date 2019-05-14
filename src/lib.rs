use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use pyo3::wrap_pyfunction;

use crate::import::into_instance_from_obj;
use crate::instance::{Instance, InstanceContext};
use crate::module::Module;
use std::cell::RefCell;
use std::rc::Rc;

mod function;
mod import;
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
    import_obj: &PyDict,
) -> PyResult<Py<InstantiateResultObject>> {
    let wasm_data = buffer_source.as_bytes();

    let generate_debug_info = false;

    let isa = {
        let isa_builder =
            cranelift_native::builder().expect("host machine is not a supported target");
        let flag_builder = cranelift_codegen::settings::builder();
        isa_builder.finish(cranelift_codegen::settings::Flags::new(flag_builder))
    };

    let mut context = wasmtime_jit::Context::with_isa(isa);
    context.set_debug_info(generate_debug_info);

    for (name, obj) in import_obj.iter() {
        context.name_instance(
            name.to_string(),
            into_instance_from_obj(py, obj).expect("obj instance"),
        )
    }

    let instance = context
        .instantiate_module(None, wasm_data)
        .expect("instance");

    let module = Py::new(
        py,
        Module {
            module: instance.module(),
        },
    )?;
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
