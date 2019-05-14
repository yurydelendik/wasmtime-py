use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use wasmtime_jit;

use crate::value::{outcome_into_pyobj, pyobj_to_runtime_value};

#[pyclass]
pub struct WasmFn {
    pub instance_context: Rc<RefCell<InstanceContext>>,
    pub name: String,
}

#[pymethods]
impl WasmFn {
    #[__call__]
    #[args(args = "*")]
    fn call(&self, py: Python, args: &PyTuple) -> PyResult<PyObject> {
        let (mut jit_context, mut instance) =
            RefMut::map_split(self.instance_context.borrow_mut(), |ic| {
                (&mut ic.jit_context, &mut ic.instance)
            });
        let args = args
            .iter()
            .map(|i| pyobj_to_runtime_value(py, i))
            .collect::<PyResult<Vec<_>>>();
        let outcome = jit_context
            .invoke(&mut instance, self.name.as_str(), args?.as_slice())
            .expect("good run");
        outcome_into_pyobj(py, outcome)
    }
}

pub struct InstanceContext {
    pub jit_context: wasmtime_jit::Context,
    pub instance: wasmtime_jit::InstanceHandle,
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
        for (name, _) in self.context.borrow().instance.exports() {
            let f = Py::new(
                py,
                WasmFn {
                    instance_context: self.context.clone(),
                    name: name.to_string(),
                },
            )?;
            exports.set_item(name, f)?;
        }
        Ok(exports.to_object(py))
    }
}
