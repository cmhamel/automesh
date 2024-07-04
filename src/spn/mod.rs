use super::Exodus;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(feature = "python")]
pub mod py;

type Data = Vec<Vec<Vec<u8>>>;

/// The SPN file type.
pub struct Spn {
    data: Data,
}

/// Inherent implementation of the SPN file type.
impl Spn {
    /// Constructs and returns a new Exodus file type from the SPN file data.
    pub fn exodus(&self) -> Exodus {
        Exodus {}
    }
    /// Returns a reference to the internal SPN file data.
    pub fn get_data(&self) -> &Data {
        &self.data
    }
    /// Constructs and returns a new SPN file type.
    pub fn new(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Self {
        let data = init_data(file_path, nelx, nely, nelz);
        Self { data }
    }
}

fn init_data(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Data {
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
    data
}
