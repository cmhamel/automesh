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
    io::{BufRead, BufReader, ErrorKind},
};

type SpnData = Array3<u8>;
type VoxelData<const N: usize> = Vec<[usize; N]>;

/// The SPN type.
pub struct Spn {
    data: SpnData,
}

/// Inherent implementation of the SPN type.
///
/// # SPN file
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
/// # Examples:
/// The [letter "F" unit test file](../../../tests/input/f.spn), consists of 60 lines, one line
/// for each of the voxels in the `(slice, column, row) -> (z, y, x) -> (4, 5, 3)`  voxel volume.  The interpretation
/// of this data is shown below:
///
/// ![letter F spn file contents annotated](../../../doc/fig/letter_f_spn_annotated.png)
///
/// # See Also:
///
///  * [autotwin/mesh letter "F" unit test](https://github.com/autotwin/mesh/blob/main/doc/npy_to_mesh.md)
///  * [Sculpt Input Microstructure SPN File](https://cubit.sandia.gov/files/cubit/16.10/help_manual/WebHelp/mesh_generation/meshing_schemes/parallel/sculpt_input.htm)
///
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
    let mut element_connectivity: ElementConnectivity = filtered_voxel_data
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
    let number_of_nodes = element_connectivity
        .clone()
        .into_iter()
        .flatten()
        .unique()
        .collect::<Vec<usize>>()
        .len();
    let nodal_coordinates = vec![vec![0.0; 3]; number_of_nodes];
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

/// Given a file path to a `.npy` file, creates a `SpnData` data type.
///
/// # Arguments
///
/// * `file_path` - The fully pathed name of the `.npy` file.
///
/// # Returns
///
/// * The data from the `.npy` file as a `SpnData` type.
///
fn spn_data_from_npy(file_path: &str) -> SpnData {
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
    // The return type of File::open is a Result<T, E> enum.
    // * If `File::open` succeeds, the value in `npy_file_result`` will be an
    //   instance of `Ok` that contains a file handle.
    // * If `File::open` fails, the value in `npy_file_result`` will be an
    //   instance of Err that contains more information about the kind of
    //   error that occurred.
    let npy_file_result = File::open(file_path);

    let npy_file = match npy_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // panic!("File not found: {error:?}");
                panic!(".npy file nonexistent")
            }
            _ => {
                panic!("Problem opening file: {error:?}");
            }
        },
    };

    SpnData::read_npy(npy_file).expect(".npy file unreadable")
}
