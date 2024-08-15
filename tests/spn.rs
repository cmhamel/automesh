use automesh::Spn;

const NELZ: usize = 4;
const NELY: usize = 5;
const NELX: usize = 3;
const NSD: usize = 3;
const NEL: [usize; NSD] = [NELX, NELY, NELZ];
const NUM_ELEMENTS: usize = 39;
const NUM_NODES: usize = 102;
const NUM_NODES_ELEMENT: usize = 8;
const SCALE: [f64; NSD] = [1.2, 2.3, 0.4];
const TRANSLATE: [f64; NSD] = [-0.3, 1.1, 0.5];

const GOLD_BLOCKS: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const GOLD_CONNECTIVITY: [[usize; NUM_NODES_ELEMENT]; NUM_ELEMENTS] = [
    [2, 1, 6, 7, 32, 31, 36, 37],
    [32, 31, 36, 37, 62, 61, 66, 67],
    [62, 61, 66, 67, 86, 85, 90, 91],
    [7, 6, 11, 12, 37, 36, 41, 42],
    [37, 36, 41, 42, 67, 66, 71, 72],
    [67, 66, 71, 72, 91, 90, 95, 96],
    [12, 11, 16, 17, 42, 41, 46, 47],
    [42, 41, 46, 47, 72, 71, 76, 77],
    [72, 71, 76, 77, 96, 95, 97, 98],
    [17, 16, 21, 22, 47, 46, 51, 52],
    [47, 46, 51, 52, 77, 76, 81, 82],
    [77, 76, 81, 82, 98, 97, 99, 100],
    [22, 21, 26, 27, 52, 51, 56, 57],
    [52, 51, 56, 57, 82, 81, 83, 84],
    [82, 81, 83, 84, 100, 99, 101, 102],
    [3, 2, 7, 8, 33, 32, 37, 38],
    [33, 32, 37, 38, 63, 62, 67, 68],
    [63, 62, 67, 68, 87, 86, 91, 92],
    [8, 7, 12, 13, 38, 37, 42, 43],
    [13, 12, 17, 18, 43, 42, 47, 48],
    [43, 42, 47, 48, 73, 72, 77, 78],
    [18, 17, 22, 23, 48, 47, 52, 53],
    [23, 22, 27, 28, 53, 52, 57, 58],
    [4, 3, 8, 9, 34, 33, 38, 39],
    [34, 33, 38, 39, 64, 63, 68, 69],
    [64, 63, 68, 69, 88, 87, 92, 93],
    [9, 8, 13, 14, 39, 38, 43, 44],
    [14, 13, 18, 19, 44, 43, 48, 49],
    [44, 43, 48, 49, 74, 73, 78, 79],
    [19, 18, 23, 24, 49, 48, 53, 54],
    [24, 23, 28, 29, 54, 53, 58, 59],
    [5, 4, 9, 10, 35, 34, 39, 40],
    [35, 34, 39, 40, 65, 64, 69, 70],
    [65, 64, 69, 70, 89, 88, 93, 94],
    [10, 9, 14, 15, 40, 39, 44, 45],
    [15, 14, 19, 20, 45, 44, 49, 50],
    [45, 44, 49, 50, 75, 74, 79, 80],
    [20, 19, 24, 25, 50, 49, 54, 55],
    [25, 24, 29, 30, 55, 54, 59, 60],
];
const GOLD_COORDINATES: [[f64; 3]; NUM_NODES] = [
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 2.0],
    [0.0, 0.0, 3.0],
    [0.0, 0.0, 4.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 2.0],
    [0.0, 1.0, 3.0],
    [0.0, 1.0, 4.0],
    [0.0, 2.0, 0.0],
    [0.0, 2.0, 1.0],
    [0.0, 2.0, 2.0],
    [0.0, 2.0, 3.0],
    [0.0, 2.0, 4.0],
    [0.0, 3.0, 0.0],
    [0.0, 3.0, 1.0],
    [0.0, 3.0, 2.0],
    [0.0, 3.0, 3.0],
    [0.0, 3.0, 4.0],
    [0.0, 4.0, 0.0],
    [0.0, 4.0, 1.0],
    [0.0, 4.0, 2.0],
    [0.0, 4.0, 3.0],
    [0.0, 4.0, 4.0],
    [0.0, 5.0, 0.0],
    [0.0, 5.0, 1.0],
    [0.0, 5.0, 2.0],
    [0.0, 5.0, 3.0],
    [0.0, 5.0, 4.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 0.0, 2.0],
    [1.0, 0.0, 3.0],
    [1.0, 0.0, 4.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 2.0],
    [1.0, 1.0, 3.0],
    [1.0, 1.0, 4.0],
    [1.0, 2.0, 0.0],
    [1.0, 2.0, 1.0],
    [1.0, 2.0, 2.0],
    [1.0, 2.0, 3.0],
    [1.0, 2.0, 4.0],
    [1.0, 3.0, 0.0],
    [1.0, 3.0, 1.0],
    [1.0, 3.0, 2.0],
    [1.0, 3.0, 3.0],
    [1.0, 3.0, 4.0],
    [1.0, 4.0, 0.0],
    [1.0, 4.0, 1.0],
    [1.0, 4.0, 2.0],
    [1.0, 4.0, 3.0],
    [1.0, 4.0, 4.0],
    [1.0, 5.0, 0.0],
    [1.0, 5.0, 1.0],
    [1.0, 5.0, 2.0],
    [1.0, 5.0, 3.0],
    [1.0, 5.0, 4.0],
    [2.0, 0.0, 0.0],
    [2.0, 0.0, 1.0],
    [2.0, 0.0, 2.0],
    [2.0, 0.0, 3.0],
    [2.0, 0.0, 4.0],
    [2.0, 1.0, 0.0],
    [2.0, 1.0, 1.0],
    [2.0, 1.0, 2.0],
    [2.0, 1.0, 3.0],
    [2.0, 1.0, 4.0],
    [2.0, 2.0, 0.0],
    [2.0, 2.0, 1.0],
    [2.0, 2.0, 2.0],
    [2.0, 2.0, 3.0],
    [2.0, 2.0, 4.0],
    [2.0, 3.0, 0.0],
    [2.0, 3.0, 1.0],
    [2.0, 3.0, 2.0],
    [2.0, 3.0, 3.0],
    [2.0, 3.0, 4.0],
    [2.0, 4.0, 0.0],
    [2.0, 4.0, 1.0],
    [2.0, 5.0, 0.0],
    [2.0, 5.0, 1.0],
    [3.0, 0.0, 0.0],
    [3.0, 0.0, 1.0],
    [3.0, 0.0, 2.0],
    [3.0, 0.0, 3.0],
    [3.0, 0.0, 4.0],
    [3.0, 1.0, 0.0],
    [3.0, 1.0, 1.0],
    [3.0, 1.0, 2.0],
    [3.0, 1.0, 3.0],
    [3.0, 1.0, 4.0],
    [3.0, 2.0, 0.0],
    [3.0, 2.0, 1.0],
    [3.0, 3.0, 0.0],
    [3.0, 3.0, 1.0],
    [3.0, 4.0, 0.0],
    [3.0, 4.0, 1.0],
    [3.0, 5.0, 0.0],
    [3.0, 5.0, 1.0],
];
const GOLD_DATA: [[[u8; NELX]; NELY]; NELZ] = [
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
];

