use pyo3::prelude::*;
use pyo3::types::PyTuple;

use cranelift_codegen::ir;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::instance::InstanceContext;
use crate::value::{default_value_for, outcome_into_pyobj, pyobj_to_runtime_value};

// TODO support non-export functions
#[pyclass]
pub struct Function {
    pub instance_context: Rc<RefCell<InstanceContext>>,
    pub export_name: String,
    pub args_types: Vec<ir::Type>,
}

#[pymethods]
impl Function {
    #[__call__]
    #[args(args = "*")]
    fn call(&self, py: Python, args: &PyTuple) -> PyResult<PyObject> {
        let (mut jit_context, mut instance) =
            RefMut::map_split(self.instance_context.borrow_mut(), |ic| {
                (&mut ic.jit_context, &mut ic.instance)
            });
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
        let outcome = jit_context
            .invoke(&mut instance, self.export_name.as_str(), &runtime_args)
            .expect("good run");
        outcome_into_pyobj(py, outcome)
    }
}
