use super::{Blocks, Connectivity, Coordinates, FiniteElements, Nodes};

fn test_finite_elements(
    element_blocks: Blocks,
    element_node_connectivity: Connectivity,
    nodal_coordinates: Coordinates,
    node_element_connectivity_gold: Connectivity,
    node_node_connectivity_gold: Connectivity,
    exterior_nodes_gold: Nodes,
    interface_nodes_gold: Nodes,
    interior_nodes_gold: Nodes,
) {
    let mut finite_elements =
        FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
    assert_eq!(
        finite_elements.calculate_node_node_connectivity(),
        Err("Need to calculate and set the node-to-element connectivity first.")
    );
    assert_eq!(
        finite_elements.calculate_nodal_hierarchy(),
        Err("Need to calculate and set the node-to-element connectivity first.")
    );
    finite_elements
        .calculate_node_element_connectivity()
        .unwrap();
    assert_eq!(
        finite_elements.calculate_node_element_connectivity(),
        Err("Already calculated and set the node-to-element connectivity.")
    );
    finite_elements.calculate_node_node_connectivity().unwrap();
    assert_eq!(
        finite_elements.calculate_node_node_connectivity(),
        Err("Already calculated and set the node-to-node connectivity.")
    );
    finite_elements.calculate_nodal_hierarchy().unwrap();
    assert_eq!(
        finite_elements.calculate_nodal_hierarchy(),
        Err("Already calculated and set the nodal hierarchy.")
    );
    assert_eq!(
        finite_elements.get_node_element_connectivity(),
        &node_element_connectivity_gold
    );
    assert_eq!(
        finite_elements.get_node_node_connectivity(),
        &node_node_connectivity_gold
    );
    assert_eq!(finite_elements.get_exterior_nodes(), &exterior_nodes_gold);
    assert_eq!(finite_elements.get_interface_nodes(), &interface_nodes_gold);
    assert_eq!(finite_elements.get_interior_nodes(), &interior_nodes_gold);
}

