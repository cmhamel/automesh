#[cfg(feature = "python")]
pub mod py;

#[cfg(test)]
pub mod test;

#[cfg(feature = "profile")]
use std::time::Instant;

use super::{abaqus::Abaqus, ELEMENT_NUMBERING_OFFSET, NODE_NUMBERING_OFFSET};
use chrono::Utc;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufWriter, Error, Write},
};

const ELEMENT_TYPE: &str = "C3D8R";
const ELEMENT_NUM_NODES: usize = 8;

pub type Blocks = Vec<usize>;
pub type Connectivity = Vec<Vec<usize>>;
pub type Coordinates = Vec<Vec<f64>>;
pub type Nodes = Vec<usize>;

/// The finite elements type.
pub struct FiniteElements {
    calculated_nodal_hierarchy: bool,
    calculated_node_element_connectivity: bool,
    calculated_node_node_connectivity: bool,
    element_blocks: Blocks,
    element_node_connectivity: Connectivity,
    exterior_nodes: Nodes,
    interface_nodes: Nodes,
    interior_nodes: Nodes,
    nodal_coordinates: Coordinates,
    node_element_connectivity: Connectivity,
    node_node_connectivity: Connectivity,
}

/// Inherent implementation of the finite elements type.
impl FiniteElements {
    /// Constructs and returns a new finite elements type from data.
    pub fn from_data(
        element_blocks: Blocks,
        element_node_connectivity: Connectivity,
        nodal_coordinates: Coordinates,
    ) -> Self {
        Self {
            calculated_nodal_hierarchy: false,
            calculated_node_element_connectivity: false,
            calculated_node_node_connectivity: false,
            element_blocks,
            element_node_connectivity,
            exterior_nodes: vec![],
            interface_nodes: vec![],
            interior_nodes: vec![],
            nodal_coordinates,
            node_element_connectivity: vec![],
            node_node_connectivity: vec![],
        }
    }
    /// Calculates and sets the nodal hierarchy.
    pub fn calculate_nodal_hierarchy(&mut self) -> Result<(), &str> {
        if self.calculated_nodal_hierarchy {
            Err("Already calculated and set the nodal hierarchy.")
        } else if self.calculated_node_element_connectivity {
            let element_blocks = self.get_element_blocks();
            let mut exterior_nodes_unsorted = vec![];
            let mut interface_nodes_unsorted = vec![];
            let mut interior_nodes_unsorted = vec![];
            self.get_node_element_connectivity()
                .iter()
                .enumerate()
                .for_each(|(node, connected_elements)| {
                    if connected_elements
                        .iter()
                        .map(|element| element_blocks[element - ELEMENT_NUMBERING_OFFSET])
                        .unique()
                        .collect::<Vec<usize>>()
                        .len()
                        > 1
                    {
                        interface_nodes_unsorted.push(node + NODE_NUMBERING_OFFSET);
                    } else if connected_elements.len() == 8 {
                        interior_nodes_unsorted.push(node + NODE_NUMBERING_OFFSET);
                    } else {
                        exterior_nodes_unsorted.push(node + NODE_NUMBERING_OFFSET);
                    }
                });
            self.exterior_nodes = exterior_nodes_unsorted.into_iter().sorted().collect();
            self.interface_nodes = interface_nodes_unsorted.into_iter().sorted().collect();
            self.interior_nodes = interior_nodes_unsorted.into_iter().sorted().collect();
            self.calculated_nodal_hierarchy = true;
            Ok(())
        } else {
            Err("Need to calculate and set the node-to-element connectivity first.")
        }
    }
    /// Calculates and sets the node-to-element connectivity.
    pub fn calculate_node_element_connectivity(&mut self) -> Result<(), &str> {
        if self.calculated_node_element_connectivity {
            Err("Already calculated and set the node-to-element connectivity.")
        } else {
            let element_node_connectivity = self.get_element_node_connectivity();
            let number_of_nodes = self.get_nodal_coordinates().len();
            let mut node_element_connectivity = vec![vec![]; number_of_nodes];
            node_element_connectivity.iter_mut().enumerate().for_each(
                |(node, node_connectivity)| {
                    element_node_connectivity.iter().enumerate().for_each(
                        |(element, element_connectivity)| {
                            if element_connectivity.contains(&(node + NODE_NUMBERING_OFFSET)) {
                                node_connectivity.push(element + ELEMENT_NUMBERING_OFFSET)
                            }
                        },
                    )
                },
            );
            self.node_element_connectivity = node_element_connectivity;
            self.calculated_node_element_connectivity = true;
            Ok(())
        }
    }
    /// Calculates and sets the node-to-node connectivity.
    pub fn calculate_node_node_connectivity(&mut self) -> Result<(), &str> {
        if self.calculated_node_node_connectivity {
            Err("Already calculated and set the node-to-node connectivity.")
        } else if self.calculated_node_element_connectivity {
            let mut element_connectivity = vec![0; ELEMENT_NUM_NODES];
            let element_node_connectivity = self.get_element_node_connectivity();
            let node_element_connectivity = self.get_node_element_connectivity();
            let number_of_nodes = self.get_nodal_coordinates().len();
            let mut node_node_connectivity_nonunique_unsorted = vec![vec![]; number_of_nodes];
            node_node_connectivity_nonunique_unsorted
                .iter_mut()
                .zip(node_element_connectivity.iter().enumerate())
                .try_for_each(|(connectivity, (node, node_connectivity))| {
                    node_connectivity.iter().try_for_each(|element| {
                        element_connectivity.clone_from(
                            &element_node_connectivity[element - ELEMENT_NUMBERING_OFFSET],
                        );
                        match element_connectivity
                            .iter()
                            .position(|&n| n == node + NODE_NUMBERING_OFFSET)
                        {
                            Some(0) => {
                                connectivity.push(element_connectivity[1]);
                                connectivity.push(element_connectivity[3]);
                                connectivity.push(element_connectivity[4]);
                                Ok(())
                            }
                            Some(1) => {
                                connectivity.push(element_connectivity[0]);
                                connectivity.push(element_connectivity[2]);
                                connectivity.push(element_connectivity[5]);
                                Ok(())
                            }
                            Some(2) => {
                                connectivity.push(element_connectivity[1]);
                                connectivity.push(element_connectivity[3]);
                                connectivity.push(element_connectivity[6]);
                                Ok(())
                            }
                            Some(3) => {
                                connectivity.push(element_connectivity[0]);
                                connectivity.push(element_connectivity[2]);
                                connectivity.push(element_connectivity[7]);
                                Ok(())
                            }
                            Some(4) => {
                                connectivity.push(element_connectivity[0]);
                                connectivity.push(element_connectivity[5]);
                                connectivity.push(element_connectivity[7]);
                                Ok(())
                            }
                            Some(5) => {
                                connectivity.push(element_connectivity[1]);
                                connectivity.push(element_connectivity[4]);
                                connectivity.push(element_connectivity[6]);
                                Ok(())
                            }
                            Some(6) => {
                                connectivity.push(element_connectivity[2]);
                                connectivity.push(element_connectivity[5]);
                                connectivity.push(element_connectivity[7]);
                                Ok(())
                            }
                            Some(7) => {
                                connectivity.push(element_connectivity[3]);
                                connectivity.push(element_connectivity[4]);
                                connectivity.push(element_connectivity[6]);
                                Ok(())
                            }
                            Some(8..) => Err(
                                "The element-to-node connectivity has been incorrectly calculated.",
                            ),
                            None => Err(
                                "The node-to-element connectivity has been incorrectly calculated.",
                            ),
                        }
                    })
                })?;
            self.node_node_connectivity = node_node_connectivity_nonunique_unsorted
                .into_iter()
                .map(|connectivity| connectivity.into_iter().unique().sorted().collect())
                .collect();
            self.calculated_node_node_connectivity = true;
            Ok(())
        } else {
            Err("Need to calculate and set the node-to-element connectivity first.")
        }
    }
    /// Returns a reference to the element blocks.
    pub fn get_element_blocks(&self) -> &Blocks {
        &self.element_blocks
    }
    /// Returns a reference to the element-to-node connectivity.
    pub fn get_element_node_connectivity(&self) -> &Connectivity {
        &self.element_node_connectivity
    }
    /// Returns a reference to the exterior nodes.
    pub fn get_exterior_nodes(&self) -> &Nodes {
        &self.exterior_nodes
    }
    /// Returns a reference to the interface nodes.
    pub fn get_interface_nodes(&self) -> &Nodes {
        &self.interface_nodes
    }
    /// Returns a reference to the interior nodes.
    pub fn get_interior_nodes(&self) -> &Nodes {
        &self.interior_nodes
    }
    /// Returns a reference to the nodal coordinates.
    pub fn get_nodal_coordinates(&self) -> &Coordinates {
        &self.nodal_coordinates
    }
    /// Returns a reference to the node-to-element connectivity.
    pub fn get_node_element_connectivity(&self) -> &Connectivity {
        &self.node_element_connectivity
    }
    /// Returns a reference to the node-to-node connectivity.
    pub fn get_node_node_connectivity(&self) -> &Connectivity {
        &self.node_node_connectivity
    }
}

