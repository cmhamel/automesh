use super::{
    super::py::PyIntermediateError, write_fem_to_exo, write_fem_to_inp, Blocks, Connectivity,
    Coordinates,
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
    /// Writes the finite elements data to a new Exodus file.
    pub fn write_exo(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_fem_to_exo(
            file_path,
            &self.element_blocks,
            &self.element_node_connectivity,
            &self.nodal_coordinates,
        )?)
    }
    /// Writes the finite elements data to a new Abaqus file.
    pub fn write_inp(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_fem_to_inp(
            file_path,
            &self.element_blocks,
            &self.element_node_connectivity,
            &self.nodal_coordinates,
        )?)
    }
}
