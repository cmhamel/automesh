use super::{write_fem_to_inp, Abaqus, Blocks, Connectivity, Coordinates, Exodus};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<FiniteElements>()?;
    Ok(())
}

/// The finite element type.
#[pyclass]
pub struct FiniteElements {
    element_blocks: Blocks,
    element_node_connectivity: Connectivity,
    nodal_coordinates: Coordinates,
}

#[pymethods]
impl FiniteElements {
    /// Constructs and returns a new Exodus type from data.
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
            &self.element_node_connectivity,
            &self.nodal_coordinates,
        )
    }
}

impl Exodus for FiniteElements {
    fn write_exo(&self, _file_path: &str) {
        todo!("Writing Exodus files has not yet been implemented.")
    }
}
