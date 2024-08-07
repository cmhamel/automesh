#[cfg(feature = "python")]
pub mod py;

use super::{abaqus::Abaqus, exodus::Exodus};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub type ElementBlocks = Vec<usize>;
pub type ElementConnectivity = Vec<Vec<usize>>;
pub type NodalCoordinates = Vec<Vec<f64>>;

/// The finite element type.
pub struct FiniteElements {
    element_blocks: ElementBlocks,
    element_connectivity: ElementConnectivity,
    nodal_coordinates: NodalCoordinates,
}

/// Inherent implementation of the finite element type.
impl FiniteElements {
    /// Constructs and returns a new Exodus type from data.
    pub fn from_data(
        element_blocks: ElementBlocks,
        element_connectivity: ElementConnectivity,
        nodal_coordinates: NodalCoordinates,
    ) -> Self {
        Self {
            element_blocks,
            element_connectivity,
            nodal_coordinates,
        }
    }
    /// Returns a reference to the element blocks.
    pub fn get_element_blocks(&self) -> &ElementBlocks {
        &self.element_blocks
    }
    /// Returns a reference to the element connectivity.
    pub fn get_element_connectivity(&self) -> &ElementConnectivity {
        &self.element_connectivity
    }
    /// Returns a reference to the nodal coordinates.
    pub fn get_nodal_coordinates(&self) -> &NodalCoordinates {
        &self.nodal_coordinates
    }
}

/// Abaqus implementation of the finite element type.
impl Abaqus for FiniteElements {
    fn write_inp(&self, file_path: &str) {
        write_fem_to_inp(
            file_path,
            self.get_element_blocks(),
            self.get_element_connectivity(),
            self.get_nodal_coordinates(),
        )
    }
}

/// Exodus implementation of the finite element type.
impl Exodus for FiniteElements {
    fn write_exo(&self, _file_path: &str) {
        todo!("Writing Exodus files has not yet been implemented.")
    }
}

fn write_fem_to_inp(
    file_path: &str,
    _element_blocks: &ElementBlocks,
    element_connectivity: &ElementConnectivity,
    _nodal_coordinates: &NodalCoordinates,
) {
    let mut file = BufWriter::new(File::create(file_path).unwrap());
    element_connectivity
        .iter()
        .enumerate()
        .for_each(|(element, _connectivity)| {
            file.write_all(element.to_string().as_bytes()).unwrap()
        });

    // let test_data = "\nhello\n";
    // file.write(&test_data.as_bytes()).unwrap();
    // file.write(&1_i32.to_string().as_bytes()).unwrap();
    // file.write(&test_data.as_bytes()).unwrap();
    // file.write(&1_i32.to_le_bytes()).unwrap();
    // file.write(&test_data.as_bytes()).unwrap();
    // file.write(&1_i32.to_be_bytes()).unwrap();

    // let number: usize = 1234;
    // let mut file = File::create(file_path).expect("create failed");
    // file.write_all(&number.to_le_bytes()).expect("write failed");

    // let data = "Some data!";
    // std::fs::write("/tmp/foo", data).expect("Unable to write file");

    todo!("Writing Abaqus files has not yet been implemented.")
}
