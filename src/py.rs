use pyo3::prelude::*;

#[pymodule]
fn automesh(m: &Bound<'_, PyModule>) -> PyResult<()> {
    super::fem::py::register_module(m)?;
    super::spn::py::register_module(m)?;
    Ok(())
}
