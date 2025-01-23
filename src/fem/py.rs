use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<super::hex::py::HexahedralFiniteElements>()?;
    parent_module.add_class::<super::tri::py::TriangularFiniteElements>()?;
    Ok(())
}
