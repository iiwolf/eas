use std::{collections::HashMap};
use cpython::{Python, PyResult, PyDict, PyObject, ToPyObject, PyFloat, ObjectProtocol, PyErr};
use eas::{python_component::PythonProcess, component::Value, execution_process::ExecutionProcess};

fn call_python(code: &str, variables: HashMap<String, Value>) -> HashMap<String, Value>{
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    for (key, value) in variables.iter() {
        match value {
            Value::Float(f) => locals.set_item(py, key, f).unwrap(),
            Value::Vectorf32(v) => locals.set_item(py, key, v).unwrap(),
        }
    }
    
    py.run(code, None, Some(&locals)).unwrap();

    let mut output_variables = HashMap::new();
    for (key, result) in locals.items(py).iter() {
        let key: String = key.extract(py).unwrap();
        match result.get_type(py).name(py).as_ref() {
            "float" => output_variables.insert(key, Value::Float(result.extract(py).unwrap())),
            "int" => todo!(),
            "str" => todo!(),
            "list" => output_variables.insert(key, Value::Vectorf32(result.extract(py).unwrap())),
            "dict" => todo!(),
            _ => {
                println!("Unsupported variable \"{}\" of type \"{}\"", key, result.get_type(py).name(py).as_ref());
                None
            },
        };
    }

    output_variables

}