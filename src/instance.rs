use cpython::{
    PyBytes, PyClone, PyDict, PyList, PyObject, PyResult, Python, PythonObject, ToPyObject,
};
use std::boxed::Box;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use wasmtime_jit;

use crate::module::Module;
use crate::value::{outcome_into_pyobj, pyobj_to_runtime_value};

py_class!(pub class WasmFn |py| {
    data instance: PyObject;
    data name: String;
    def call(&self, args: PyList) -> PyResult<impl ToPyObject> {
        let instance_obj = self.instance(py).cast_as::<Instance>(py)?;
        let mut instance = instance_obj.instance(py).borrow_mut();
        let mut context = instance_obj.context(py).borrow_mut();
        let args = args.iter(py).map(|i| pyobj_to_runtime_value(py, &i)).collect::<PyResult<Vec<_>>>();
        let outcome = context.invoke(&mut instance, self.name(py).as_str(), args?.as_slice()).expect("good run");
        outcome_into_pyobj(py, outcome)
    }
    def __call__(&self) -> PyResult<impl ToPyObject> {
        let instance_obj = self.instance(py).cast_as::<Instance>(py)?;
        let mut instance = instance_obj.instance(py).borrow_mut();
        let mut context = instance_obj.context(py).borrow_mut();
        let outcome = context.invoke(&mut instance, self.name(py).as_str(), &[]).expect("good run");
        outcome_into_pyobj(py, outcome)
    }
});

py_class!(pub class Instance |py| {
    data context: RefCell<wasmtime_jit::Context>;
    data instance: RefCell<wasmtime_jit::InstanceHandle>;
    def get_exports(&self) -> PyResult<PyDict> {
        let exports = PyDict::new(py);
        for (name, _) in self.instance(py).borrow().exports() {
            let f = WasmFn::create_instance(py, self.as_object().clone_ref(py), name.to_string())?;
            exports.set_item(py, name, f)?;
        }
        Ok(exports)
    }
});

py_class!(pub class InstantiateResultObject |py| {
    data instance: PyObject;
    data module: PyObject;
    def get_instance(&self) -> PyResult<PyObject> {
        Ok(self.instance(py).clone_ref(py))
    }
    def get_module(&self) -> PyResult<PyObject> {
        Ok(self.module(py).clone_ref(py))
    }
});

pub fn instantiate_py(
    py: Python,
    buffer_source: PyBytes,
    _import_obj: PyDict,
) -> PyResult<InstantiateResultObject> {
    let wasm_data = buffer_source.data(py);

    let isa = {
        let isa_builder =
            cranelift_native::builder().expect("host machine is not a supported target");
        let flag_builder = cranelift_codegen::settings::builder();
        isa_builder.finish(cranelift_codegen::settings::Flags::new(flag_builder))
    };
    let mut compiler = wasmtime_jit::Compiler::new(isa);
    let mut resolver = wasmtime_jit::NullResolver {};
    let global_exports = Rc::new(RefCell::new(HashMap::new()));
    let mut module = Cell::new(
        wasmtime_jit::CompiledModule::new(
            &mut compiler,
            wasm_data,
            &mut resolver,
            global_exports,
            true,
        )
        .expect("compiled"),
    );

    let mut context = wasmtime_jit::Context::new(Box::new(compiler));
    context.set_debug_info(true);
    let _global_exports = context.get_global_exports();

    let instance = module.get_mut().instantiate().expect("instance");

    let context = RefCell::new(context);
    let instance = RefCell::new(instance);

    let module = Module::new(py, module)?.into_object();
    let instance = Instance::create_instance(py, context, instance)?.into_object();
    let result = InstantiateResultObject::create_instance(py, instance, module)?;
    Ok(result)
}
