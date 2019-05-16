use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::function::Function;
use std::cell::RefCell;
use std::rc::Rc;

use wasmtime_environ::Export;
use wasmtime_jit::{Context, InstanceHandle};
use wasmtime_runtime::Export as RuntimeExport;

pub struct InstanceContext {
    pub jit_context: Context,
    pub instance: InstanceHandle,
}

#[pyclass]
pub struct Instance {
    pub context: Rc<RefCell<InstanceContext>>,
}

#[pymethods]
impl Instance {
    #[getter(exports)]
    fn get_exports(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let exports = PyDict::new(py);
        let instance = &mut self.context.borrow_mut().instance;
        let mut function_exports = Vec::new();
        for (name, export) in instance.exports() {
            match export {
                Export::Function(_) => function_exports.push(name.to_string()),
                _ => {
                    // Skip unknown export type.
                    continue;
                }
            }
        }
        for name in function_exports {
            if let Some(RuntimeExport::Function { signature, .. }) = instance.lookup(&name) {
                // TODO Annotate params/result
                let mut args_types = Vec::new();
                for index in 1..signature.params.len() {
                    args_types.push(signature.params[index].value_type);
                }
                let f = Py::new(
                    py,
                    Function {
                        instance_context: self.context.clone(),
                        export_name: name.clone(),
                        args_types,
                    },
                )?;
                exports.set_item(name, f)?;
            } else {
                panic!("function is exported");
            }
        }

        Ok(exports.to_object(py))
    }
}
