/// The Exodus trait for finite elements.
pub trait Exodus {
    /// Writes the finite element data to a new Exodus mesh file.
    fn write_exo(&self, file_path: &str);
}
