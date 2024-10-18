//! [![book](https://img.shields.io/badge/automesh-Book-blue?logo=mdbook&logoColor=000000)](https://autotwin.github.io/automesh)
//! [![crates](https://img.shields.io/crates/v/automesh?logo=rust&logoColor=000000&label=Crates&color=32592f)](https://crates.io/crates/automesh)
//! [![docs](https://img.shields.io/badge/Docs-API-e57300?logo=docsdotrs&logoColor=000000)](https://docs.rs/automesh)
//! [![pypi](https://img.shields.io/pypi/v/automesh?logo=pypi&logoColor=FBE072&label=PyPI&color=4B8BBE)](https://pypi.org/project/automesh)
//! [![docs](https://img.shields.io/badge/Docs-API-8CA1AF?logo=readthedocs)](https://automesh.readthedocs.io)
//! [![DOI](https://img.shields.io/badge/DOI-10.5281/zenodo.13845433-blue)](https://doi.org/10.5281/zenodo.13845433)
//!
//! Automatic mesh generation.

#![doc(html_logo_url = "https://github.com/autotwin/automesh/blob/main/docs/logo.png?raw=true")]

#[cfg(feature = "python")]
mod py;

mod abaqus;
mod fem;
mod voxel;

pub use abaqus::Abaqus;
pub use fem::FiniteElements;
pub use voxel::Voxels;

const ELEMENT_NUMBERING_OFFSET: usize = 1;
const NODE_NUMBERING_OFFSET: usize = 1;
const NSD: usize = 3;
