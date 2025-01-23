use super::{Blocks, Connectivity, Coordinates, FiniteElements, Nodes, VecConnectivity};
use conspire::math::TensorVec;

/// The number of nodes in a triangular finite element.
pub const NUM_NODES_TRI: usize = 3;

/// The number of nodes a given node is connected to in a triangular finite element.
pub const NODES_CONN_ELEMENT_TRI: usize = 2;

/// The triangular finite elements type.
pub struct TriangularFiniteElements {
    boundary_nodes: Nodes,
    element_blocks: Blocks,
    element_node_connectivity: Connectivity<NUM_NODES_TRI>,
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

impl FiniteElements<NUM_NODES_TRI, NODES_CONN_ELEMENT_TRI> for TriangularFiniteElements {
    fn connected_nodes(node: &usize) -> [usize; NODES_CONN_ELEMENT_TRI] {
        match node {
            0 => [1, 2],
            1 => [0, 2],
            2 => [0, 1],
            _ => panic!(),
        }
    }
    fn from_data(
        element_blocks: Blocks,
        element_node_connectivity: Connectivity<NUM_NODES_TRI>,
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
        unimplemented!()
    }
    fn get_boundary_nodes(&self) -> &Nodes {
        &self.boundary_nodes
    }
    fn get_element_blocks(&self) -> &Blocks {
        &self.element_blocks
    }
    fn get_element_node_connectivity(&self) -> &Connectivity<NUM_NODES_TRI> {
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
        _homogeneous: Option<Nodes>,
        _inhomogeneous: Option<(Coordinates, Nodes)>,
    ) -> Result<(), &str> {
        unimplemented!("Too wet!")
    }
}
