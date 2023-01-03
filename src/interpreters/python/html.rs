// Standard Uses

// Crate Uses

// External Uses
use pyo3::prelude::*;


#[pyfunction]
pub fn render() -> bool {
    todo!()
}


#[allow(unused)]
#[pymodule]
fn html(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render, m)?)?;
    Ok(())
}

