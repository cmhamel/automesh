use pyo3::prelude::*;

mod exodus;
mod spn;

pub use exodus::Exodus;
pub use spn::Spn;

#[pymodule]
fn automesh(m: &Bound<'_, PyModule>) -> PyResult<()> {
    exodus::register_module(m)?;
    spn::register_module(m)?;
    Ok(())
}
