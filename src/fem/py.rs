use super::{
    super::py::PyIntermediateError, finite_element_data_from_inp, write_finite_elements_to_abaqus,
    write_finite_elements_to_exodus, write_finite_elements_to_mesh, write_finite_elements_to_vtk,
    Blocks, Connectivity, Coordinates,
};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<FiniteElements>()?;
    Ok(())
}

/// The finite elements class.
#[pyclass]
pub struct FiniteElements {
    element_blocks: Blocks,
    element_node_connectivity: Connectivity,
    nodal_coordinates: Coordinates,
}

#[pymethods]
impl FiniteElements {
    /// Constructs and returns a new finite elements type from data.
    #[new]
    pub fn from_data(
        element_blocks: Blocks,
        element_node_connectivity: Connectivity,
        nodal_coordinates: Coordinates,
    ) -> Self {
        Self {
            element_blocks,
            element_node_connectivity,
            nodal_coordinates,
        }
    }
    /// Constructs and returns a new finite elements type from an Abaqus file.
    #[staticmethod]
    pub fn from_inp(file_path: &str) -> Result<Self, PyIntermediateError> {
        let (element_blocks, element_node_connectivity, nodal_coordinates) =
            finite_element_data_from_inp(file_path)?;
        Ok(Self::from_data(
            element_blocks,
            element_node_connectivity,
            nodal_coordinates,
        ))
    }
    /// Smooths the nodal coordinates according to the provided smoothing method.
    pub fn smooth(&mut self, method: String) -> Result<(), PyIntermediateError> {
        Ok(Err(format!(
            "Invalid smoothing method {} specified.",
            method
        ))?)
    }
    /// Writes the finite elements data to a new Exodus file.
    pub fn write_exo(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_finite_elements_to_exodus(
            file_path,
            &self.element_blocks,
            &self.element_node_connectivity,
            &self.nodal_coordinates,
        )?)
    }
    /// Writes the finite elements data to a new Abaqus file.
    pub fn write_inp(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_finite_elements_to_abaqus(
            file_path,
            &self.element_blocks,
            &self.element_node_connectivity,
            &self.nodal_coordinates,
        )?)
    }
    /// Writes the finite elements data to a new mesh file.
    pub fn write_mesh(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_finite_elements_to_mesh(
            file_path,
            &self.element_blocks,
            &self.element_node_connectivity,
            &self.nodal_coordinates,
        )?)
    }
    /// Writes the finite elements data to a new VTK file.
    pub fn write_vtk(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_finite_elements_to_vtk(
            file_path,
            &self.element_blocks,
            &self.element_node_connectivity,
            &self.nodal_coordinates,
        )?)
    }
}
