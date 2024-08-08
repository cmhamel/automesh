#[cfg(feature = "python")]
pub mod py;

use super::{abaqus::Abaqus, exodus::Exodus};
use itertools::Itertools;
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
    element_blocks: &ElementBlocks,
    element_connectivity: &ElementConnectivity,
    nodal_coordinates: &NodalCoordinates,
) {
    let inp_file = File::create(file_path).expect("Could not create the .inp file.");
    let mut file = BufWriter::new(inp_file);
    write_heading_to_inp(&mut file);
    write_nodal_coordinates_to_inp(&mut file, nodal_coordinates);
    write_element_connectivity_to_inp(&mut file, element_blocks, element_connectivity);
    file.flush().expect("Forgot to flush!");
}

fn write_heading_to_inp(file: &mut BufWriter<File>) {
    file.write_all(format!("*HEADING\nautomesh {}", env!("CARGO_PKG_VERSION")).as_bytes())
        .unwrap();
    end_section(file);
    file.write_all("*PART, NAME=Part-Default".as_bytes())
        .unwrap();
    end_section(file);
}

fn write_nodal_coordinates_to_inp(
    file: &mut BufWriter<File>,
    nodal_coordinates: &NodalCoordinates,
) {
    file.write_all("*NODE, NSET=ALLNODES".as_bytes()).unwrap();
    nodal_coordinates
        .iter()
        .enumerate()
        .for_each(|(node, coordinates)| {
            indent(file);
            file.write_all((node + 1).to_string().as_bytes()).unwrap();
            coordinates.iter().for_each(|coordinate| {
                delimiter(file);
                file.write_all(format!("{:.6e}", coordinate).as_bytes())
                    .unwrap();
            });
        });
    end_section(file);
}

fn write_element_connectivity_to_inp(
    file: &mut BufWriter<File>,
    element_blocks: &ElementBlocks,
    element_connectivity: &ElementConnectivity,
) {
    let element_type = "C3D8R";
    element_blocks.iter().unique().for_each(|current_block| {
        file.write_all(
            format!("*ELEMENT, TYPE={}, ELSET=EB{}", element_type, current_block).as_bytes(),
        )
        .unwrap();
        element_blocks
            .iter()
            .enumerate()
            .filter(|(_, block)| block == &current_block)
            .for_each(|(element, _)| {
                indent(file);
                file.write_all((element + 1).to_string().as_bytes())
                    .unwrap();
                element_connectivity[element].iter().for_each(|entry| {
                    delimiter(file);
                    file.write_all(entry.to_string().as_bytes()).unwrap();
                });
            });
    });
    end_section(file);
}

fn end_section(file: &mut BufWriter<File>) {
    file.write_all(&[10, 42, 42, 10]).unwrap()
}

fn delimiter(file: &mut BufWriter<File>) {
    file.write_all(&[44, 9]).unwrap()
}

fn indent(file: &mut BufWriter<File>) {
    file.write_all(&[10, 9]).unwrap()
}
