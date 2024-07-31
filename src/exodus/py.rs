use numpy::{PyArray1, PyArray2};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Exodus>()?;
    Ok(())
}

#[pyclass]
pub struct Exodus {
    element_blocks: super::ElementBlocks,
    element_connectivity: super::ElementConnectivity,
    nodal_coordinates: super::NodalCoordinates,
}

#[pymethods]
impl Exodus {
    #[new]
    pub fn from_data(
        element_blocks: super::ElementBlocks,
        element_connectivity: super::ElementConnectivity,
        nodal_coordinates: super::NodalCoordinates,
    ) -> Self {
        Self {
            element_blocks,
            element_connectivity,
            nodal_coordinates,
        }
    }
    #[getter]
    pub fn get_element_blocks<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray1<usize>> {
        PyArray1::from_vec_bound(python, self.element_blocks.clone())
    }
    #[getter]
    pub fn get_element_connectivity<'py>(
        &self,
        python: Python<'py>,
    ) -> Bound<'py, PyArray2<usize>> {
        PyArray2::from_vec2_bound(python, &self.element_connectivity).unwrap()
    }
    #[getter]
    pub fn get_nodal_coordinates<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        PyArray2::from_vec2_bound(python, &self.nodal_coordinates).unwrap()
    }
}
