use pyo3::prelude::*;
use pyo3::types::PyAny;

use wasmtime_jit::InstanceHandle;

pub fn into_instance_from_obj(_py: Python, _obj: &PyAny) -> PyResult<InstanceHandle> {
    panic!("not implemented");
}
