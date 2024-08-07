#[cfg(feature = "python")]
pub mod py;

use super::{abaqus::Abaqus, exodus::Exodus};

pub type ElementBlocks = Vec<usize>;
pub type ElementConnectivity = Vec<Vec<usize>>;
pub type NodalCoordinates = Vec<Vec<f64>>;

/// The finite element type.
pub struct FiniteElements {
    element_blocks: ElementBlocks,
    element_connectivity: ElementConnectivity,
    nodal_coordinates: NodalCoordinates,
}

/// Inherent implementation of the finite element type.
impl FiniteElements {
    /// Constructs and returns a new Exodus type from data.
    pub fn from_data(
        element_blocks: ElementBlocks,
        element_connectivity: ElementConnectivity,
        nodal_coordinates: NodalCoordinates,
    ) -> Self {
        Self {
            element_blocks,
            element_connectivity,
            nodal_coordinates,
        }
    }
    /// Returns a reference to the element blocks.
    pub fn get_element_blocks(&self) -> &ElementBlocks {
        &self.element_blocks
    }
    /// Returns a reference to the element connectivity.
    pub fn get_element_connectivity(&self) -> &ElementConnectivity {
        &self.element_connectivity
    }
    /// Returns a reference to the nodal coordinates.
    pub fn get_nodal_coordinates(&self) -> &NodalCoordinates {
        &self.nodal_coordinates
    }
}

/// Abaqus implementation of the finite element type.
impl Abaqus for FiniteElements {
    fn write_inp(&self, _file_path: &str) {
        write_fem_to_inp(
            self.get_element_blocks(),
            self.get_element_connectivity(),
            self.get_nodal_coordinates(),
        )
    }
}

/// Exodus implementation of the finite element type.
impl Exodus for FiniteElements {
    fn write_exo(&self, _file_path: &str) {
        todo!("Writing Exodus files has not yet been implemented.")
    }
}

fn write_fem_to_inp(
    _element_blocks: &ElementBlocks,
    _element_connectivity: &ElementConnectivity,
    _nodal_coordinates: &NodalCoordinates,
) {
    todo!("Writing Abaqus files has not yet been implemented.")
}