/// Abaqus implementation of the finite elements type.
impl Abaqus for FiniteElements {
    fn write_inp(&self, file_path: &str) -> Result<(), Error> {
        write_fem_to_inp(
            file_path,
            self.get_element_blocks(),
            self.get_element_node_connectivity(),
            self.get_nodal_coordinates(),
        )
    }
}

fn write_fem_to_inp(
    file_path: &str,
    element_blocks: &Blocks,
    element_node_connectivity: &Connectivity,
    nodal_coordinates: &Coordinates,
) -> Result<(), Error> {
    let element_number_width = get_width(element_node_connectivity);
    let node_number_width = get_width(nodal_coordinates);
    let inp_file = File::create(file_path)?;
    let mut file = BufWriter::new(inp_file);
    write_heading_to_inp(&mut file)?;
    write_nodal_coordinates_to_inp(&mut file, nodal_coordinates, &node_number_width)?;
    write_element_node_connectivity_to_inp(
        &mut file,
        element_blocks,
        element_node_connectivity,
        &element_number_width,
        &node_number_width,
    )?;
    file.flush()
}

fn write_heading_to_inp(file: &mut BufWriter<File>) -> Result<(), Error> {
    let heading = format!(
        "*HEADING\nautotwin.automesh\nversion {}\nautogenerated on {}",
        env!("CARGO_PKG_VERSION"),
        Utc::now()
    );
    file.write_all(heading.as_bytes())?;
    end_section(file)
}

