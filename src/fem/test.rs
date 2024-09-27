use super::FiniteElements;

mod single_element {
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
}

mod two_elements_x {
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
        node_element_connectivity
            .iter()
            .flatten()
            .zip(node_element_connectivity_gold.iter().flatten())
            .for_each(|(entry, gold)| assert_eq!(entry, gold));
    }
}
