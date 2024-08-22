use super::{
    finite_element_data_from_npy_data, spn_data_from_npy, spn_data_from_spn, write_spn_to_npy, Nel,
    Scale, SpnData, Translate,
};
use crate::fem::py::FiniteElements;
use numpy::{PyArray3, ToPyArray};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Spn>()?;
    Ok(())
}

#[pyclass]
pub struct Spn {
    data: SpnData,
}

#[pymethods]
impl Spn {
    #[pyo3(signature = (scale=[1.0, 1.0, 1.0], translate=[0.0, 0.0, 0.0]))]
    pub fn as_finite_elements(&self, scale: Scale, translate: Translate) -> FiniteElements {
        let (element_blocks, element_connectivity, nodal_coordinates) =
            finite_element_data_from_npy_data(&self.data, &scale, &translate);
        FiniteElements::from_data(element_blocks, element_connectivity, nodal_coordinates)
    }
    #[getter]
    pub fn get_data<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        self.data.to_pyarray_bound(python)
    }
    #[staticmethod]
    pub fn from_npy(file_path: &str) -> Self {
        let data = spn_data_from_npy(file_path);
        Self { data }
    }
    #[staticmethod]
    pub fn from_spn(file_path: &str, nel: Nel) -> Self {
        let data = spn_data_from_spn(file_path, nel);
        Self { data }
    }
    pub fn write_npy(&self, file_path: &str) {
        write_spn_to_npy(&self.data, file_path);
    }
}
