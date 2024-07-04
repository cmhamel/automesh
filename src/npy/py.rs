use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Npy>()?;
    Ok(())
}

#[pyclass]
pub struct Npy {}

#[pymethods]
impl Npy {
    #[new]
    pub fn new(_file_path: &str) -> Self {
        Self {}
    }
}
