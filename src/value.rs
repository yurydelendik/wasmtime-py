use cpython::{PyInt, PyObject, PyResult, PyTuple, Python, PythonObject, ToPyObject};

use wasmtime_jit::{ActionOutcome, RuntimeValue};

use crate::exception::get_pyexception;

pub fn pyobj_to_runtime_value(py: Python, obj: &PyObject) -> PyResult<RuntimeValue> {
    let val = obj.extract::<i32>(py)?;
    Ok(RuntimeValue::I32(val))
}

pub fn outcome_into_pyobj(py: Python, outcome: ActionOutcome) -> PyResult<impl ToPyObject> {
    Ok(match outcome {
        ActionOutcome::Returned { values } => match values.len() {
            0 => PyTuple::empty(py).into_object(),
            1 => match values[0] {
                RuntimeValue::I32(x) => PyInt::new(py, x.into()).into_object(),
                _ => return get_pyexception(py, "return type unsupported"),
            },
            _ => return get_pyexception(py, "multivalue return unsupported"),
        },
        _ => return get_pyexception(py, "error during wasmcall"),
    })
}
