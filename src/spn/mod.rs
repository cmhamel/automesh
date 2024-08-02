#[cfg(feature = "python")]
pub mod py;

#[cfg(test)]
pub mod test;

use super::{ElementBlocks, ElementConnectivity, Exodus, NodalCoordinates};
use itertools::Itertools;
use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const NODE_NUMBERING_OFFSET: usize = 1;

type Nel = [usize; 3];
type Scale = [f64; 3];
type SpnData = Array3<u8>;
type VoxelData<const N: usize> = Vec<[usize; N]>;

/// The SPN type.
pub struct Spn {
    data: SpnData,
    scale: Scale,
}

/// Inherent implementation of the SPN type.
impl Spn {
    /// Constructs and returns a new SPN type from an NPY file.
    pub fn from_npy(file_path: &str, scale: Scale) -> Self {
        let data = spn_data_from_npy(file_path);
        Self { data, scale }
    }
    /// Returns a reference to the internal SPN data.
    pub fn get_data(&self) -> &SpnData {
        &self.data
    }
    /// Returns a reference to the scale data.
    pub fn get_scale(&self) -> &Scale {
        &self.scale
    }
    #[doc = svgbobdoc::transform!(
    /// Converts the SPN type into an Exodus type, consuming the SPN type.
    ///
    /// ```svgbob
    ///     8       7
    ///      *-------*        +x
    ///   5 /|    6 /|          ^
    ///    *-+-----* |          |
    ///    | |4    | |3         |
    ///    | *-----|-*          +-----> -z
    ///    |/      |/          /
    ///    *-------*          v
    ///   1       2         -y
    /// ```
    )]
    pub fn into_exodus(self) -> Exodus {
        let (element_blocks, element_connectivity, nodal_coordinates) =
            exodus_data_from_npy_data(self.get_data(), self.get_scale());
        Exodus::from_data(element_blocks, element_connectivity, nodal_coordinates)
    }
    /// Constructs and returns a new SPN type from file.
    pub fn new(file_path: &str, nel: Nel, scale: Scale) -> Self {
        let data = new(file_path, nel);
        Self { data, scale }
    }
}

fn element_connectivity_node_renumbering(element_connectivity: &mut ElementConnectivity) {
    element_connectivity
        .clone()
        .into_iter()
        .flatten()
        .unique()
        .sorted()
        .enumerate()
        .filter(|(index, id)| &(index + 1) != id)
        .for_each(|(index, id)| {
            element_connectivity
                .iter_mut()
                .flatten()
                .filter(|entry| *entry == &id)
                .for_each(|entry| *entry = index + 1)
        });
}

fn exodus_data_from_npy_data(
    data: &SpnData,
    scale: &Scale,
) -> (ElementBlocks, ElementConnectivity, NodalCoordinates) {
    let shape = data.shape();
    let nelzplus1 = shape[0] + 1;
    let nelyplus1 = shape[1] + 1;
    let xscale = scale[0];
    let yscale = scale[1];
    let zscale = scale[2];
    let (filtered_voxel_data, element_blocks) = filter_spn_data(data);
    let mut element_connectivity: ElementConnectivity = filtered_voxel_data
        .iter()
        .map(|entry| {
            vec![
                entry[2] * nelzplus1 * nelyplus1
                    + entry[1] * nelzplus1
                    + entry[0]
                    + 1
                    + NODE_NUMBERING_OFFSET,
                entry[2] * nelzplus1 * nelyplus1
                    + entry[1] * nelzplus1
                    + entry[0]
                    + NODE_NUMBERING_OFFSET,
                entry[2] * nelzplus1 * nelyplus1
                    + (entry[1] + 1) * nelzplus1
                    + entry[0]
                    + NODE_NUMBERING_OFFSET,
                entry[2] * nelzplus1 * nelyplus1
                    + (entry[1] + 1) * nelzplus1
                    + entry[0]
                    + 1
                    + NODE_NUMBERING_OFFSET,
                (entry[2] + 1) * nelzplus1 * nelyplus1
                    + entry[1] * nelzplus1
                    + entry[0]
                    + 1
                    + NODE_NUMBERING_OFFSET,
                (entry[2] + 1) * nelzplus1 * nelyplus1
                    + entry[1] * nelzplus1
                    + entry[0]
                    + NODE_NUMBERING_OFFSET,
                (entry[2] + 1) * nelzplus1 * nelyplus1
                    + (entry[1] + 1) * nelzplus1
                    + entry[0]
                    + NODE_NUMBERING_OFFSET,
                (entry[2] + 1) * nelzplus1 * nelyplus1
                    + (entry[1] + 1) * nelzplus1
                    + entry[0]
                    + 1
                    + NODE_NUMBERING_OFFSET,
            ]
        })
        .collect();
    element_connectivity_node_renumbering(&mut element_connectivity);
    let number_of_nodes = element_connectivity
        .clone()
        .into_iter()
        .flatten()
        .unique()
        .collect::<Vec<usize>>()
        .len();
    let mut nodal_coordinates = vec![vec![0.0; 3]; number_of_nodes];
    filtered_voxel_data
        .iter()
        .zip(element_connectivity.iter())
        .for_each(|(entry, connectivity)| {
            nodal_coordinates[connectivity[0] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64) * xscale,
                (entry[1] as f64) * yscale,
                (entry[0] as f64 + 1.0) * zscale,
            ];
            nodal_coordinates[connectivity[1] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64) * xscale,
                (entry[1] as f64) * yscale,
                (entry[0] as f64) * zscale,
            ];
            nodal_coordinates[connectivity[2] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64) * xscale,
                (entry[1] as f64 + 1.0) * yscale,
                (entry[0] as f64) * zscale,
            ];
            nodal_coordinates[connectivity[3] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64) * xscale,
                (entry[1] as f64 + 1.0) * yscale,
                (entry[0] as f64 + 1.0) * zscale,
            ];
            nodal_coordinates[connectivity[4] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64 + 1.0) * xscale,
                (entry[1] as f64) * yscale,
                (entry[0] as f64 + 1.0) * zscale,
            ];
            nodal_coordinates[connectivity[5] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64 + 1.0) * xscale,
                (entry[1] as f64) * yscale,
                (entry[0] as f64) * zscale,
            ];
            nodal_coordinates[connectivity[6] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64 + 1.0) * xscale,
                (entry[1] as f64 + 1.0) * yscale,
                (entry[0] as f64) * zscale,
            ];
            nodal_coordinates[connectivity[7] - NODE_NUMBERING_OFFSET] = vec![
                (entry[2] as f64 + 1.0) * xscale,
                (entry[1] as f64 + 1.0) * yscale,
                (entry[0] as f64 + 1.0) * zscale,
            ];
        });
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

fn new(file_path: &str, nel: Nel) -> SpnData {
    let flat = BufReader::new(File::open(file_path).expect("File was not found."))
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<u8>>();
    let mut data = SpnData::zeros((nel[2], nel[1], nel[0]));
    data.iter_mut()
        .zip(flat.iter())
        .for_each(|(data_entry, flat_entry)| *data_entry = *flat_entry);
    data
}

fn spn_data_from_npy(file_path: &str) -> SpnData {
    SpnData::read_npy(File::open(file_path).unwrap()).unwrap()
}
