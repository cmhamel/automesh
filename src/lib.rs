#![doc = include_str!("../README.md")]

use pyo3::prelude::*;

pub mod io;

#[pymodule]
pub fn automesh(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    io::register_module(py, m)?;
    Ok(())
}
