#[cfg(feature = "python")]
pub mod py;

/// The Exodus file type.
pub struct Exodus {}

/// Inherent implementation of the Exodus file type.
impl Exodus {
    /// Constructs and returns a new Exodus file type.
    pub fn new(_file_path: &str) -> Self {
        Self {}
    }
}