#[test]
fn single() {
    let element_blocks = vec![11];
    let element_node_connectivity = vec![vec![1, 2, 4, 3, 5, 6, 8, 7]];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![1]; 8];
    let node_node_connectivity_gold = vec![
        vec![2, 3, 5],
        vec![1, 4, 6],
        vec![1, 4, 7],
        vec![2, 3, 8],
        vec![1, 6, 7],
        vec![2, 5, 8],
        vec![3, 5, 8],
        vec![4, 6, 7],
    ];
    let exterior_nodes_gold = (1..=8).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn double_x() {
    let element_blocks = vec![11; 2];
    let element_node_connectivity = vec![
        vec![1, 2, 5, 4, 7, 8, 11, 10],
        vec![2, 3, 6, 5, 8, 9, 12, 11],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1, 2],
        vec![2],
        vec![1],
        vec![1, 2],
        vec![2],
        vec![1],
        vec![1, 2],
        vec![2],
        vec![1],
        vec![1, 2],
        vec![2],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 4, 7],
        vec![1, 3, 5, 8],
        vec![2, 6, 9],
        vec![1, 5, 10],
        vec![2, 4, 6, 11],
        vec![3, 5, 12],
        vec![1, 8, 10],
        vec![2, 7, 9, 11],
        vec![3, 8, 12],
        vec![4, 7, 11],
        vec![5, 8, 10, 12],
        vec![6, 9, 11],
    ];
    let exterior_nodes_gold = (1..=12).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn double_y() {
    let element_blocks = vec![11; 2];
    let element_node_connectivity = vec![
        vec![1, 2, 4, 3, 7, 8, 10, 9],
        vec![3, 4, 6, 5, 9, 10, 12, 11],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![1.0, 2.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![0.0, 2.0, 1.0],
        vec![1.0, 2.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1],
        vec![1, 2],
        vec![1, 2],
        vec![2],
        vec![2],
        vec![1],
        vec![1],
        vec![1, 2],
        vec![1, 2],
        vec![2],
        vec![2],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 3, 7],
        vec![1, 4, 8],
        vec![1, 4, 5, 9],
        vec![2, 3, 6, 10],
        vec![3, 6, 11],
        vec![4, 5, 12],
        vec![1, 8, 9],
        vec![2, 7, 10],
        vec![3, 7, 10, 11],
        vec![4, 8, 9, 12],
        vec![5, 9, 12],
        vec![6, 10, 11],
    ];
    let exterior_nodes_gold = (1..=12).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn triple() {
    let element_blocks = vec![11; 3];
    let element_node_connectivity = vec![
        vec![1, 2, 6, 5, 9, 10, 14, 13],
        vec![2, 3, 7, 6, 10, 11, 15, 14],
        vec![3, 4, 8, 7, 11, 12, 16, 15],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3],
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3],
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3],
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 5, 9],
        vec![1, 3, 6, 10],
        vec![2, 4, 7, 11],
        vec![3, 8, 12],
        vec![1, 6, 13],
        vec![2, 5, 7, 14],
        vec![3, 6, 8, 15],
        vec![4, 7, 16],
        vec![1, 10, 13],
        vec![2, 9, 11, 14],
        vec![3, 10, 12, 15],
        vec![4, 11, 16],
        vec![5, 9, 14],
        vec![6, 10, 13, 15],
        vec![7, 11, 14, 16],
        vec![8, 12, 15],
    ];
    let exterior_nodes_gold = (1..=16).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple() {
    let element_blocks = vec![11; 4];
    let element_node_connectivity = vec![
        vec![1, 2, 7, 6, 11, 12, 17, 16],
        vec![2, 3, 8, 7, 12, 13, 18, 17],
        vec![3, 4, 9, 8, 13, 14, 19, 18],
        vec![4, 5, 10, 9, 14, 15, 20, 19],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4],
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4],
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4],
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 6, 11],
        vec![1, 3, 7, 12],
        vec![2, 4, 8, 13],
        vec![3, 5, 9, 14],
        vec![4, 10, 15],
        vec![1, 7, 16],
        vec![2, 6, 8, 17],
        vec![3, 7, 9, 18],
        vec![4, 8, 10, 19],
        vec![5, 9, 20],
        vec![1, 12, 16],
        vec![2, 11, 13, 17],
        vec![3, 12, 14, 18],
        vec![4, 13, 15, 19],
        vec![5, 14, 20],
        vec![6, 11, 17],
        vec![7, 12, 16, 18],
        vec![8, 13, 17, 19],
        vec![9, 14, 18, 20],
        vec![10, 15, 19],
    ];
    let exterior_nodes_gold = (1..=20).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_voids() {
    let element_blocks = vec![99; 2];
    let element_node_connectivity = vec![
        vec![1, 2, 6, 5, 9, 10, 14, 13],
        vec![3, 4, 8, 7, 11, 12, 16, 15],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks() {
    let element_blocks = vec![11, 21, 21, 11];
    let element_node_connectivity = vec![
        vec![1, 2, 7, 6, 11, 12, 17, 16],
        vec![2, 3, 8, 7, 12, 13, 18, 17],
        vec![3, 4, 9, 8, 13, 14, 19, 18],
        vec![4, 5, 10, 9, 14, 15, 20, 19],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks_remove_1() {
    let element_blocks = vec![21; 2];
    let element_node_connectivity = vec![
        vec![1, 2, 5, 4, 7, 8, 11, 10],
        vec![2, 3, 6, 5, 8, 9, 12, 11],
    ];
    let nodal_coordinates = vec![
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks_remove_2() {
    let element_blocks = vec![11; 2];
    let element_node_connectivity = vec![
        vec![1, 2, 6, 5, 9, 10, 14, 13],
        vec![3, 4, 8, 7, 11, 12, 16, 15],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks_void() {
    let element_blocks = vec![11, 21, 11];
    let element_node_connectivity = vec![
        vec![1, 2, 7, 6, 11, 12, 17, 16],
        vec![2, 3, 8, 7, 12, 13, 18, 17],
        vec![4, 5, 10, 9, 14, 15, 20, 19],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks_void_remove_0() {
    let element_blocks = vec![11, 21, 11];
    let element_node_connectivity = vec![
        vec![1, 2, 7, 6, 11, 12, 17, 16],
        vec![2, 3, 8, 7, 12, 13, 18, 17],
        vec![4, 5, 10, 9, 14, 15, 20, 19],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks_void_remove_1() {
    let element_blocks = vec![21];
    let element_node_connectivity = vec![vec![1, 2, 4, 3, 5, 6, 8, 7]];
    let nodal_coordinates = vec![
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks_void_remove_2() {
    let element_blocks = vec![11; 2];
    let element_node_connectivity = vec![
        vec![1, 2, 6, 5, 9, 10, 14, 13],
        vec![3, 4, 8, 7, 11, 12, 16, 15],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn quadruple_2_blocks_void_remove_3() {
    let element_blocks = vec![0];
    let element_node_connectivity = vec![vec![1, 2, 4, 3, 5, 6, 8, 7]];
    let nodal_coordinates = vec![
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn cube() {
    let element_blocks = vec![11; 9];
    let element_node_connectivity = vec![
        vec![1, 2, 5, 4, 10, 11, 14, 13],
        vec![2, 3, 6, 5, 11, 12, 15, 14],
        vec![4, 5, 8, 7, 13, 14, 17, 16],
        vec![5, 6, 9, 8, 14, 15, 18, 17],
        vec![10, 11, 14, 13, 19, 20, 23, 22],
        vec![11, 12, 15, 14, 20, 21, 24, 23],
        vec![13, 14, 17, 16, 22, 23, 26, 25],
        vec![14, 15, 18, 17, 23, 24, 27, 26],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![1.0, 2.0, 0.0],
        vec![2.0, 2.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 2.0, 1.0],
        vec![1.0, 2.0, 1.0],
        vec![2.0, 2.0, 1.0],
        vec![0.0, 0.0, 2.0],
        vec![1.0, 0.0, 2.0],
        vec![2.0, 0.0, 2.0],
        vec![0.0, 1.0, 2.0],
        vec![1.0, 1.0, 2.0],
        vec![2.0, 1.0, 2.0],
        vec![0.0, 2.0, 2.0],
        vec![1.0, 2.0, 2.0],
        vec![2.0, 2.0, 2.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1, 2],
        vec![2],
        vec![1, 3],
        vec![1, 2, 3, 4],
        vec![2, 4],
        vec![3],
        vec![3, 4],
        vec![4],
        vec![1, 5],
        vec![1, 2, 5, 6],
        vec![2, 6],
        vec![1, 3, 5, 7],
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![2, 4, 6, 8],
        vec![3, 7],
        vec![3, 4, 7, 8],
        vec![4, 8],
        vec![5],
        vec![5, 6],
        vec![6],
        vec![5, 7],
        vec![5, 6, 7, 8],
        vec![6, 8],
        vec![7],
        vec![7, 8],
        vec![8],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 4, 10],
        vec![1, 3, 5, 11],
        vec![2, 6, 12],
        vec![1, 5, 7, 13],
        vec![2, 4, 6, 8, 14],
        vec![3, 5, 9, 15],
        vec![4, 8, 16],
        vec![5, 7, 9, 17],
        vec![6, 8, 18],
        vec![1, 11, 13, 19],
        vec![2, 10, 12, 14, 20],
        vec![3, 11, 15, 21],
        vec![4, 10, 14, 16, 22],
        vec![5, 11, 13, 15, 17, 23],
        vec![6, 12, 14, 18, 24],
        vec![7, 13, 17, 25],
        vec![8, 14, 16, 18, 26],
        vec![9, 15, 17, 27],
        vec![10, 20, 22],
        vec![11, 19, 21, 23],
        vec![12, 20, 24],
        vec![13, 19, 23, 25],
        vec![14, 20, 22, 24, 26],
        vec![15, 21, 23, 27],
        vec![16, 22, 26],
        vec![17, 23, 25, 27],
        vec![18, 24, 26],
    ];
    let mut exterior_nodes_gold = (1..=27).collect::<Nodes>();
    exterior_nodes_gold.remove(13);
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![14];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn cube_multi() {
    let element_blocks = vec![82, 2, 2, 2, 31, 44];
    let element_node_connectivity = vec![
        vec![1, 2, 5, 4, 10, 11, 14, 13],
        vec![2, 3, 6, 5, 11, 12, 15, 14],
        vec![4, 5, 8, 7, 13, 14, 17, 16],
        vec![5, 6, 9, 8, 14, 15, 18, 17],
        vec![11, 12, 15, 14, 19, 20, 22, 21],
        vec![14, 15, 18, 17, 21, 22, 24, 23],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![1.0, 2.0, 0.0],
        vec![2.0, 2.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 2.0, 1.0],
        vec![1.0, 2.0, 1.0],
        vec![2.0, 2.0, 1.0],
        vec![1.0, 0.0, 2.0],
        vec![2.0, 0.0, 2.0],
        vec![1.0, 1.0, 2.0],
        vec![2.0, 1.0, 2.0],
        vec![1.0, 2.0, 2.0],
        vec![2.0, 2.0, 2.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1, 2],
        vec![2],
        vec![1, 3],
        vec![1, 2, 3, 4],
        vec![2, 4],
        vec![3],
        vec![3, 4],
        vec![4],
        vec![1],
        vec![1, 2, 5],
        vec![2, 5],
        vec![1, 3],
        vec![1, 2, 3, 4, 5, 6],
        vec![2, 4, 5, 6],
        vec![3],
        vec![3, 4, 6],
        vec![4, 6],
        vec![5],
        vec![5],
        vec![5, 6],
        vec![5, 6],
        vec![6],
        vec![6],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 4, 10],
        vec![1, 3, 5, 11],
        vec![2, 6, 12],
        vec![1, 5, 7, 13],
        vec![2, 4, 6, 8, 14],
        vec![3, 5, 9, 15],
        vec![4, 8, 16],
        vec![5, 7, 9, 17],
        vec![6, 8, 18],
        vec![1, 11, 13],
        vec![2, 10, 12, 14, 19],
        vec![3, 11, 15, 20],
        vec![4, 10, 14, 16],
        vec![5, 11, 13, 15, 17, 21],
        vec![6, 12, 14, 18, 22],
        vec![7, 13, 17],
        vec![8, 14, 16, 18, 23],
        vec![9, 15, 17, 24],
        vec![11, 20, 21],
        vec![12, 19, 22],
        vec![14, 19, 22, 23],
        vec![15, 20, 21, 24],
        vec![17, 21, 24],
        vec![18, 22, 23],
    ];
    let exterior_nodes_gold = vec![1, 3, 6, 7, 8, 9, 10, 16, 19, 20, 23, 24];
    let interface_nodes_gold = vec![2, 4, 5, 11, 12, 13, 14, 15, 17, 18, 21, 22];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}
#[test]
fn letter_f() {
    let element_blocks = vec![11; 8];
    let element_node_connectivity = vec![
        vec![1, 2, 4, 3, 19, 20, 22, 21],
        vec![3, 4, 6, 5, 21, 22, 24, 23],
        vec![5, 6, 9, 8, 23, 24, 27, 26],
        vec![6, 7, 10, 9, 24, 25, 28, 27],
        vec![8, 9, 12, 11, 26, 27, 30, 29],
        vec![11, 12, 16, 15, 29, 30, 34, 33],
        vec![12, 13, 17, 16, 30, 31, 35, 34],
        vec![13, 14, 18, 17, 31, 32, 36, 35],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![1.0, 2.0, 0.0],
        vec![2.0, 2.0, 0.0],
        vec![0.0, 3.0, 0.0],
        vec![1.0, 3.0, 0.0],
        vec![2.0, 3.0, 0.0],
        vec![0.0, 4.0, 0.0],
        vec![1.0, 4.0, 0.0],
        vec![2.0, 4.0, 0.0],
        vec![3.0, 4.0, 0.0],
        vec![0.0, 5.0, 0.0],
        vec![1.0, 5.0, 0.0],
        vec![2.0, 5.0, 0.0],
        vec![3.0, 5.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![0.0, 2.0, 1.0],
        vec![1.0, 2.0, 1.0],
        vec![2.0, 2.0, 1.0],
        vec![0.0, 3.0, 1.0],
        vec![1.0, 3.0, 1.0],
        vec![2.0, 3.0, 1.0],
        vec![0.0, 4.0, 1.0],
        vec![1.0, 4.0, 1.0],
        vec![2.0, 4.0, 1.0],
        vec![3.0, 4.0, 1.0],
        vec![0.0, 5.0, 1.0],
        vec![1.0, 5.0, 1.0],
        vec![2.0, 5.0, 1.0],
        vec![3.0, 5.0, 1.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1],
        vec![1, 2],
        vec![1, 2],
        vec![2, 3],
        vec![2, 3, 4],
        vec![4],
        vec![3, 5],
        vec![3, 4, 5],
        vec![4],
        vec![5, 6],
        vec![5, 6, 7],
        vec![7, 8],
        vec![8],
        vec![6],
        vec![6, 7],
        vec![7, 8],
        vec![8],
        vec![1],
        vec![1],
        vec![1, 2],
        vec![1, 2],
        vec![2, 3],
        vec![2, 3, 4],
        vec![4],
        vec![3, 5],
        vec![3, 4, 5],
        vec![4],
        vec![5, 6],
        vec![5, 6, 7],
        vec![7, 8],
        vec![8],
        vec![6],
        vec![6, 7],
        vec![7, 8],
        vec![8],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 3, 19],
        vec![1, 4, 20],
        vec![1, 4, 5, 21],
        vec![2, 3, 6, 22],
        vec![3, 6, 8, 23],
        vec![4, 5, 7, 9, 24],
        vec![6, 10, 25],
        vec![5, 9, 11, 26],
        vec![6, 8, 10, 12, 27],
        vec![7, 9, 28],
        vec![8, 12, 15, 29],
        vec![9, 11, 13, 16, 30],
        vec![12, 14, 17, 31],
        vec![13, 18, 32],
        vec![11, 16, 33],
        vec![12, 15, 17, 34],
        vec![13, 16, 18, 35],
        vec![14, 17, 36],
        vec![1, 20, 21],
        vec![2, 19, 22],
        vec![3, 19, 22, 23],
        vec![4, 20, 21, 24],
        vec![5, 21, 24, 26],
        vec![6, 22, 23, 25, 27],
        vec![7, 24, 28],
        vec![8, 23, 27, 29],
        vec![9, 24, 26, 28, 30],
        vec![10, 25, 27],
        vec![11, 26, 30, 33],
        vec![12, 27, 29, 31, 34],
        vec![13, 30, 32, 35],
        vec![14, 31, 36],
        vec![15, 29, 34],
        vec![16, 30, 33, 35],
        vec![17, 31, 34, 36],
        vec![18, 32, 35],
    ];
    let exterior_nodes_gold = (1..=36).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}
#[test]
fn letter_f_3d() {
    let element_blocks = vec![11; 39];
    let element_node_connectivity = vec![
        vec![1, 2, 7, 6, 31, 32, 37, 36],
        vec![2, 3, 8, 7, 32, 33, 38, 37],
        vec![3, 4, 9, 8, 33, 34, 39, 38],
        vec![4, 5, 10, 9, 34, 35, 40, 39],
        vec![6, 7, 12, 11, 36, 37, 42, 41],
        vec![7, 8, 13, 12, 37, 38, 43, 42],
        vec![8, 9, 14, 13, 38, 39, 44, 43],
        vec![9, 10, 15, 14, 39, 40, 45, 44],
        vec![11, 12, 17, 16, 41, 42, 47, 46],
        vec![12, 13, 18, 17, 42, 43, 48, 47],
        vec![13, 14, 19, 18, 43, 44, 49, 48],
        vec![14, 15, 20, 19, 44, 45, 50, 49],
        vec![16, 17, 22, 21, 46, 47, 52, 51],
        vec![17, 18, 23, 22, 47, 48, 53, 52],
        vec![18, 19, 24, 23, 48, 49, 54, 53],
        vec![19, 20, 25, 24, 49, 50, 55, 54],
        vec![21, 22, 27, 26, 51, 52, 57, 56],
        vec![22, 23, 28, 27, 52, 53, 58, 57],
        vec![23, 24, 29, 28, 53, 54, 59, 58],
        vec![24, 25, 30, 29, 54, 55, 60, 59],
        vec![31, 32, 37, 36, 61, 62, 64, 63],
        vec![36, 37, 42, 41, 63, 64, 66, 65],
        vec![41, 42, 47, 46, 65, 66, 71, 70],
        vec![42, 43, 48, 47, 66, 67, 72, 71],
        vec![43, 44, 49, 48, 67, 68, 73, 72],
        vec![44, 45, 50, 49, 68, 69, 74, 73],
        vec![46, 47, 52, 51, 70, 71, 76, 75],
        vec![51, 52, 57, 56, 75, 76, 81, 80],
        vec![52, 53, 58, 57, 76, 77, 82, 81],
        vec![53, 54, 59, 58, 77, 78, 83, 82],
        vec![54, 55, 60, 59, 78, 79, 84, 83],
        vec![61, 62, 64, 63, 85, 86, 88, 87],
        vec![63, 64, 66, 65, 87, 88, 90, 89],
        vec![65, 66, 71, 70, 89, 90, 92, 91],
        vec![70, 71, 76, 75, 91, 92, 94, 93],
        vec![75, 76, 81, 80, 93, 94, 99, 98],
        vec![76, 77, 82, 81, 94, 95, 100, 99],
        vec![77, 78, 83, 82, 95, 96, 101, 100],
        vec![78, 79, 84, 83, 96, 97, 102, 101],
    ];
    let nodal_coordinates = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![3.0, 0.0, 0.0],
        vec![4.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![1.0, 2.0, 0.0],
        vec![2.0, 2.0, 0.0],
        vec![3.0, 2.0, 0.0],
        vec![4.0, 2.0, 0.0],
        vec![0.0, 3.0, 0.0],
        vec![1.0, 3.0, 0.0],
        vec![2.0, 3.0, 0.0],
        vec![3.0, 3.0, 0.0],
        vec![4.0, 3.0, 0.0],
        vec![0.0, 4.0, 0.0],
        vec![1.0, 4.0, 0.0],
        vec![2.0, 4.0, 0.0],
        vec![3.0, 4.0, 0.0],
        vec![4.0, 4.0, 0.0],
        vec![0.0, 5.0, 0.0],
        vec![1.0, 5.0, 0.0],
        vec![2.0, 5.0, 0.0],
        vec![3.0, 5.0, 0.0],
        vec![4.0, 5.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
        vec![0.0, 2.0, 1.0],
        vec![1.0, 2.0, 1.0],
        vec![2.0, 2.0, 1.0],
        vec![3.0, 2.0, 1.0],
        vec![4.0, 2.0, 1.0],
        vec![0.0, 3.0, 1.0],
        vec![1.0, 3.0, 1.0],
        vec![2.0, 3.0, 1.0],
        vec![3.0, 3.0, 1.0],
        vec![4.0, 3.0, 1.0],
        vec![0.0, 4.0, 1.0],
        vec![1.0, 4.0, 1.0],
        vec![2.0, 4.0, 1.0],
        vec![3.0, 4.0, 1.0],
        vec![4.0, 4.0, 1.0],
        vec![0.0, 5.0, 1.0],
        vec![1.0, 5.0, 1.0],
        vec![2.0, 5.0, 1.0],
        vec![3.0, 5.0, 1.0],
        vec![4.0, 5.0, 1.0],
        vec![0.0, 0.0, 2.0],
        vec![1.0, 0.0, 2.0],
        vec![0.0, 1.0, 2.0],
        vec![1.0, 1.0, 2.0],
        vec![0.0, 2.0, 2.0],
        vec![1.0, 2.0, 2.0],
        vec![2.0, 2.0, 2.0],
        vec![3.0, 2.0, 2.0],
        vec![4.0, 2.0, 2.0],
        vec![0.0, 3.0, 2.0],
        vec![1.0, 3.0, 2.0],
        vec![2.0, 3.0, 2.0],
        vec![3.0, 3.0, 2.0],
        vec![4.0, 3.0, 2.0],
        vec![0.0, 4.0, 2.0],
        vec![1.0, 4.0, 2.0],
        vec![2.0, 4.0, 2.0],
        vec![3.0, 4.0, 2.0],
        vec![4.0, 4.0, 2.0],
        vec![0.0, 5.0, 2.0],
        vec![1.0, 5.0, 2.0],
        vec![2.0, 5.0, 2.0],
        vec![3.0, 5.0, 2.0],
        vec![4.0, 5.0, 2.0],
        vec![0.0, 0.0, 3.0],
        vec![1.0, 0.0, 3.0],
        vec![0.0, 1.0, 3.0],
        vec![1.0, 1.0, 3.0],
        vec![0.0, 2.0, 3.0],
        vec![1.0, 2.0, 3.0],
        vec![0.0, 3.0, 3.0],
        vec![1.0, 3.0, 3.0],
        vec![0.0, 4.0, 3.0],
        vec![1.0, 4.0, 3.0],
        vec![2.0, 4.0, 3.0],
        vec![3.0, 4.0, 3.0],
        vec![4.0, 4.0, 3.0],
        vec![0.0, 5.0, 3.0],
        vec![1.0, 5.0, 3.0],
        vec![2.0, 5.0, 3.0],
        vec![3.0, 5.0, 3.0],
        vec![4.0, 5.0, 3.0],
    ];
    let node_element_connectivity_gold = vec![
        vec![1],
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4],
        vec![1, 5],
        vec![1, 2, 5, 6],
        vec![2, 3, 6, 7],
        vec![3, 4, 7, 8],
        vec![4, 8],
        vec![5, 9],
        vec![5, 6, 9, 10],
        vec![6, 7, 10, 11],
        vec![7, 8, 11, 12],
        vec![8, 12],
        vec![9, 13],
        vec![9, 10, 13, 14],
        vec![10, 11, 14, 15],
        vec![11, 12, 15, 16],
        vec![12, 16],
        vec![13, 17],
        vec![13, 14, 17, 18],
        vec![14, 15, 18, 19],
        vec![15, 16, 19, 20],
        vec![16, 20],
        vec![17],
        vec![17, 18],
        vec![18, 19],
        vec![19, 20],
        vec![20],
        vec![1, 21],
        vec![1, 2, 21],
        vec![2, 3],
        vec![3, 4],
        vec![4],
        vec![1, 5, 21, 22],
        vec![1, 2, 5, 6, 21, 22],
        vec![2, 3, 6, 7],
        vec![3, 4, 7, 8],
        vec![4, 8],
        vec![5, 9, 22, 23],
        vec![5, 6, 9, 10, 22, 23, 24],
        vec![6, 7, 10, 11, 24, 25],
        vec![7, 8, 11, 12, 25, 26],
        vec![8, 12, 26],
        vec![9, 13, 23, 27],
        vec![9, 10, 13, 14, 23, 24, 27],
        vec![10, 11, 14, 15, 24, 25],
        vec![11, 12, 15, 16, 25, 26],
        vec![12, 16, 26],
        vec![13, 17, 27, 28],
        vec![13, 14, 17, 18, 27, 28, 29],
        vec![14, 15, 18, 19, 29, 30],
        vec![15, 16, 19, 20, 30, 31],
        vec![16, 20, 31],
        vec![17, 28],
        vec![17, 18, 28, 29],
        vec![18, 19, 29, 30],
        vec![19, 20, 30, 31],
        vec![20, 31],
        vec![21, 32],
        vec![21, 32],
        vec![21, 22, 32, 33],
        vec![21, 22, 32, 33],
        vec![22, 23, 33, 34],
        vec![22, 23, 24, 33, 34],
        vec![24, 25],
        vec![25, 26],
        vec![26],
        vec![23, 27, 34, 35],
        vec![23, 24, 27, 34, 35],
        vec![24, 25],
        vec![25, 26],
        vec![26],
        vec![27, 28, 35, 36],
        vec![27, 28, 29, 35, 36, 37],
        vec![29, 30, 37, 38],
        vec![30, 31, 38, 39],
        vec![31, 39],
        vec![28, 36],
        vec![28, 29, 36, 37],
        vec![29, 30, 37, 38],
        vec![30, 31, 38, 39],
        vec![31, 39],
        vec![32],
        vec![32],
        vec![32, 33],
        vec![32, 33],
        vec![33, 34],
        vec![33, 34],
        vec![34, 35],
        vec![34, 35],
        vec![35, 36],
        vec![35, 36, 37],
        vec![37, 38],
        vec![38, 39],
        vec![39],
        vec![36],
        vec![36, 37],
        vec![37, 38],
        vec![38, 39],
        vec![39],
    ];
    let node_node_connectivity_gold = vec![
        vec![2, 6, 31],
        vec![1, 3, 7, 32],
        vec![2, 4, 8, 33],
        vec![3, 5, 9, 34],
        vec![4, 10, 35],
        vec![1, 7, 11, 36],
        vec![2, 6, 8, 12, 37],
        vec![3, 7, 9, 13, 38],
        vec![4, 8, 10, 14, 39],
        vec![5, 9, 15, 40],
        vec![6, 12, 16, 41],
        vec![7, 11, 13, 17, 42],
        vec![8, 12, 14, 18, 43],
        vec![9, 13, 15, 19, 44],
        vec![10, 14, 20, 45],
        vec![11, 17, 21, 46],
        vec![12, 16, 18, 22, 47],
        vec![13, 17, 19, 23, 48],
        vec![14, 18, 20, 24, 49],
        vec![15, 19, 25, 50],
        vec![16, 22, 26, 51],
        vec![17, 21, 23, 27, 52],
        vec![18, 22, 24, 28, 53],
        vec![19, 23, 25, 29, 54],
        vec![20, 24, 30, 55],
        vec![21, 27, 56],
        vec![22, 26, 28, 57],
        vec![23, 27, 29, 58],
        vec![24, 28, 30, 59],
        vec![25, 29, 60],
        vec![1, 32, 36, 61],
        vec![2, 31, 33, 37, 62],
        vec![3, 32, 34, 38],
        vec![4, 33, 35, 39],
        vec![5, 34, 40],
        vec![6, 31, 37, 41, 63],
        vec![7, 32, 36, 38, 42, 64],
        vec![8, 33, 37, 39, 43],
        vec![9, 34, 38, 40, 44],
        vec![10, 35, 39, 45],
        vec![11, 36, 42, 46, 65],
        vec![12, 37, 41, 43, 47, 66],
        vec![13, 38, 42, 44, 48, 67],
        vec![14, 39, 43, 45, 49, 68],
        vec![15, 40, 44, 50, 69],
        vec![16, 41, 47, 51, 70],
        vec![17, 42, 46, 48, 52, 71],
        vec![18, 43, 47, 49, 53, 72],
        vec![19, 44, 48, 50, 54, 73],
        vec![20, 45, 49, 55, 74],
        vec![21, 46, 52, 56, 75],
        vec![22, 47, 51, 53, 57, 76],
        vec![23, 48, 52, 54, 58, 77],
        vec![24, 49, 53, 55, 59, 78],
        vec![25, 50, 54, 60, 79],
        vec![26, 51, 57, 80],
        vec![27, 52, 56, 58, 81],
        vec![28, 53, 57, 59, 82],
        vec![29, 54, 58, 60, 83],
        vec![30, 55, 59, 84],
        vec![31, 62, 63, 85],
        vec![32, 61, 64, 86],
        vec![36, 61, 64, 65, 87],
        vec![37, 62, 63, 66, 88],
        vec![41, 63, 66, 70, 89],
        vec![42, 64, 65, 67, 71, 90],
        vec![43, 66, 68, 72],
        vec![44, 67, 69, 73],
        vec![45, 68, 74],
        vec![46, 65, 71, 75, 91],
        vec![47, 66, 70, 72, 76, 92],
        vec![48, 67, 71, 73],
        vec![49, 68, 72, 74],
        vec![50, 69, 73],
        vec![51, 70, 76, 80, 93],
        vec![52, 71, 75, 77, 81, 94],
        vec![53, 76, 78, 82, 95],
        vec![54, 77, 79, 83, 96],
        vec![55, 78, 84, 97],
        vec![56, 75, 81, 98],
        vec![57, 76, 80, 82, 99],
        vec![58, 77, 81, 83, 100],
        vec![59, 78, 82, 84, 101],
        vec![60, 79, 83, 102],
        vec![61, 86, 87],
        vec![62, 85, 88],
        vec![63, 85, 88, 89],
        vec![64, 86, 87, 90],
        vec![65, 87, 90, 91],
        vec![66, 88, 89, 92],
        vec![70, 89, 92, 93],
        vec![71, 90, 91, 94],
        vec![75, 91, 94, 98],
        vec![76, 92, 93, 95, 99],
        vec![77, 94, 96, 100],
        vec![78, 95, 97, 101],
        vec![79, 96, 102],
        vec![80, 93, 99],
        vec![81, 94, 98, 100],
        vec![82, 95, 99, 101],
        vec![83, 96, 100, 102],
        vec![84, 97, 101],
    ];
    let exterior_nodes_gold = (1..=102).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}

#[test]
fn sparse() {
    let element_blocks = vec![
        2, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1,
        2, 1, 1, 1, 2, 2, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 1, 1, 1, 2, 1,
    ];
    let element_node_connectivity = vec![
        vec![1, 2, 4, 3, 29, 30, 36, 35],
        vec![3, 4, 10, 9, 35, 36, 42, 41],
        vec![5, 6, 12, 11, 37, 38, 44, 43],
        vec![6, 7, 13, 12, 38, 39, 45, 44],
        vec![8, 9, 15, 14, 40, 41, 47, 46],
        vec![9, 10, 16, 15, 41, 42, 48, 47],
        vec![11, 12, 18, 17, 43, 44, 50, 49],
        vec![15, 16, 22, 21, 47, 48, 54, 53],
        vec![17, 18, 24, 23, 49, 50, 56, 55],
        vec![18, 19, 25, 24, 50, 51, 57, 56],
        vec![20, 21, 27, 26, 52, 53, 59, 58],
        vec![21, 22, 28, 27, 53, 54, 60, 59],
        vec![31, 32, 38, 37, 64, 65, 71, 70],
        vec![32, 33, 39, 38, 65, 66, 72, 71],
        vec![34, 35, 41, 40, 67, 68, 74, 73],
        vec![35, 36, 42, 41, 68, 69, 75, 74],
        vec![40, 41, 47, 46, 73, 74, 80, 79],
        vec![43, 44, 50, 49, 76, 77, 83, 82],
        vec![44, 45, 51, 50, 77, 78, 84, 83],
        vec![46, 47, 53, 52, 79, 80, 86, 85],
        vec![49, 50, 56, 55, 82, 83, 89, 88],
        vec![54, 55, 61, 60, 87, 88, 93, 92],
        vec![62, 63, 69, 68, 96, 97, 102, 101],
        vec![63, 64, 70, 69, 97, 98, 103, 102],
        vec![64, 65, 71, 70, 98, 99, 104, 103],
        vec![70, 71, 77, 76, 103, 104, 110, 109],
        vec![75, 76, 82, 81, 108, 109, 114, 113],
        vec![76, 77, 83, 82, 109, 110, 115, 114],
        vec![81, 82, 88, 87, 113, 114, 119, 118],
        vec![82, 83, 89, 88, 114, 115, 120, 119],
        vec![86, 87, 92, 91, 117, 118, 123, 122],
        vec![88, 89, 94, 93, 119, 120, 125, 124],
        vec![89, 90, 95, 94, 120, 121, 126, 125],
        vec![98, 99, 104, 103, 130, 131, 137, 136],
        vec![99, 100, 105, 104, 131, 132, 138, 137],
        vec![101, 102, 108, 107, 134, 135, 141, 140],
        vec![102, 103, 109, 108, 135, 136, 142, 141],
        vec![106, 107, 112, 111, 139, 140, 146, 145],
        vec![108, 109, 114, 113, 141, 142, 148, 147],
        vec![111, 112, 117, 116, 145, 146, 151, 150],
        vec![112, 113, 118, 117, 146, 147, 152, 151],
        vec![114, 115, 120, 119, 148, 149, 154, 153],
        vec![118, 119, 124, 123, 152, 153, 159, 158],
        vec![120, 121, 126, 125, 154, 155, 161, 160],
        vec![127, 128, 134, 133, 162, 163, 168, 167],
        vec![129, 130, 136, 135, 164, 165, 170, 169],
        vec![130, 131, 137, 136, 165, 166, 171, 170],
        vec![133, 134, 140, 139, 167, 168, 174, 173],
        vec![134, 135, 141, 140, 168, 169, 175, 174],
        vec![135, 136, 142, 141, 169, 170, 176, 175],
        vec![136, 137, 143, 142, 170, 171, 177, 176],
        vec![137, 138, 144, 143, 171, 172, 178, 177],
        vec![141, 142, 148, 147, 175, 176, 180, 179],
        vec![147, 148, 153, 152, 179, 180, 185, 184],
        vec![148, 149, 154, 153, 180, 181, 186, 185],
        vec![150, 151, 157, 156, 182, 183, 189, 188],
        vec![151, 152, 158, 157, 183, 184, 190, 189],
        vec![154, 155, 161, 160, 186, 187, 192, 191],
    ];
    let nodal_coordinates = vec![
        vec![1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![5.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![1.0, 2.0, 0.0],
        vec![2.0, 2.0, 0.0],
        vec![3.0, 2.0, 0.0],
        vec![4.0, 2.0, 0.0],
        vec![5.0, 2.0, 0.0],
        vec![0.0, 3.0, 0.0],
        vec![1.0, 3.0, 0.0],
        vec![2.0, 3.0, 0.0],
        vec![3.0, 3.0, 0.0],
        vec![4.0, 3.0, 0.0],
        vec![5.0, 3.0, 0.0],
        vec![0.0, 4.0, 0.0],
        vec![1.0, 4.0, 0.0],
        vec![2.0, 4.0, 0.0],
        vec![3.0, 4.0, 0.0],
        vec![4.0, 4.0, 0.0],
        vec![5.0, 4.0, 0.0],
        vec![0.0, 5.0, 0.0],
        vec![1.0, 5.0, 0.0],
        vec![2.0, 5.0, 0.0],
        vec![1.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![3.0, 0.0, 1.0],
        vec![4.0, 0.0, 1.0],
        vec![5.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![3.0, 1.0, 1.0],
        vec![4.0, 1.0, 1.0],
        vec![5.0, 1.0, 1.0],
        vec![0.0, 2.0, 1.0],
        vec![1.0, 2.0, 1.0],
        vec![2.0, 2.0, 1.0],
        vec![3.0, 2.0, 1.0],
        vec![4.0, 2.0, 1.0],
        vec![5.0, 2.0, 1.0],
        vec![0.0, 3.0, 1.0],
        vec![1.0, 3.0, 1.0],
        vec![2.0, 3.0, 1.0],
        vec![3.0, 3.0, 1.0],
        vec![4.0, 3.0, 1.0],
        vec![5.0, 3.0, 1.0],
        vec![0.0, 4.0, 1.0],
        vec![1.0, 4.0, 1.0],
        vec![2.0, 4.0, 1.0],
        vec![3.0, 4.0, 1.0],
        vec![4.0, 4.0, 1.0],
        vec![5.0, 4.0, 1.0],
        vec![0.0, 5.0, 1.0],
        vec![1.0, 5.0, 1.0],
        vec![2.0, 5.0, 1.0],
        vec![3.0, 5.0, 1.0],
        vec![1.0, 0.0, 2.0],
        vec![2.0, 0.0, 2.0],
        vec![3.0, 0.0, 2.0],
        vec![4.0, 0.0, 2.0],
        vec![5.0, 0.0, 2.0],
        vec![0.0, 1.0, 2.0],
        vec![1.0, 1.0, 2.0],
        vec![2.0, 1.0, 2.0],
        vec![3.0, 1.0, 2.0],
        vec![4.0, 1.0, 2.0],
        vec![5.0, 1.0, 2.0],
        vec![0.0, 2.0, 2.0],
        vec![1.0, 2.0, 2.0],
        vec![2.0, 2.0, 2.0],
        vec![3.0, 2.0, 2.0],
        vec![4.0, 2.0, 2.0],
        vec![5.0, 2.0, 2.0],
        vec![0.0, 3.0, 2.0],
        vec![1.0, 3.0, 2.0],
        vec![2.0, 3.0, 2.0],
        vec![3.0, 3.0, 2.0],
        vec![4.0, 3.0, 2.0],
        vec![5.0, 3.0, 2.0],
        vec![0.0, 4.0, 2.0],
        vec![1.0, 4.0, 2.0],
        vec![2.0, 4.0, 2.0],
        vec![3.0, 4.0, 2.0],
        vec![4.0, 4.0, 2.0],
        vec![5.0, 4.0, 2.0],
        vec![1.0, 5.0, 2.0],
        vec![2.0, 5.0, 2.0],
        vec![3.0, 5.0, 2.0],
        vec![4.0, 5.0, 2.0],
        vec![5.0, 5.0, 2.0],
        vec![1.0, 0.0, 3.0],
        vec![2.0, 0.0, 3.0],
        vec![3.0, 0.0, 3.0],
        vec![4.0, 0.0, 3.0],
        vec![5.0, 0.0, 3.0],
        vec![1.0, 1.0, 3.0],
        vec![2.0, 1.0, 3.0],
        vec![3.0, 1.0, 3.0],
        vec![4.0, 1.0, 3.0],
        vec![5.0, 1.0, 3.0],
        vec![0.0, 2.0, 3.0],
        vec![1.0, 2.0, 3.0],
        vec![2.0, 2.0, 3.0],
        vec![3.0, 2.0, 3.0],
        vec![4.0, 2.0, 3.0],
        vec![0.0, 3.0, 3.0],
        vec![1.0, 3.0, 3.0],
        vec![2.0, 3.0, 3.0],
        vec![3.0, 3.0, 3.0],
        vec![4.0, 3.0, 3.0],
        vec![0.0, 4.0, 3.0],
        vec![1.0, 4.0, 3.0],
        vec![2.0, 4.0, 3.0],
        vec![3.0, 4.0, 3.0],
        vec![4.0, 4.0, 3.0],
        vec![5.0, 4.0, 3.0],
        vec![1.0, 5.0, 3.0],
        vec![2.0, 5.0, 3.0],
        vec![3.0, 5.0, 3.0],
        vec![4.0, 5.0, 3.0],
        vec![5.0, 5.0, 3.0],
        vec![0.0, 0.0, 4.0],
        vec![1.0, 0.0, 4.0],
        vec![2.0, 0.0, 4.0],
        vec![3.0, 0.0, 4.0],
        vec![4.0, 0.0, 4.0],
        vec![5.0, 0.0, 4.0],
        vec![0.0, 1.0, 4.0],
        vec![1.0, 1.0, 4.0],
        vec![2.0, 1.0, 4.0],
        vec![3.0, 1.0, 4.0],
        vec![4.0, 1.0, 4.0],
        vec![5.0, 1.0, 4.0],
        vec![0.0, 2.0, 4.0],
        vec![1.0, 2.0, 4.0],
        vec![2.0, 2.0, 4.0],
        vec![3.0, 2.0, 4.0],
        vec![4.0, 2.0, 4.0],
        vec![5.0, 2.0, 4.0],
        vec![0.0, 3.0, 4.0],
        vec![1.0, 3.0, 4.0],
        vec![2.0, 3.0, 4.0],
        vec![3.0, 3.0, 4.0],
        vec![4.0, 3.0, 4.0],
        vec![0.0, 4.0, 4.0],
        vec![1.0, 4.0, 4.0],
        vec![2.0, 4.0, 4.0],
        vec![3.0, 4.0, 4.0],
        vec![4.0, 4.0, 4.0],
        vec![5.0, 4.0, 4.0],
        vec![0.0, 5.0, 4.0],
        vec![1.0, 5.0, 4.0],
        vec![2.0, 5.0, 4.0],
        vec![3.0, 5.0, 4.0],
        vec![4.0, 5.0, 4.0],
        vec![5.0, 5.0, 4.0],
        vec![0.0, 0.0, 5.0],
        vec![1.0, 0.0, 5.0],
        vec![2.0, 0.0, 5.0],
        vec![3.0, 0.0, 5.0],
        vec![4.0, 0.0, 5.0],
        vec![0.0, 1.0, 5.0],
        vec![1.0, 1.0, 5.0],
        vec![2.0, 1.0, 5.0],
        vec![3.0, 1.0, 5.0],
        vec![4.0, 1.0, 5.0],
        vec![5.0, 1.0, 5.0],
        vec![0.0, 2.0, 5.0],
        vec![1.0, 2.0, 5.0],
        vec![2.0, 2.0, 5.0],
        vec![3.0, 2.0, 5.0],
        vec![4.0, 2.0, 5.0],
        vec![5.0, 2.0, 5.0],
        vec![2.0, 3.0, 5.0],
        vec![3.0, 3.0, 5.0],
        vec![4.0, 3.0, 5.0],
        vec![0.0, 4.0, 5.0],
        vec![1.0, 4.0, 5.0],
        vec![2.0, 4.0, 5.0],
        vec![3.0, 4.0, 5.0],
        vec![4.0, 4.0, 5.0],
        vec![5.0, 4.0, 5.0],
        vec![0.0, 5.0, 5.0],
        vec![1.0, 5.0, 5.0],
        vec![2.0, 5.0, 5.0],
        vec![4.0, 5.0, 5.0],
        vec![5.0, 5.0, 5.0],
    ];
    let node_element_connectivity_gold = vec![vec![]];
    let node_node_connectivity_gold = vec![vec![]];
    let exterior_nodes_gold = (1..=1).collect();
    let interface_nodes_gold = vec![];
    let interior_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        exterior_nodes_gold,
        interface_nodes_gold,
        interior_nodes_gold,
    );
}
