//! Automatic mesh generation.

#[cfg(feature = "python")]
mod py;

mod exodus;
mod spn;

pub use exodus::Exodus;
pub use spn::Spn;

type ElementBlocks = Vec<usize>;
type ElementConnectivity = Vec<Vec<usize>>;
type NodalCoordinates = Vec<Vec<f64>>;
