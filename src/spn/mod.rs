#[cfg(feature = "python")]
pub mod py;

#[cfg(test)]
pub mod test;

use super::fem::{ElementBlocks, ElementConnectivity, FiniteElements, NodalCoordinates};
use itertools::Itertools;
use ndarray::{Array3, Axis};
use ndarray_npy::{ReadNpyExt, WriteNpyExt};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, ErrorKind},
};

const NODE_NUMBERING_OFFSET: usize = 1;

// TODO:
// SPN is a file type, shouldn't the struct be Voxels or something?

type Nel = [usize; 3];
type Scale = [f64; 3];
type SpnData = Array3<u8>;
type Translate = [f64; 3];
type VoxelData<const N: usize> = Vec<[usize; N]>;

/// The SPN type.
///
/// A SPN file is a text (human-readable) file
/// that contains a single a column of non-negative integer values.  Each
/// integer value defines a unique category of a semantic segmentation.
/// A category is typically used to defined a material (such as void, solid,
/// air, or precipitate) in a 3D computed tomography (CT) scan of a part or
/// assembly.  A 3D scan is composed of a stack of 2D images.
/// Each image is composed of pixels.
/// A stack of images composes a 3D voxel representation.
///
/// The range of integer values is not limited, but a practical example of a
/// range could be `[0, 1, 2, 3, 4]`.  The integers do not need to be sequential,
/// so a range, for example, of `[4, 501, 2]` is valid, but not conventional.
/// The number of rows that compose the SPN file equals the number of voxels
/// used to construct a 3D semantic segmentation.  Axis order (for example,
/// `x`, `y`, then `z`; or, `z`, `y`, `x`, etc.) is not implied by the SPN structure;
/// so additional data, typically provided through a configuration file, is
/// needed to uniquely interpret the pixel tiling and voxel stacking order
/// of the data in the SPN file.
///
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
    /// Constructs and returns a new SPN type from an SPN file.
    pub fn from_spn(file_path: &str, nel: Nel) -> Self {
        let data = spn_data_from_spn(file_path, nel);
        Self { data }
    }
    /// Returns a reference to the internal SPN data.
    pub fn get_data(&self) -> &SpnData {
        &self.data
    }
    #[doc = svgbobdoc::transform!(
    /// Converts the SPN type into a finite element type, consuming the SPN type.
    ///
    /// The SPN data can be scaled and translated (in that order).
    ///
    /// ```math
    /// x \mapsto s_x x + t_x\qquad y \mapsto s_y y + t_y\qquad z \mapsto s_z z + t_z
    /// ```
    ///
    /// ```svgbob
    ///     8       7
    ///      *-------*         z
    ///   5 /|    6 /|          ^
    ///    *-+-----* |          |
    ///    | |4    | |3         |
    ///    | *-----|-*          +-----> x
    ///    |/      |/          /
    ///    *-------*          v
    ///   1       2         -y
    /// ```
    )]
    pub fn into_finite_elements(self, scale: &Scale, translate: &Translate) -> FiniteElements {
        let (element_blocks, element_connectivity, nodal_coordinates) =
            finite_element_data_from_npy_data(self.get_data(), scale, translate);
        FiniteElements::from_data(element_blocks, element_connectivity, nodal_coordinates)
    }
    /// Writes the internal SPN data to a NPY file.
    pub fn write_npy(&self, file_path: &str) {
        write_spn_to_npy(self.get_data(), file_path)
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

fn filter_spn_data(data: &SpnData) -> (VoxelData<3>, ElementBlocks) {
    let filtered_voxel_data_combo: VoxelData<4> = data
        .axis_iter(Axis(2))
        .enumerate()
        .map(|(i, data_i)| {
            data_i
                .axis_iter(Axis(1))
                .enumerate()
                .map(|(j, data_ij)| {
                    data_ij
                        .iter()
                        .enumerate()
                        .filter(|(_, &data_ijk)| data_ijk > 0)
                        .map(|(k, data_ijk)| [i, j, k, *data_ijk as usize])
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
        .map(|entry| [entry[2], entry[1], entry[0]])
        .collect();
    (filtered_voxel_data, element_blocks)
}

fn finite_element_data_from_npy_data(
    data: &SpnData,
    scale: &Scale,
    translate: &Translate,
) -> (ElementBlocks, ElementConnectivity, NodalCoordinates) {
    let shape = data.shape();
    let nelxplus1 = shape[0] + 1;
    let nelyplus1 = shape[1] + 1;
    let xscale = scale[0];
    let yscale = scale[1];
    let zscale = scale[2];
    let xtranslate = translate[0];
    let ytranslate = translate[1];
    let ztranslate = translate[2];
    let (filtered_voxel_data, element_blocks) = filter_spn_data(data);
    let mut element_connectivity: ElementConnectivity = filtered_voxel_data
        .iter()
        .map(|entry| {
            vec![
                entry[0]
                    + entry[1] * nelxplus1
                    + entry[2] * nelxplus1 * nelyplus1
                    + NODE_NUMBERING_OFFSET,
                entry[0]
                    + entry[1] * nelxplus1
                    + entry[2] * nelxplus1 * nelyplus1
                    + 1
                    + NODE_NUMBERING_OFFSET,
                entry[0]
                    + (entry[1] + 1) * nelxplus1
                    + entry[2] * nelxplus1 * nelyplus1
                    + NODE_NUMBERING_OFFSET,
                entry[0]
                    + (entry[1] + 1) * nelxplus1
                    + entry[2] * nelxplus1 * nelyplus1
                    + 1
                    + NODE_NUMBERING_OFFSET,
                entry[0]
                    + entry[1] * nelxplus1
                    + (entry[2] + 1) * nelxplus1 * nelyplus1
                    + NODE_NUMBERING_OFFSET,
                entry[0]
                    + entry[1] * nelxplus1
                    + (entry[2] + 1) * nelxplus1 * nelyplus1
                    + 1
                    + NODE_NUMBERING_OFFSET,
                entry[0]
                    + (entry[1] + 1) * nelxplus1
                    + (entry[2] + 1) * nelxplus1 * nelyplus1
                    + NODE_NUMBERING_OFFSET,
                entry[0]
                    + (entry[1] + 1) * nelxplus1
                    + (entry[2] + 1) * nelxplus1 * nelyplus1
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
                (entry[0] as f64) * xscale + xtranslate,
                (entry[1] as f64) * yscale + ytranslate,
                (entry[2] as f64) * zscale + ztranslate,
            ];
            nodal_coordinates[connectivity[1] - NODE_NUMBERING_OFFSET] = vec![
                (entry[0] as f64 + 1.0) * xscale + xtranslate,
                (entry[1] as f64) * yscale + ytranslate,
                (entry[2] as f64) * zscale + ztranslate,
            ];
            nodal_coordinates[connectivity[2] - NODE_NUMBERING_OFFSET] = vec![
                (entry[0] as f64) * xscale + xtranslate,
                (entry[1] as f64 + 1.0) * yscale + ytranslate,
                (entry[2] as f64) * zscale + ztranslate,
            ];
            nodal_coordinates[connectivity[3] - NODE_NUMBERING_OFFSET] = vec![
                (entry[0] as f64 + 1.0) * xscale + xtranslate,
                (entry[1] as f64 + 1.0) * yscale + ytranslate,
                (entry[2] as f64) * zscale + ztranslate,
            ];
            nodal_coordinates[connectivity[4] - NODE_NUMBERING_OFFSET] = vec![
                (entry[0] as f64) * xscale + xtranslate,
                (entry[1] as f64) * yscale + ytranslate,
                (entry[2] as f64 + 1.0) * zscale + ztranslate,
            ];
            nodal_coordinates[connectivity[5] - NODE_NUMBERING_OFFSET] = vec![
                (entry[0] as f64 + 1.0) * xscale + xtranslate,
                (entry[1] as f64) * yscale + ytranslate,
                (entry[2] as f64 + 1.0) * zscale + ztranslate,
            ];
            nodal_coordinates[connectivity[6] - NODE_NUMBERING_OFFSET] = vec![
                (entry[0] as f64) * xscale + xtranslate,
                (entry[1] as f64 + 1.0) * yscale + ytranslate,
                (entry[2] as f64 + 1.0) * zscale + ztranslate,
            ];
            nodal_coordinates[connectivity[7] - NODE_NUMBERING_OFFSET] = vec![
                (entry[0] as f64 + 1.0) * xscale + xtranslate,
                (entry[1] as f64 + 1.0) * yscale + ytranslate,
                (entry[2] as f64 + 1.0) * zscale + ztranslate,
            ];
        });
    (element_blocks, element_connectivity, nodal_coordinates)
}

fn spn_data_from_npy(file_path: &str) -> SpnData {
    if !file_path.ends_with(".npy") {
        panic!("File type must be .npy.")
    }
    let npy_file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("Could not find the .npy file.")
            }
            _ => {
                // See https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html#errors
                // errors aside from NotFound error. This match arm
                // is difficult to test because it requires setting
                // permissions and getting those permissions to exist
                // in a valid setting for CI/CD. Currently this
                // match arm has no test.
                panic!("Could not open the .npy file.");
            }
        },
    };

    SpnData::read_npy(npy_file).expect("Could not open the .npy file")
}

fn spn_data_from_spn(file_path: &str, nel: Nel) -> SpnData {
    let flat = BufReader::new(File::open(file_path).expect("File was not found."))
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<u8>>();
    let mut data = SpnData::zeros((nel[0], nel[1], nel[2]));
    data.iter_mut()
        .zip(flat.iter())
        .for_each(|(data_entry, flat_entry)| *data_entry = *flat_entry);
    data
}

fn write_spn_to_npy(data: &SpnData, file_path: &str) {
    data.write_npy(BufWriter::new(File::create(file_path).unwrap()))
        .unwrap();
}
