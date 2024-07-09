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
        // let number_of_elements = self.get_data().iter().filter(|&entry| entry > &0).count();
        // let mut block_connectivity = Vec::with_capacity(number_of_elements);

        // Exodus needs this, right?
        let block_connectivity = self
            .get_data()
            .iter()
            .filter(|&entry| entry > &0)
            .copied()
            .collect();

        // Need to filter() out "air" elements
        let element_connectivity = vec![[0; 8]];

        // Need to filter() out nodes only beloning to "air" elements
        let nodal_coordinates = vec![[0.0; 3]];
        // self.get_data()
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, &entry)| entry > 0)
        //     .for_each(|(i, entry)| println!("{:?}", (i, entry)));
        self.get_data()
            .outer_iter()
            .enumerate()
            .for_each(|(k, data_k)| {
                data_k.outer_iter().enumerate().for_each(|(j, data_kj)| {
                    data_kj
                        .iter()
                        .enumerate()
                        .filter(|(_, &data_kji)| data_kji > 0)
                        .for_each(
                            |(i, data_kji)| println!("{:?}", (k, j, i, data_kji)), // every (i, j, k) is origin of a hex
                                                                                   // then combos of +1 to each give other 7 vertices
                                                                                   // but can't double-count for nodal coordinates
                                                                                   // but do need to track for connectivity
                        )
                })
            });

        // Would the nodal connectivity (elements connected to each node) help you here?
        // And would that be worth saving in the struct?

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
