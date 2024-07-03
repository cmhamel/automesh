use super::Exodus;
use pyo3::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Spn>()?;
    Ok(())
}

type Data = Vec<Vec<Vec<u8>>>;

#[pyclass]
pub struct Spn {
    data: Data,
}

#[pymethods]
impl Spn {
    pub fn exodus(&self) -> Exodus {
        let _ = self.data;
        Exodus {}
    }
    #[new]
    pub fn init(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Self {
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
        Self { data }
    }
}

impl Spn {
    pub fn get_data(&self) -> &Data {
        &self.data
    }
}
