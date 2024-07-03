use super::super::py::Exodus;
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Spn>()?;
    Ok(())
}

#[pyclass]
pub struct Spn {
    data: super::Data,
}

#[pymethods]
impl Spn {
    pub fn exodus(&self) -> Exodus {
        let _ = self.data;
        Exodus {}
    }
    #[new]
    pub fn init(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Self {
        let data = super::init_data(file_path, nelx, nely, nelz);
        Self { data }
    }
}
