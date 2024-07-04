#[cfg(feature = "python")]
pub mod py;

/// The NPY file type.
pub struct Npy {}

/// Inherent implementation of the NPY file type.
impl Npy {
    /// Constructs and returns a new NPY file type.
    pub fn new(_file_path: &str) -> Self {
        Self {}
    }
}