fn write_nodal_coordinates_to_inp(
    file: &mut BufWriter<File>,
    nodal_coordinates: &Coordinates,
    node_number_width: &usize,
) -> Result<(), Error> {
    #[cfg(feature = "profile")]
    let time = Instant::now();
    file.write_all("*NODE, NSET=ALLNODES".as_bytes())?;
    nodal_coordinates
        .iter()
        .enumerate()
        .try_for_each(|(node, coordinates)| {
            indent(file)?;
            file.write_all(
                format!(
                    "{:>width$}",
                    node + NODE_NUMBERING_OFFSET,
                    width = node_number_width
                )
                .as_bytes(),
            )?;
            coordinates.iter().try_for_each(|coordinate| {
                delimiter(file)?;
                file.write_all(format!("{:>15.6e}", coordinate).as_bytes())
            })
        })?;
    let result = end_section(file);
    #[cfg(feature = "profile")]
    println!(
        "           \x1b[1;93m⤷ Coordinates\x1b[0m {:?}",
        time.elapsed()
    );
    result
}

fn write_element_node_connectivity_to_inp(
    file: &mut BufWriter<File>,
    element_blocks: &Blocks,
    element_node_connectivity: &Connectivity,
    element_number_width: &usize,
    node_number_width: &usize,
) -> Result<(), Error> {
    #[cfg(feature = "profile")]
    let time = Instant::now();
    let mut unique_element_blocks_iter = element_blocks.iter().unique().sorted();
    unique_element_blocks_iter
        .clone()
        .try_for_each(|current_block| {
            file.write_all(
                format!("*ELEMENT, TYPE={}, ELSET=EB{}", ELEMENT_TYPE, current_block).as_bytes(),
            )?;
            element_blocks
                .iter()
                .enumerate()
                .filter(|(_, block)| block == &current_block)
                .try_for_each(|(element, _)| {
                    indent(file)?;
                    file.write_all(
                        format!(
                            "{:>width$}",
                            element + ELEMENT_NUMBERING_OFFSET,
                            width = element_number_width
                        )
                        .as_bytes(),
                    )?;
                    element_node_connectivity[element]
                        .iter()
                        .try_for_each(|entry| {
                            delimiter(file)?;
                            file.write_all(
                                format!("{:>width$}", entry, width = node_number_width + 3)
                                    .as_bytes(),
                            )
                        })
                })?;
            end_section(file)
        })?;
    let result = unique_element_blocks_iter.try_for_each(|block| {
        file.write_all(
            format!(
                "*SOLID SECTION, ELSET=EB{}, MATERIAL=Default-Steel\n",
                block
            )
            .as_bytes(),
        )
    });
    #[cfg(feature = "profile")]
    println!(
        "             \x1b[1;93mConnectivity\x1b[0m {:?}",
        time.elapsed()
    );
    result
}

fn end_section(file: &mut BufWriter<File>) -> Result<(), Error> {
    file.write_all(&[10, 42, 42, 10])
}

fn delimiter(file: &mut BufWriter<File>) -> Result<(), Error> {
    file.write_all(&[44, 32])
}

fn indent(file: &mut BufWriter<File>) -> Result<(), Error> {
    file.write_all(&[10, 32, 32, 32, 32])
}

fn get_width<T>(input: &[T]) -> usize {
    input.len().to_string().chars().count()
}
