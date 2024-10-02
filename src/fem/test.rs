use super::{Blocks, Connectivity, Coordinates, FiniteElements, Nodes};

fn test_finite_elements(
    element_blocks: Blocks,
    element_node_connectivity: Connectivity,
    nodal_coordinates: Coordinates,
    node_element_connectivity_gold: Connectivity,
    node_node_connectivity_gold: Connectivity,
    interface_nodes_gold: Nodes,
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
    assert_eq!(finite_elements.get_interface_nodes(), &interface_nodes_gold);
}

#[test]
fn single() {
    let element_blocks = vec![1];
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
    let interface_nodes_gold = vec![];
    test_finite_elements(
        element_blocks,
        element_node_connectivity,
        nodal_coordinates,
        node_element_connectivity_gold,
        node_node_connectivity_gold,
        interface_nodes_gold,
    );
}

mod single {
    use super::*;
    #[test]
    #[should_panic(expected = "Need to calculate and set the node-to-element connectivity first.")]
    fn calculate_nodal_hierarchy_did_not_calculate_node_element_connectivity() {
        let element_blocks = vec![1];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements.calculate_nodal_hierarchy().unwrap();
    }
    #[test]
    #[should_panic(expected = "Already calculated and set the nodal hierarchy.")]
    fn calculate_nodal_hierarchy_twice() {
        let element_blocks = vec![1];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        finite_elements.calculate_nodal_hierarchy().unwrap();
        finite_elements.calculate_nodal_hierarchy().unwrap();
    }
    #[test]
    #[should_panic(expected = "Already calculated and set the node-to-element connectivity.")]
    fn calculate_node_element_connectivity_twice() {
        let element_blocks = vec![1];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
    }
    #[test]
    #[should_panic(expected = "Need to calculate and set the node-to-element connectivity first.")]
    fn calculate_node_node_connectivity_did_not_calculate_node_element_connectivity() {
        let element_blocks = vec![1];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements.calculate_node_node_connectivity().unwrap();
    }
    #[test]
    #[should_panic(expected = "Already calculated and set the node-to-node connectivity.")]
    fn calculate_node_node_connectivity_twice() {
        let element_blocks = vec![1];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        finite_elements.calculate_node_node_connectivity().unwrap();
        finite_elements.calculate_node_node_connectivity().unwrap();
    }
}

mod double_x {
    use super::*;
    #[test]
    fn calculate_nodal_hierarchy() {
        let element_blocks = vec![1, 2];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        finite_elements.calculate_nodal_hierarchy().unwrap();
        let interface_nodes = finite_elements.get_interface_nodes();
        assert_eq!(interface_nodes, &vec![2, 5, 8, 11]);
    }
    #[test]
    fn calculate_node_element_connectivity() {
        let element_blocks = vec![1, 2];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        let node_element_connectivity = finite_elements.get_node_element_connectivity();
        assert_eq!(node_element_connectivity, &node_element_connectivity_gold);
    }
    #[test]
    fn calculate_node_node_connectivity() {
        let element_blocks = vec![1, 2];
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        finite_elements.calculate_node_node_connectivity().unwrap();
        let node_node_connectivity = finite_elements.get_node_node_connectivity();
        assert_eq!(node_node_connectivity, &node_node_connectivity_gold);
    }
}

mod cube {
    use super::*;
    #[test]
    fn calculate_node_element_connectivity() {
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        let node_element_connectivity = finite_elements.get_node_element_connectivity();
        assert_eq!(node_element_connectivity, &node_element_connectivity_gold);
    }
    #[test]
    fn calculate_node_node_connectivity() {
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
        finite_elements.calculate_node_node_connectivity().unwrap();
        let node_node_connectivity = finite_elements.get_node_node_connectivity();
        assert_eq!(node_node_connectivity, &node_node_connectivity_gold);
    }
}
