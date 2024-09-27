use super::FiniteElements;

mod single {
    use super::*;
    #[test]
    fn calculate_node_element_connectivity() {
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
            .expect("the unexpected");
        let node_element_connectivity = finite_elements.get_node_element_connectivity();
        assert_eq!(node_element_connectivity.len(), 8);
        node_element_connectivity.iter().for_each(|connectivity| {
            assert_eq!(connectivity.len(), 1);
            assert_eq!(connectivity[0], 1);
        });
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
            .expect("the unexpected");
        finite_elements
            .calculate_node_element_connectivity()
            .unwrap();
    }
    #[test]
    fn calculate_node_node_connectivity() {
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
        let mut finite_elements =
            FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates);
        finite_elements
            .calculate_node_element_connectivity()
            .expect("the unexpected");
        finite_elements
            .calculate_node_node_connectivity()
            .expect("the unexpected");
        let node_node_connectivity = finite_elements.get_node_node_connectivity();
        assert_eq!(node_node_connectivity.len(), 8);
        assert_eq!(node_node_connectivity_gold.len(), 8);
        node_node_connectivity
            .iter()
            .flatten()
            .zip(node_node_connectivity_gold.iter().flatten())
            .for_each(|(entry, gold)| assert_eq!(entry, gold));
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
            .expect("the unexpected");
        finite_elements
            .calculate_node_node_connectivity()
            .expect("the unexpected");
        finite_elements.calculate_node_node_connectivity().unwrap();
    }
}

mod double_x {
    use super::*;
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
            .expect("the unexpected");
        let node_element_connectivity = finite_elements.get_node_element_connectivity();
        assert_eq!(node_element_connectivity.len(), 12);
        assert_eq!(node_element_connectivity_gold.len(), 12);
        node_element_connectivity
            .iter()
            .flatten()
            .zip(node_element_connectivity_gold.iter().flatten())
            .for_each(|(entry, gold)| assert_eq!(entry, gold));
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
            .expect("the unexpected");
        let node_element_connectivity = finite_elements.get_node_element_connectivity();
        assert_eq!(node_element_connectivity.len(), 27);
        assert_eq!(node_element_connectivity_gold.len(), 27);
        node_element_connectivity
            .iter()
            .flatten()
            .zip(node_element_connectivity_gold.iter().flatten())
            .for_each(|(entry, gold)| assert_eq!(entry, gold));
    }
}
