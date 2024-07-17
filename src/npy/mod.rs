use super::exodus::{ElementBlocks, ElementConnectivity, Exodus, NodalCoordinates};
use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::fs::File;

#[cfg(feature = "python")]
pub mod py;

type Data = Array3<u8>;
type LatticeData = Vec<Vec<Vec<Vec<usize>>>>;

/// The NPY file type.
pub struct Npy {
    data: Data,
}

/// Inherent implementation of the NPY file type.
impl Npy {
    /// Constructs and returns a new Exodus file type from the NPY file data.
    pub fn exodus(&self) -> Exodus {
        let (element_blocks, element_connectivity, nodal_coordinates) = exodus(self.get_data());
        Exodus::new(element_blocks, element_connectivity, nodal_coordinates)
    }
    /// Returns a reference to the internal NPY file data.
    pub fn get_data(&self) -> &Data {
        &self.data
    }
    /// Constructs and returns a new NPY file type.
    pub fn new(file_path: &str) -> Self {
        let data = new(file_path);
        Self { data }
    }
}

fn filter(data: &Data) -> (LatticeData, ElementBlocks) {
    let filtered_lattice_data: LatticeData = data
        .outer_iter()
        .enumerate()
        .map(|(k, data_k)| {
            data_k
                .outer_iter()
                .enumerate()
                .map(|(j, data_kj)| {
                    data_kj
                        .iter()
                        .enumerate()
                        .filter(|(_, &data_kji)| data_kji > 0)
                        .map(|(i, data_kji)| vec![k, j, i, *data_kji as usize])
                        .collect()
                })
                .collect()
        })
        .collect();
    let element_blocks = filtered_lattice_data
        .iter()
        .flatten()
        .flatten()
        .map(|entry| entry[3])
        .collect();
    let lattice_data = filtered_lattice_data
        .into_iter()
        .map(|data_k| {
            data_k
                .into_iter()
                .map(|data_kj| {
                    data_kj
                        .into_iter()
                        .map(|data_kji| data_kji.into_iter().take(3).collect())
                        .collect()
                })
                .collect()
        })
        .collect();
    (lattice_data, element_blocks)
}

fn exodus(data: &Data) -> (ElementBlocks, ElementConnectivity, NodalCoordinates) {
    let shape = data.shape();
    let nelzplus1 = shape[0] + 1;
    let nelyplus1 = shape[1] + 1;
    let (lattice_data, element_blocks) = filter(data);
    let element_connectivity = lattice_data
        .iter()
        .flatten()
        .flatten()
        .map(|entry| {
            vec![
                entry[2] * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 1,
                entry[2] * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 2,
                entry[2] * nelzplus1 * nelyplus1 + (entry[1] + 1) * nelzplus1 + entry[0] + 2,
                entry[2] * nelzplus1 * nelyplus1 + (entry[1] + 1) * nelzplus1 + entry[0] + 1,
                (entry[2] + 1) * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 1,
                (entry[2] + 1) * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 2,
                (entry[2] + 1) * nelzplus1 * nelyplus1 + (entry[1] + 1) * nelzplus1 + entry[0] + 2,
                (entry[2] + 1) * nelzplus1 * nelyplus1 + (entry[1] + 1) * nelzplus1 + entry[0] + 1,
            ]
        })
        .collect();
    let nodal_coordinates = vec![vec![0.0; 3]];
    (element_blocks, element_connectivity, nodal_coordinates)
}

fn new(file_path: &str) -> Data {
    Array3::read_npy(File::open(file_path).unwrap()).unwrap()
}
