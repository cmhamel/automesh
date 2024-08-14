use automesh::Spn;

const NELZ: usize = 1;
const NELY: usize = 1;
const NELX: usize = 1;
const NSD: usize = 3; // number of space dimensions R3
const NEL: [usize; NSD] = [NELX, NELY, NELZ];
const NUM_ELEMENTS: usize = 1;
const NUM_NODES_ELEMENT: usize = 8;
const SCALE: [f64; NSD] = [1.0, 2.0, 3.0];
const TRANSLATE: [f64; NSD] = [4.0, 5.0, 6.0];

const GOLD_BLOCKS_SINGLE: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const GOLD_CONNECTIVITY_SINGLE: [[usize; NUM_NODES_ELEMENT]; NUM_ELEMENTS] =
    [[1, 2, 4, 3, 5, 6, 8, 7]];
const GOLD_COORDINATES_SINGLE: [[f64; NSD]; NUM_NODES_ELEMENT] = [
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
];
const GOLD_DATA_SINGLE: [[[u8; NELX]; NELY]; NELZ] = [[[1]]];

fn assert_data_eq_gold(spn: Spn) {
    let data = spn.get_data();
    data.shape()
        .iter()
        .zip(NEL.iter().rev())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    data.iter()
        .zip(GOLD_DATA_SINGLE.iter().flatten().flatten())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
}

#[test]
fn from_npy() {
    let spn = Spn::from_npy("tests/input/single.npy");
    assert_data_eq_gold(spn);
}

#[test]
fn into_finite_elements() {
    let spn = Spn::from_npy("tests/input/single.npy");
    let fem = spn.into_finite_elements(&SCALE, &TRANSLATE);
    let blocks = fem.get_element_blocks();
    assert_eq!(GOLD_BLOCKS_SINGLE.len(), NUM_ELEMENTS);
    assert_eq!(blocks.len(), NUM_ELEMENTS);
    blocks
        .iter()
        .zip(GOLD_BLOCKS_SINGLE.iter())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    let connectivity = fem.get_element_connectivity();
    assert_eq!(GOLD_BLOCKS_SINGLE.len(), NUM_ELEMENTS);
    assert_eq!(connectivity.len(), NUM_ELEMENTS);
    connectivity
        .iter()
        .flatten()
        .zip(GOLD_CONNECTIVITY_SINGLE.iter().flatten())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    let coordinates = fem.get_nodal_coordinates();
    assert_eq!(GOLD_COORDINATES_SINGLE.len(), NUM_NODES_ELEMENT);
    assert_eq!(coordinates.len(), NUM_NODES_ELEMENT);
    let gold_coordinates: Vec<Vec<f64>> = GOLD_COORDINATES_SINGLE
        .iter()
        .map(|coordinates| {
            coordinates
                .iter()
                .zip(SCALE.iter().zip(TRANSLATE.iter()))
                .map(|(coordinate, (scale, translate))| coordinate * scale + translate)
                .collect()
        })
        .collect();
    coordinates
        .iter()
        .flatten()
        .zip(gold_coordinates.iter().flatten())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
}

#[test]
fn new() {
    let spn = Spn::new("tests/input/single.spn", NEL);
    assert_data_eq_gold(spn);
}

#[test]
fn write_npy() {
    Spn::new("tests/input/single.spn", NEL).write_npy("target/single.npy");
    let spn = Spn::from_npy("target/single.npy");
    assert_data_eq_gold(spn);
}
