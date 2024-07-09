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
        let nelz = shape[0];
        let nely = shape[1];
        // let nelx = shape[2];
        // let number_of_elements_with_void = nelz * nely * nelx;
        let nelzplus1 = nelz + 1;
        let nelyplus1 = nely + 1;
        // let nelxplus1 = nelx + 1;
        // let number_of_nodes_with_void = nelxplus1 * nelyplus1 * nelzplus1;

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

        let block_connectivity: Vec<usize> = filtered_data
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

        // filtered_data.iter().flatten().flatten().for_each(|c| println!("{:?}", (c[0], c[1], c[2], c[3])));
        // element_connectivity.iter().for_each(|c| println!("{:?}", (c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7])));

        // let element_connectivity = data
        //     .outer_iter()
        //     .enumerate()
        //     .map(|(k, data_k)|
        //         data_k
        //         .outer_iter()
        //         .enumerate()
        //         .map(|(j, data_kj)|
        //             data_kj
        //             .iter()
        //             .enumerate()
        //             .filter(|(_, &data_kji)| data_kji > 0)
        //             .map(|(i, _)|
        //                 [i * nelzplus1 * nelyplus1 + j * nelzplus1 + k,
        //                 0,
        //                 0,
        //                 0,
        //                 0,
        //                 0,
        //                 0,
        //                 0]
        //             ).collect()
        //         ).collect()
        //     ).collect()
        //     .flatten();

        // let mut element_connectivity_with_void = vec![[0; 8]; number_of_elements_with_void];

        // data
        //     .outer_iter()
        //     .enumerate()
        //     .for_each(|(k, data_k)|
        //         data_k.outer_iter().enumerate().for_each(|(j, data_kj)|
        //             data_kj
        //                 .iter()
        //                 .enumerate()
        //                 .filter(|(_, &data_kji)| data_kji > 0)
        //                 .for_each(
        //                     |(i, data_kji)| println!("{:?}", (k, j, i, data_kji))
        //                 )
        //         )
        //     );

        // cant you filter out the air elements while you do this?
        // just will have weird element numbering, which might be OK

        // let mut nodal_coordinates_with_void = vec![[0.0; 3]; number_of_nodes_with_void];

        // Exodus needs this, right?
        // let block_connectivity: Vec<u8> = data
        //     .iter()
        //     .filter(|&entry| entry > &0)
        //     .copied()
        // .collect();

        // let number_of_elements = block_connectivity.len();

        // Need to filter() out "air" elements
        // let element_connectivity = vec![[0; 8]; number_of_elements];

        // Need to filter() out nodes only belonging to "air" elements
        let nodal_coordinates = vec![[0.0; 3]];
        // data
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, &entry)| entry > 0)
        //     .for_each(|(i, entry)| println!("{:?}", (i, entry)));

        // data
        //     .outer_iter()
        //     .enumerate()
        //     .for_each(|(k, data_k)| {
        //         data_k.outer_iter().enumerate().for_each(|(j, data_kj)| {
        //             data_kj
        //                 .iter()
        //                 .enumerate()
        //                 .filter(|(_, &data_kji)| data_kji > 0)
        //                 .for_each(
        //                     |(i, data_kji)| println!("{:?}", (k, j, i, data_kji)), // every (i, j, k) is origin of a hex
        //                                                                            // then combos of +1 to each give other 7 vertices
        //                                                                            // but can't double-count for nodal coordinates
        //                                                                            // but do need to track for connectivity
        //                                                                            // you should be able to use nel(x,y,z) to get connectivity
        //                                                                            // can zip() get_data().iter() for origin ids
        //                                                                            // but still hard to get back other 7 ids?
        //                                                                            // should you just fill all the nodal coordinates "unmerged"
        //                                                                            // and then just look afterward for coincident nodes (and do connectivity)?
        //                                                                            // Is it better to filter out "air" elements at the end?
        //                                                                            // Is probably easier to do coords/connect with dense cube
        //                                                                            // Then maybe you use nodal connectivity to remove nodal coords only attached to "air" elements?
        //                                                                            // And use block connectivity to remove "air" elements?
        //                                                                            // for large meshes, most nodes in nodal connectivity are connected to 8 hexes
        //                                                                            // so maybe make that sized and use a special number for null entries
        //                 )
        //         })
        //     });

        // Would the nodal connectivity (elements connected to each node) help you here?
        // And would that be worth saving in the struct?
        // What about node sets?

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
