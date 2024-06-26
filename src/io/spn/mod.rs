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
    data: Vec<Vec<Vec<u8>>>,
    scale: f64,
}

#[pymethods]
impl Spn {
    pub fn exodus(&self) -> Exodus {
        Exodus {}
    }
    #[new]
    pub fn init(file_path: &str, nelx: usize, nely: usize, nelz: usize, scale: f64) -> Self {
        let flat = BufReader::new(File::open(file_path).expect("File was not found."))
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect::<Vec<u8>>();
        let mut data = vec![vec![vec![0; nelx]; nely]; nelz];
        data.iter_mut()
            .flatten()
            .flatten()
            .zip(flat.iter())
            .for_each(|(data_entry, flat_entry)| *data_entry = *flat_entry);
        Self { data, scale }
    }
}
