use super::{
    Blocks, Connectivity, Coordinates, FiniteElements, Nodes, VecConnectivity,
    ELEMENT_NUMBERING_OFFSET, NODE_NUMBERING_OFFSET,
};

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