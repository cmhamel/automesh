use super::{write_fem_to_inp, Abaqus, Connectivity, ElementBlocks, Exodus, NodalCoordinates};
use numpy::{PyArray1, PyArray2};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<FiniteElements>()?;
    Ok(())
}

/// The finite element type.
#[pyclass]
pub struct FiniteElements {
    element_blocks: ElementBlocks,
    element_connectivity: Connectivity,
    nodal_coordinates: NodalCoordinates,
}

#[pymethods]
impl FiniteElements {
    /// Constructs and returns a new Exodus type from data.
    #[new]
    pub fn from_data(
        element_blocks: ElementBlocks,
        element_connectivity: Connectivity,
        nodal_coordinates: NodalCoordinates,
    ) -> Self {
        Self {
            element_blocks,
            element_connectivity,
            nodal_coordinates,
        }
    }
    /// The block for each finite element.
    #[getter]
    pub fn get_element_blocks<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray1<usize>> {
        PyArray1::from_vec_bound(python, self.element_blocks.clone())
    }
    /// The nodal connectivity for each finite element.
    #[getter]
    pub fn get_element_connectivity<'py>(
        &self,
        python: Python<'py>,
    ) -> Bound<'py, PyArray2<usize>> {
        PyArray2::from_vec2_bound(python, &self.element_connectivity).unwrap()
    }
    /// The nodal coordinates for each finite element.
    #[getter]
    pub fn get_nodal_coordinates<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        PyArray2::from_vec2_bound(python, &self.nodal_coordinates).unwrap()
    }
    /// Writes the finite element data to a new Abaqus input file.
    pub fn write_inp(&self, file_path: &str) {
        Abaqus::write_inp(self, file_path)
    }
    /// Writes the finite element data to a new Exodus mesh file.
    pub fn write_exo(&self, file_path: &str) {
        Exodus::write_exo(self, file_path)
    }
}

impl Abaqus for FiniteElements {
    fn write_inp(&self, file_path: &str) {
        write_fem_to_inp(
            file_path,
            &self.element_blocks,
            &self.element_connectivity,
            &self.nodal_coordinates,
        )
    }
}

impl Exodus for FiniteElements {
    fn write_exo(&self, _file_path: &str) {
        todo!("Writing Exodus files has not yet been implemented.")
    }
}
