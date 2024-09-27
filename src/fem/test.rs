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
        finite_elements
            .get_node_element_connectivity()
            .iter()
            .for_each(|c| println!("{:?}", c));
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
