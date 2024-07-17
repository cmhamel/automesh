use super::exodus::{ElementBlocks, ElementConnectivity, Exodus, NodalCoordinates};
use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::fs::File;

#[cfg(feature = "python")]
pub mod py;

#[cfg(test)]
pub mod test;

type Data = Array3<u8>;
type VoxelData<const N: usize> = Vec<[usize; N]>;

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

fn filter_data(data: &Data) -> (VoxelData<3>, ElementBlocks) {
    let filtered_voxel_data_combo: VoxelData<4> = data
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
        .collect::<Vec<Vec<Vec<[usize; 4]>>>>()
        .into_iter()
        .flatten()
        .flatten()
        .collect();
    let element_blocks = filtered_voxel_data_combo
        .iter()
        .map(|entry| entry[3])
        .collect();
    let filtered_voxel_data = filtered_voxel_data_combo
        .into_iter()
        .map(|entry| [entry[0], entry[1], entry[2]])
        .collect();
    (filtered_voxel_data, element_blocks)
}

fn exodus(data: &Data) -> (ElementBlocks, ElementConnectivity, NodalCoordinates) {
    let shape = data.shape();
    let nelzplus1 = shape[0] + 1;
    let nelyplus1 = shape[1] + 1;
    let (filtered_voxel_data, element_blocks) = filter_data(data);
    let element_connectivity = filtered_voxel_data
        .iter()
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
