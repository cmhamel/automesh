use crate::exodus::py::Exodus;
use numpy::{PyArray3, ToPyArray};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Npy>()?;
    Ok(())
}

#[pyclass]
pub struct Npy {
    data: super::Data,
}

#[pymethods]
impl Npy {
    pub fn get_data<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        self.data.to_pyarray_bound(python)
    }
    pub fn exodus(&self) -> Exodus {
        todo!()
    }
    #[new]
    pub fn new(file_path: &str) -> Self {
        let data = super::new(file_path);
        Self { data }
    }
}
