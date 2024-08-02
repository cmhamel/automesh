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
}

#[pymethods]
impl Spn {
    #[pyo3(signature = (scale=[1.0, 1.0, 1.0], translate=[0.0, 0.0, 0.0]))]
    pub fn as_exodus(&self, scale: super::Scale, translate: super::Translate) -> Exodus {
        let (element_blocks, element_connectivity, nodal_coordinates) =
            super::exodus_data_from_npy_data(&self.data, &scale, &translate);
        Exodus::from_data(element_blocks, element_connectivity, nodal_coordinates)
    }
    #[getter]
    pub fn get_data<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        self.data.to_pyarray_bound(python)
    }
    #[staticmethod]
    pub fn from_npy(file_path: &str) -> Self {
        let data = super::spn_data_from_npy(file_path);
        Self { data }
    }
    #[new]
    pub fn new(file_path: &str, nel: super::Nel) -> Self {
        let data = super::new(file_path, nel);
        Self { data }
    }
}
