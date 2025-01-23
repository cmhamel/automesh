use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    super::hex::py::register_module(parent_module)?;
    parent_module.add_class::<super::hex::py::HexahedralFiniteElements>()?;
    Ok(())
}
