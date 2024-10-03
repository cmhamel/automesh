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
    let exterior_nodes_gold = vec![1, 2, 3, 4, 5, 6, 7, 8];
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
    let element_blocks = vec![11, 11];
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
    let exterior_nodes_gold = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
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
    let element_blocks = vec![11, 11];
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
    let exterior_nodes_gold = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
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
fn triple_x() {
    let element_blocks = vec![11, 11, 11];
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
    let exterior_nodes_gold = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
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
fn quadruple_x() {
    let element_blocks = vec![11, 11, 11, 11];
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
    let exterior_nodes_gold = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
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
fn quadruple_2_voids_x() {
    todo!()
}

#[test]
fn quadruple_2_blocks() {
    todo!()
}

#[test]
fn quadruple_2_blocks_remove_1() {
    todo!()
}

#[test]
fn quadruple_2_blocks_remove_2() {
    todo!()
}

#[test]
fn quadruple_2_blocks_void() {
    todo!()
}

#[test]
fn quadruple_2_blocks_void_remove_0() {
    todo!()
}

#[test]
fn quadruple_2_blocks_void_remove_1() {
    todo!()
}

#[test]
fn quadruple_2_blocks_void_remove_2() {
    todo!()
}

#[test]
fn quadruple_2_blocks_void_remove_3() {
    todo!()
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
    let exterior_nodes_gold = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27,
    ];
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
        vec![],
    ];
    let exterior_nodes_gold = vec![
        1, 3, 6, 7, 8, 9, 10, 16, 19, 20, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
    ];
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
    todo!()
}
#[test]
fn sparse() {
    todo!()
}
