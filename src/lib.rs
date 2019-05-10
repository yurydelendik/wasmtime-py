#[macro_use]
extern crate cpython;
use cpython::{PyBytes, PyDict};

use crate::instance::{instantiate_py, Instance, InstantiateResultObject};
use crate::module::Module;

mod exception;
mod instance;
mod module;
mod value;

py_module_initializer!(wasmtime_py, initwasmtime_py, PyInit_wasmtime_py, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add_class::<Instance>(py)?;
    m.add_class::<Module>(py)?;
    m.add_class::<InstantiateResultObject>(py)?;
    m.add(
        py,
        "instantiate",
        py_fn!(
            py,
            instantiate_py(buffer_source: PyBytes, import_obj: PyDict)
        ),
    )?;
    Ok(())
});
