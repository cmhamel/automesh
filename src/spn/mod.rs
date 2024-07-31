#[cfg(feature = "python")]
pub mod py;

#[cfg(test)]
pub mod test;

use super::{ElementBlocks, ElementConnectivity, Exodus, NodalCoordinates};
use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type SpnData = Array3<u8>;
type VoxelData<const N: usize> = Vec<[usize; N]>;

/// The SPN type.
pub struct Spn {
    data: SpnData,
}

/// Inherent implementation of the SPN type.
impl Spn {
    /// Constructs and returns a new SPN type from an NPY file.
    pub fn from_npy(file_path: &str) -> Self {
        let data = spn_data_from_npy(file_path);
        Self { data }
    }
    /// Returns a reference to the internal SPN data.
    pub fn get_data(&self) -> &SpnData {
        &self.data
    }
    /// Converts the SPN type into an Exodus type, consuming the SPN type.
    pub fn into_exodus(self) -> Exodus {
        let (element_blocks, element_connectivity, nodal_coordinates) =
            exodus_data_from_npy_data(self.get_data());
        Exodus::from_data(element_blocks, element_connectivity, nodal_coordinates)
    }
    /// Constructs and returns a new SPN type from file.
    pub fn new(file_path: &str, nelx: usize, nely: usize, nelz: usize) -> Self {
        let data = new(file_path, nelx, nely, nelz);
        Self { data }
    }
}

fn exodus_data_from_npy_data(
    data: &SpnData,
) -> (ElementBlocks, ElementConnectivity, NodalCoordinates) {
    let shape = data.shape();
    let nelzplus1 = shape[0] + 1;
    let nelyplus1 = shape[1] + 1;
    let (filtered_voxel_data, element_blocks) = filter_spn_data(data);
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

fn filter_spn_data(data: &SpnData) -> (VoxelData<3>, ElementBlocks) {
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

fn new(file_path: &str, nelz: usize, nely: usize, nelx: usize) -> SpnData {
    let flat = BufReader::new(File::open(file_path).expect("File was not found."))
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<u8>>();
    let mut data = SpnData::zeros((nelz, nely, nelx));
    data.iter_mut()
        .zip(flat.iter())
        .for_each(|(data_entry, flat_entry)| *data_entry = *flat_entry);
    data
}

fn spn_data_from_npy(file_path: &str) -> SpnData {
    SpnData::read_npy(File::open(file_path).unwrap()).unwrap()
}
