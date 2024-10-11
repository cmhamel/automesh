use std::io::Error;

/// The Abaqus trait for finite elements.
pub trait Abaqus {
    /// Writes the finite elements data to a new Abaqus input file.
    fn write_inp(&self, file_path: &str) -> Result<(), Error>;
}
