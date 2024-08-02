use crate::exodus::py::Exodus;
use numpy::{PyArray3, ToPyArray};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Spn>()?;
    Ok(())
}

#[pyclass]
pub struct Spn {
    data: super::SpnData,
    scale: super::Scale,
}

#[pymethods]
impl Spn {
    pub fn as_exodus(&self) -> Exodus {
        let (element_blocks, element_connectivity, nodal_coordinates) =
            super::exodus_data_from_npy_data(&self.data, &self.scale);
        Exodus::from_data(element_blocks, element_connectivity, nodal_coordinates)
    }
    #[getter]
    pub fn get_data<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        self.data.to_pyarray_bound(python)
    }
    #[staticmethod]
    pub fn from_npy(file_path: &str, scale: super::Scale) -> Self {
        let data = super::spn_data_from_npy(file_path);
        Self { data, scale }
    }
    #[new]
    pub fn new(file_path: &str, nel: super::Nel, scale: super::Scale) -> Self {
        let data = super::new(file_path, nel);
        Self { data, scale }
    }
}
