use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::value::{default_value_for, outcome_into_pyobj, pyobj_to_runtime_value};
use std::cell::RefCell;
use std::rc::Rc;

use cranelift_codegen::ir;
use wasmtime_jit::{Context, InstanceHandle};

// TODO support non-export functions
#[pyclass]
pub struct Function {
    pub context: Rc<RefCell<Context>>,
    pub instance: InstanceHandle,
    pub export_name: String,
    pub args_types: Vec<ir::Type>,
}

#[pymethods]
impl Function {
    #[__call__]
    #[args(args = "*")]
    fn call(&self, py: Python, args: &PyTuple) -> PyResult<PyObject> {
        let mut runtime_args = Vec::new();
        for i in 0..self.args_types.len() {
            if i >= args.len() {
                runtime_args.push(default_value_for(py, self.args_types[i])?);
                continue;
            }
            runtime_args.push(pyobj_to_runtime_value(
                py,
                args.get_item(i),
                self.args_types[i],
            )?);
        }
        let mut instance = self.instance.clone();
        let outcome = self
            .context
            .borrow_mut()
            .invoke(&mut instance, self.export_name.as_str(), &runtime_args)
            .expect("good run");
        outcome_into_pyobj(py, outcome)
    }
}
