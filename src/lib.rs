use pyo3::prelude::*;

pub mod io;

#[pymodule]
fn automesh(m: &Bound<'_, PyModule>) -> PyResult<()> {
    io::register_module(m)?;
    Ok(())
}
