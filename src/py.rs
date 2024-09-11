use pyo3::prelude::*;

/// [![book](https://img.shields.io/badge/automesh-Book-blue?logo=mdbook&logoColor=000000)](https://autotwin.github.io/automesh)
/// [![crates](https://img.shields.io/crates/v/automesh?logo=rust&logoColor=000000&label=Crates&color=32592f)](https://crates.io/crates/automesh)
/// [![docs](https://img.shields.io/badge/Docs-API-e57300?logo=docsdotrs&logoColor=000000)](https://docs.rs/automesh)
/// [![pypi](https://img.shields.io/pypi/v/automesh?logo=pypi&logoColor=FBE072&label=PyPI&color=4B8BBE)](https://pypi.org/project/automesh)
/// [![docs](https://img.shields.io/badge/Docs-API-8CA1AF?logo=readthedocs)](https://automesh.readthedocs.io)
///
/// Automatic mesh generation.
#[pymodule]
fn automesh(m: &Bound<'_, PyModule>) -> PyResult<()> {
    super::fem::py::register_module(m)?;
    super::voxel::py::register_module(m)?;
    Ok(())
}
