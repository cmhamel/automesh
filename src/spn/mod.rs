use super::Exodus;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(feature = "python")]
pub mod py;

type Data = Vec<Vec<Vec<u8>>>;

pub struct Spn {
    data: Data,
}

impl Spn {
    pub fn exodus(&self) -> Exodus {
        let _ = self.data;
        Exodus {}
    }
    pub fn get_data(&self) -> &Data {
        &self.data
    }
    pub fn init(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Self {
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
