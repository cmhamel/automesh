#[cfg(feature = "python")]
pub mod py;

use super::{
    Blocks, Connectivity, Coordinates, FiniteElements, Nodes, VecConnectivity,
    ELEMENT_NUMBERING_OFFSET, NODE_NUMBERING_OFFSET,
};
use conspire::math::TensorVec;

/// The number of nodes in a hexahedral finite element.
pub const NUM_NODES_HEX: usize = 8;

/// The number of nodes a given node is connected to in a hexahedral finite element.
pub const NODES_CONN_ELEMENT_HEX: usize = 3;

/// The hexahedral finite elements mesh type.
pub struct HexahedralFiniteElements {
    boundary_nodes: Nodes,
    element_blocks: Blocks,
    element_node_connectivity: Connectivity<NUM_NODES_HEX>,
    exterior_nodes: Nodes,
    interface_nodes: Nodes,
    interior_nodes: Nodes,
    nodal_coordinates: Coordinates,
    nodal_influencers: VecConnectivity,
    node_element_connectivity: VecConnectivity,
    node_node_connectivity: VecConnectivity,
    prescribed_nodes: Nodes,
    prescribed_nodes_homogeneous: Nodes,
    prescribed_nodes_inhomogeneous: Nodes,
    prescribed_nodes_inhomogeneous_coordinates: Coordinates,
}