fn assert_data_eq_gold(spn: Spn) {
    let data = spn.get_data();
    data.shape()
        .iter()
        .zip(NEL.iter().rev())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    data.iter()
        .zip(GOLD_DATA.iter().flatten().flatten())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
}

#[test]
fn from_npy() {
    let spn = Spn::from_npy("tests/input/f.npy");
    assert_data_eq_gold(spn);
}

#[test]
fn into_finite_elements() {
    let spn = Spn::from_npy("tests/input/f.npy");
    let fem = spn.into_finite_elements(&SCALE, &TRANSLATE);
    let blocks = fem.get_element_blocks();
    assert_eq!(GOLD_BLOCKS.len(), NUM_ELEMENTS);
    assert_eq!(blocks.len(), NUM_ELEMENTS);
    blocks
        .iter()
        .zip(GOLD_BLOCKS.iter())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    let connectivity = fem.get_element_connectivity();
    assert_eq!(GOLD_BLOCKS.len(), NUM_ELEMENTS);
    assert_eq!(connectivity.len(), NUM_ELEMENTS);
    connectivity
        .iter()
        .flatten()
        .zip(GOLD_CONNECTIVITY.iter().flatten())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    let coordinates = fem.get_nodal_coordinates();
    assert_eq!(GOLD_COORDINATES.len(), NUM_NODES);
    assert_eq!(coordinates.len(), NUM_NODES);
    let gold_coordinates: Vec<Vec<f64>> = GOLD_COORDINATES
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
    let spn = Spn::new("tests/input/f.spn", NEL);
    assert_data_eq_gold(spn);
}

#[test]
fn write_npy() {
    Spn::new("tests/input/f.spn", NEL).write_npy("target/f.npy");
    let spn = Spn::from_npy("target/f.npy");
    assert_data_eq_gold(spn);
}
