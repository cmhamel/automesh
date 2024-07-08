use crate::exodus::py::Exodus;
use numpy::PyArray3;
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
    pub fn get_data<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        PyArray3::from_vec3_bound(python, &self.data).unwrap()
    }
    pub fn exodus(&self) -> Exodus {
        let _ = self.data;
        Exodus {}
    }
    #[new]
    pub fn new(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Self {
        let data = super::new(file_path, nelx, nely, nelz);
        Self { data }
    }
}
