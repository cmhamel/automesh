use super::Exodus;
use pyo3::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn register_module(_py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    parent_module.add_class::<Spn>()?;
    Ok(())
}

#[pyclass]
pub struct Spn {
    data: Vec<u8>,
    nelx: usize,
    nely: usize,
    nelz: usize,
    scale: f64,
}

#[pymethods]
impl Spn {
    pub fn exodus(&self) -> Exodus {
        Exodus {}
    }
    #[new]
    pub fn init(file_path: &str, nelx: usize, nely: usize, nelz: usize, scale: f64) -> Self {
        let data = BufReader::new(File::open(file_path).expect("File was not found."))
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        Self {
            data,
            nelx,
            nely,
            nelz,
            scale,
        }
    }
}
