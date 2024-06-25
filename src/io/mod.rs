use pyo3::prelude::*;

mod spn;

pub use spn::Spn;

pub fn register_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let io = PyModule::new(py, "io")?;
    spn::register_module(py, io)?;
    parent_module.add_submodule(io)?;
    Ok(())
}
