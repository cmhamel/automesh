use pyo3::prelude::*;

mod exodus;
mod spn;

pub use exodus::Exodus;
pub use spn::Spn;

pub fn register_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let io = PyModule::new(py, "io")?;
    parent_module.add_submodule(io)?;
    exodus::register_module(py, io)?;
    spn::register_module(py, io)?;
    Ok(())
}
