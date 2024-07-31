#[cfg(feature = "python")]
pub mod py;

use super::{ElementBlocks, ElementConnectivity, NodalCoordinates, Spn};

/// The Exodus type.
pub struct Exodus {
    element_blocks: ElementBlocks,
    element_connectivity: ElementConnectivity,
    nodal_coordinates: NodalCoordinates,
}

/// Inherent implementation of the Exodus type.
impl Exodus {
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
    /// Writes the Exodus data to a new Exodus file.
    pub fn write(&self, _file_path: &str) {
        todo!("Writing Exodus types to file has not yet been implemented.")
    }
}

impl From<Spn> for Exodus {
    fn from(spn: Spn) -> Self {
        spn.into_exodus()
    }
}
