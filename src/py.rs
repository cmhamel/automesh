use pyo3::prelude::*;

#[pymodule]
fn automesh(m: &Bound<'_, PyModule>) -> PyResult<()> {
    super::fem::py::register_module(m)?;
    super::voxel::py::register_module(m)?;
    Ok(())
}
