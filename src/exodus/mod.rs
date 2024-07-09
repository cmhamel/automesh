#[cfg(feature = "python")]
pub mod py;

pub type ElementBlocks = Vec<usize>;
pub type ElementConnectivity = Vec<Vec<usize>>;
pub type NodalCoordinates = Vec<Vec<f64>>;

/// The Exodus file type.
pub struct Exodus {
    element_blocks: ElementBlocks,
    element_connectivity: ElementConnectivity,
    nodal_coordinates: NodalCoordinates,
}

/// Inherent implementation of the Exodus file type.
impl Exodus {
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
    /// Constructs and returns a new Exodus file type.
    pub fn new(
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
}
