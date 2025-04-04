use pyo3::prelude::*;

/// A function callable from Rust
pub fn some_function() {
    println!("Hello from some_function in my_rust_lib!");
}

/// Optional logic reused in Python
fn double_in_rust(x: usize) -> usize {
    x * 2
}

#[pyfunction]
fn double(x: usize) -> usize {
    double_in_rust(x)
}

#[pymodule]
fn my_rust_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    Ok(())
}
