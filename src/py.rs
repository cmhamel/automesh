use pyo3::prelude::*;

pub use super::{exodus::py::Exodus, spn::py::Spn};

#[pymodule]
fn automesh(m: &Bound<'_, PyModule>) -> PyResult<()> {
    super::exodus::py::register_module(m)?;
    super::spn::py::register_module(m)?;
    Ok(())
}
