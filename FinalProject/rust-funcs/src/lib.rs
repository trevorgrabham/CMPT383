#[macro_use]
extern crate cpython; 

use cpython::{Python, PyResult, py_module_initializer, py_fn};


py_module_initializer!(rust_funcs, |py,m| {
    m.add(py, "double", py_fn!(py, double(x: i32)))?;
    Ok(())
});

fn double(_py: Python, x: i32) -> PyResult<i32> {
    Ok(x*2)
}
