use cpython::{exc, PyErr, PyObject, PyResult, PyString, PyTuple, Python, PythonObject};

pub fn get_pyexception(py: Python, msg: &str) -> PyResult<PyObject> {
    let et = py.get_type::<exc::Exception>();
    let inst = et.call(
        py,
        PyTuple::new(py, &[PyString::new(py, msg).into_object()]),
        None,
    )?;
    Err(PyErr {
        ptype: et.into_object(),
        pvalue: Some(inst),
        ptraceback: None,
    })
}
