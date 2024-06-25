use pyo3::prelude::*;

pub fn register_module(_py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    parent_module.add_class::<Exodus>()?;
    Ok(())
}

#[pyclass]
pub struct Exodus {}

#[pymethods]
impl Exodus {
    #[new]
    pub fn init() -> Self {
        Self {}
    }
}
