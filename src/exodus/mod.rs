#[cfg(feature = "python")]
pub mod py;

pub type BlockConnectivity = Vec<u8>;
pub type ElementConnectivity = Vec<[usize; 8]>;
pub type NodalCoordinates = Vec<[f64; 3]>;

/// The Exodus file type.
pub struct Exodus {
    block_connectivity: BlockConnectivity,
    element_connectivity: ElementConnectivity,
    nodal_coordinates: NodalCoordinates,
}

/// Inherent implementation of the Exodus file type.
impl Exodus {
    /// Returns a reference to the block connectivity.
    pub fn get_block_connectivity(&self) -> &BlockConnectivity {
        &self.block_connectivity
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
        block_connectivity: BlockConnectivity,
        element_connectivity: ElementConnectivity,
        nodal_coordinates: NodalCoordinates,
    ) -> Self {
        Self {
            block_connectivity,
            element_connectivity,
            nodal_coordinates,
        }
    }
}
