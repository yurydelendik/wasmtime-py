use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyTuple};

use wasmtime_jit::{ActionOutcome, RuntimeValue};

pub fn pyobj_to_runtime_value(_: Python, obj: &PyAny) -> PyResult<RuntimeValue> {
    let val = obj.extract::<i32>()?;
    Ok(RuntimeValue::I32(val))
}

pub fn outcome_into_pyobj(py: Python, outcome: ActionOutcome) -> PyResult<PyObject> {
    Ok(match outcome {
        ActionOutcome::Returned { values } => match values.len() {
            0 => PyTuple::empty(py).into_object(py),
            1 => match values[0] {
                RuntimeValue::I32(x) => x.into_object(py),
                _ => return Err(PyErr::new::<Exception, _>("return type unsupported")),
            },
            _ => return Err(PyErr::new::<Exception, _>("multivalue return unsupported")),
        },
        _ => return Err(PyErr::new::<Exception, _>("error during wasmcall")),
    })
}