impl FiniteElements<NUM_NODES_HEX, NODES_CONN_ELEMENT_HEX> for HexahedralFiniteElements {
    fn connected_nodes(node: &usize) -> [usize; NODES_CONN_ELEMENT_HEX] {
        match node {
            0 => [1, 3, 4],
            1 => [0, 2, 5],
            2 => [1, 3, 6],
            3 => [0, 2, 7],
            4 => [0, 5, 7],
            5 => [1, 4, 6],
            6 => [2, 5, 7],
            7 => [3, 4, 6],
            _ => panic!(),
        }
    }
    fn from_data(
        element_blocks: Blocks,
        element_node_connectivity: Connectivity<NUM_NODES_HEX>,
        nodal_coordinates: Coordinates,
    ) -> Self {
        Self {
            boundary_nodes: vec![],
            element_blocks,
            element_node_connectivity,
            exterior_nodes: vec![],
            interface_nodes: vec![],
            interior_nodes: vec![],
            nodal_coordinates,
            nodal_influencers: vec![],
            node_element_connectivity: vec![],
            node_node_connectivity: vec![],
            prescribed_nodes: vec![],
            prescribed_nodes_homogeneous: vec![],
            prescribed_nodes_inhomogeneous: vec![],
            prescribed_nodes_inhomogeneous_coordinates: Coordinates::zero(0),
        }
    }
    fn nodal_hierarchy(&mut self) -> Result<(), &str> {
        let node_element_connectivity = self.get_node_element_connectivity();
        if !node_element_connectivity.is_empty() {
            #[cfg(feature = "profile")]
            let time = Instant::now();
            let element_blocks = self.get_element_blocks();
            let mut connected_blocks: Blocks = vec![];
            let mut exterior_nodes = vec![];
            let mut interface_nodes = vec![];
            let mut interior_nodes = vec![];
            let mut number_of_connected_blocks = 0;
            let mut number_of_connected_elements = 0;
            node_element_connectivity
                .iter()
                .enumerate()
                .for_each(|(node, connected_elements)| {
                    connected_blocks = connected_elements
                        .iter()
                        .map(|element| element_blocks[element - ELEMENT_NUMBERING_OFFSET])
                        .collect();
                    connected_blocks.sort();
                    connected_blocks.dedup();
                    number_of_connected_blocks = connected_blocks.len();
                    number_of_connected_elements = connected_elements.len();
                    if number_of_connected_blocks > 1 {
                        interface_nodes.push(node + NODE_NUMBERING_OFFSET);
                        //
                        // THIS IS WHERE IT IS ASSUMED THAT THE MESH IS PERFECTLY STRUCTURED
                        // ONLY AFFECTS HIERARCHICAL SMOOTHING
                        //
                        if number_of_connected_elements < NUM_NODES_HEX {
                            exterior_nodes.push(node + NODE_NUMBERING_OFFSET);
                        }
                    } else if number_of_connected_elements < NUM_NODES_HEX {
                        exterior_nodes.push(node + NODE_NUMBERING_OFFSET);
                    } else {
                        interior_nodes.push(node + NODE_NUMBERING_OFFSET);
                    }
                });
            exterior_nodes.sort();
            interior_nodes.sort();
            interface_nodes.sort();
            self.boundary_nodes = exterior_nodes
                .clone()
                .into_iter()
                .chain(interface_nodes.clone())
                .collect();
            self.boundary_nodes.sort();
            self.boundary_nodes.dedup();
            self.exterior_nodes = exterior_nodes;
            self.interface_nodes = interface_nodes;
            self.interior_nodes = interior_nodes;
            #[cfg(feature = "profile")]
            println!(
                "             \x1b[1;93mNodal hierarchy\x1b[0m {:?} ",
                time.elapsed()
            );
            Ok(())
        } else {
            Err("Need to calculate the node-to-element connectivity first")
        }
    }
    fn get_boundary_nodes(&self) -> &Nodes {
        &self.boundary_nodes
    }
    fn get_element_blocks(&self) -> &Blocks {
        &self.element_blocks
    }
    fn get_element_node_connectivity(&self) -> &Connectivity<NUM_NODES_HEX> {
        &self.element_node_connectivity
    }
    fn get_exterior_nodes(&self) -> &Nodes {
        &self.exterior_nodes
    }
    fn get_interface_nodes(&self) -> &Nodes {
        &self.interface_nodes
    }
    fn get_interior_nodes(&self) -> &Nodes {
        &self.interior_nodes
    }
    fn get_nodal_coordinates(&self) -> &Coordinates {
        &self.nodal_coordinates
    }
    fn get_nodal_coordinates_mut(&mut self) -> &mut Coordinates {
        &mut self.nodal_coordinates
    }
    fn get_nodal_influencers(&self) -> &VecConnectivity {
        &self.nodal_influencers
    }
    fn get_node_element_connectivity(&self) -> &VecConnectivity {
        &self.node_element_connectivity
    }
    fn get_node_node_connectivity(&self) -> &VecConnectivity {
        &self.node_node_connectivity
    }
    fn get_prescribed_nodes(&self) -> &Nodes {
        &self.prescribed_nodes
    }
    fn get_prescribed_nodes_homogeneous(&self) -> &Nodes {
        &self.prescribed_nodes_homogeneous
    }
    fn get_prescribed_nodes_inhomogeneous(&self) -> &Nodes {
        &self.prescribed_nodes_inhomogeneous
    }
    fn get_prescribed_nodes_inhomogeneous_coordinates(&self) -> &Coordinates {
        &self.prescribed_nodes_inhomogeneous_coordinates
    }
    fn set_nodal_influencers(&mut self, nodal_influencers: VecConnectivity) {
        self.nodal_influencers = nodal_influencers
    }
    fn set_node_element_connectivity(&mut self, node_element_connectivity: VecConnectivity) {
        self.node_element_connectivity = node_element_connectivity
    }
    fn set_node_node_connectivity(&mut self, node_node_connectivity: VecConnectivity) {
        self.node_node_connectivity = node_node_connectivity;
    }
    fn set_prescribed_nodes(
        &mut self,
        homogeneous: Option<Nodes>,
        inhomogeneous: Option<(Coordinates, Nodes)>,
    ) -> Result<(), &str> {
        if let Some(homogeneous_nodes) = homogeneous {
            self.prescribed_nodes_homogeneous = homogeneous_nodes;
            self.prescribed_nodes_homogeneous.sort();
            self.prescribed_nodes_homogeneous.dedup();
        }
        if let Some(inhomogeneous_nodes) = inhomogeneous {
            self.prescribed_nodes_inhomogeneous = inhomogeneous_nodes.1;
            self.prescribed_nodes_inhomogeneous_coordinates = inhomogeneous_nodes.0;
            let mut sorted_unique = self.prescribed_nodes_inhomogeneous.clone();
            sorted_unique.sort();
            sorted_unique.dedup();
            if sorted_unique != self.prescribed_nodes_inhomogeneous {
                return Err("Inhomogeneously-prescribed nodes must be sorted and unique.");
            }
        }
        self.prescribed_nodes = self
            .prescribed_nodes_homogeneous
            .clone()
            .into_iter()
            .chain(self.prescribed_nodes_inhomogeneous.clone())
            .collect();
        Ok(())
    }
}
