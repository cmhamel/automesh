//! Automatic mesh generation.

pub use exodus::Exodus;
pub use npy::Npy;
pub use spn::Spn;

#[cfg(feature = "python")]
mod py;

mod exodus;
mod npy;
mod spn;
