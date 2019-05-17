//! Utility functions to handle values conversion between abstractions/targets.

use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyTuple};

use cranelift_codegen::ir;
use std::ptr;
use wasmtime_jit::{ActionOutcome, RuntimeValue};

pub fn pyobj_to_runtime_value(_: Python, obj: &PyAny, ty: ir::Type) -> PyResult<RuntimeValue> {
    Ok(match ty {
        ir::types::I32 => RuntimeValue::I32(obj.extract::<i32>()?),
        ir::types::I64 => RuntimeValue::I64(obj.extract::<i64>()?),
        ir::types::F32 => RuntimeValue::F32(obj.extract::<f32>()?.to_bits()),
        ir::types::F64 => RuntimeValue::F64(obj.extract::<f64>()?.to_bits()),
        _ => return Err(PyErr::new::<Exception, _>("unsupported value type")),
    })
}

pub fn default_value_for(_: Python, ty: ir::Type) -> PyResult<RuntimeValue> {
    Ok(match ty {
        ir::types::I32 => RuntimeValue::I32(0),
        ir::types::I64 => RuntimeValue::I64(0),
        ir::types::F32 => RuntimeValue::F32(0),
        ir::types::F64 => RuntimeValue::F64(0),
        _ => return Err(PyErr::new::<Exception, _>("unsupported value type")),
    })
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

pub unsafe fn read_value_from(py: Python, ptr: *mut i64, ty: ir::Type) -> PyObject {
    match ty {
        ir::types::I32 => ptr::read(ptr as *const i32).into_object(py),
        ir::types::I64 => ptr::read(ptr as *const i64).into_object(py),
        ir::types::F32 => ptr::read(ptr as *const f32).into_object(py),
        ir::types::F64 => ptr::read(ptr as *const f64).into_object(py),
        _ => panic!("TODO add PyResult to read_value_from"),
    }
}

pub unsafe fn write_value_to(py: Python, ptr: *mut i64, ty: ir::Type, val: PyObject) {
    match ty {
        ir::types::I32 => ptr::write(ptr as *mut i32, val.extract::<i32>(py).expect("i32")),
        ir::types::I64 => ptr::write(ptr as *mut i64, val.extract::<i64>(py).expect("i64")),
        ir::types::F32 => ptr::write(ptr as *mut f32, val.extract::<f32>(py).expect("f32")),
        ir::types::F64 => ptr::write(ptr as *mut f64, val.extract::<f64>(py).expect("f64")),
        _ => panic!("TODO add PyResult to write_value_to"),
    }
}
