#[cfg(feature = "python")]
pub mod py;

type ElementConnectivity = Vec<[usize; 8]>;
type NodalCoordinates = Vec<[f64; 3]>;

/// The Exodus file type.
pub struct Exodus {
    element_connectivity: ElementConnectivity,
    nodal_coordinates: NodalCoordinates,
}

/// Inherent implementation of the Exodus file type.
impl Exodus {
    /// Returns a reference to the element connectivity.
    pub fn get_element_connectivity(&self) -> &ElementConnectivity {
        &self.element_connectivity
    }
    /// Returns a reference to the nodal coordinates.
    pub fn get_nodal_coordinates(&self) -> &NodalCoordinates {
        &self.nodal_coordinates
    }
    /// Constructs and returns a new Exodus file type.
    pub fn new(_file_path: &str) -> Self {
        todo!()
    }
    /// Writes the Exodus data to a new Exodus file.
    pub fn write(&self, _file_path: &str) {
        todo!()
    }
}
