use super::exodus::Exodus;
use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::fs::File;

#[cfg(feature = "python")]
pub mod py;

type Data = Array3<u8>;

/// The NPY file type.
pub struct Npy {
    data: Data,
}

/// Inherent implementation of the NPY file type.
impl Npy {
    /// Constructs and returns a new Exodus file type from the NPY file data.
    pub fn exodus(&self) -> Exodus {
        let data = self.get_data();
        let shape = data.shape();
        let nelzplus1 = shape[0] + 1;
        let nelyplus1 = shape[1] + 1;
        let filtered_data: Vec<Vec<Vec<[usize; 4]>>> = data
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
                            .map(|(i, data_kji)| [k, j, i, *data_kji as usize])
                            .collect()
                    })
                    .collect()
            })
            .collect();
        let block_connectivity = filtered_data
            .iter()
            .flatten()
            .flatten()
            .map(|entry| entry[3])
            .collect();
        let element_connectivity = filtered_data
            .iter()
            .flatten()
            .flatten()
            .map(|entry| {
                [
                    entry[2] * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 1,
                    entry[2] * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 2,
                    entry[2] * nelzplus1 * nelyplus1 + (entry[1] + 1) * nelzplus1 + entry[0] + 2,
                    entry[2] * nelzplus1 * nelyplus1 + (entry[1] + 1) * nelzplus1 + entry[0] + 1,
                    (entry[2] + 1) * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 1,
                    (entry[2] + 1) * nelzplus1 * nelyplus1 + entry[1] * nelzplus1 + entry[0] + 2,
                    (entry[2] + 1) * nelzplus1 * nelyplus1
                        + (entry[1] + 1) * nelzplus1
                        + entry[0]
                        + 2,
                    (entry[2] + 1) * nelzplus1 * nelyplus1
                        + (entry[1] + 1) * nelzplus1
                        + entry[0]
                        + 1,
                ]
            })
            .collect();
        let nodal_coordinates = vec![[0.0; 3]];
        Exodus::new(block_connectivity, element_connectivity, nodal_coordinates)
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

fn new(file_path: &str) -> Data {
    Array3::read_npy(File::open(file_path).unwrap()).unwrap()
}
