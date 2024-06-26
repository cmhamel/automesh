use pyo3::prelude::*;

mod exodus;
mod spn;

pub use exodus::Exodus;
pub use spn::Spn;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let io = PyModule::new_bound(parent_module.py(), "io")?;
    parent_module.add_submodule(&io)?;
    exodus::register_module(&io)?;
    spn::register_module(&io)?;
    Ok(())
}
