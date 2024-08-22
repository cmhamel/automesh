use automesh::Spn;

const NELX: usize = 4;
const NELY: usize = 5;
const NELZ: usize = 3;
const NEL: [usize; 3] = [NELX, NELY, NELZ];
const NUM_ELEMENTS: usize = 39;
const NUM_NODES: usize = 102;
const NUM_NODES_ELEMENT: usize = 8;
const SCALE: [f64; 3] = [1.2, 2.3, 0.4];
const TRANSLATE: [f64; 3] = [-0.3, 1.1, 0.5];

const GOLD_BLOCKS: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const GOLD_CONNECTIVITY: [[usize; NUM_NODES_ELEMENT]; NUM_ELEMENTS] = [
    [1, 2, 6, 7, 31, 32, 36, 37],
    [2, 3, 7, 8, 32, 33, 37, 38],
    [3, 4, 8, 9, 33, 34, 38, 39],
    [4, 5, 9, 10, 34, 35, 39, 40],
    [6, 7, 11, 12, 36, 37, 41, 42],
    [7, 8, 12, 13, 37, 38, 42, 43],
    [8, 9, 13, 14, 38, 39, 43, 44],
    [9, 10, 14, 15, 39, 40, 44, 45],
    [11, 12, 16, 17, 41, 42, 46, 47],
    [12, 13, 17, 18, 42, 43, 47, 48],
    [13, 14, 18, 19, 43, 44, 48, 49],
    [14, 15, 19, 20, 44, 45, 49, 50],
    [16, 17, 21, 22, 46, 47, 51, 52],
    [17, 18, 22, 23, 47, 48, 52, 53],
    [18, 19, 23, 24, 48, 49, 53, 54],
    [19, 20, 24, 25, 49, 50, 54, 55],
    [21, 22, 26, 27, 51, 52, 56, 57],
    [22, 23, 27, 28, 52, 53, 57, 58],
    [23, 24, 28, 29, 53, 54, 58, 59],
    [24, 25, 29, 30, 54, 55, 59, 60],
    [31, 32, 36, 37, 61, 62, 63, 64],
    [36, 37, 41, 42, 63, 64, 65, 66],
    [41, 42, 46, 47, 65, 66, 70, 71],
    [42, 43, 47, 48, 66, 67, 71, 72],
    [43, 44, 48, 49, 67, 68, 72, 73],
    [44, 45, 49, 50, 68, 69, 73, 74],
    [46, 47, 51, 52, 70, 71, 75, 76],
    [51, 52, 56, 57, 75, 76, 80, 81],
    [52, 53, 57, 58, 76, 77, 81, 82],
    [53, 54, 58, 59, 77, 78, 82, 83],
    [54, 55, 59, 60, 78, 79, 83, 84],
    [61, 62, 63, 64, 85, 86, 87, 88],
    [63, 64, 65, 66, 87, 88, 89, 90],
    [65, 66, 70, 71, 89, 90, 91, 92],
    [70, 71, 75, 76, 91, 92, 93, 94],
    [75, 76, 80, 81, 93, 94, 98, 99],
    [76, 77, 81, 82, 94, 95, 99, 100],
    [77, 78, 82, 83, 95, 96, 100, 101],
    [78, 79, 83, 84, 96, 97, 101, 102],
];
const GOLD_COORDINATES: [[f64; 3]; NUM_NODES] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [2.0, 0.0, 0.0],
    [3.0, 0.0, 0.0],
    [4.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [2.0, 1.0, 0.0],
    [3.0, 1.0, 0.0],
    [4.0, 1.0, 0.0],
    [0.0, 2.0, 0.0],
    [1.0, 2.0, 0.0],
    [2.0, 2.0, 0.0],
    [3.0, 2.0, 0.0],
    [4.0, 2.0, 0.0],
    [0.0, 3.0, 0.0],
    [1.0, 3.0, 0.0],
    [2.0, 3.0, 0.0],
    [3.0, 3.0, 0.0],
    [4.0, 3.0, 0.0],
    [0.0, 4.0, 0.0],
    [1.0, 4.0, 0.0],
    [2.0, 4.0, 0.0],
    [3.0, 4.0, 0.0],
    [4.0, 4.0, 0.0],
    [0.0, 5.0, 0.0],
    [1.0, 5.0, 0.0],
    [2.0, 5.0, 0.0],
    [3.0, 5.0, 0.0],
    [4.0, 5.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [2.0, 0.0, 1.0],
    [3.0, 0.0, 1.0],
    [4.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [2.0, 1.0, 1.0],
    [3.0, 1.0, 1.0],
    [4.0, 1.0, 1.0],
    [0.0, 2.0, 1.0],
    [1.0, 2.0, 1.0],
    [2.0, 2.0, 1.0],
    [3.0, 2.0, 1.0],
    [4.0, 2.0, 1.0],
    [0.0, 3.0, 1.0],
    [1.0, 3.0, 1.0],
    [2.0, 3.0, 1.0],
    [3.0, 3.0, 1.0],
    [4.0, 3.0, 1.0],
    [0.0, 4.0, 1.0],
    [1.0, 4.0, 1.0],
    [2.0, 4.0, 1.0],
    [3.0, 4.0, 1.0],
    [4.0, 4.0, 1.0],
    [0.0, 5.0, 1.0],
    [1.0, 5.0, 1.0],
    [2.0, 5.0, 1.0],
    [3.0, 5.0, 1.0],
    [4.0, 5.0, 1.0],
    [0.0, 0.0, 2.0],
    [1.0, 0.0, 2.0],
    [0.0, 1.0, 2.0],
    [1.0, 1.0, 2.0],
    [0.0, 2.0, 2.0],
    [1.0, 2.0, 2.0],
    [2.0, 2.0, 2.0],
    [3.0, 2.0, 2.0],
    [4.0, 2.0, 2.0],
    [0.0, 3.0, 2.0],
    [1.0, 3.0, 2.0],
    [2.0, 3.0, 2.0],
    [3.0, 3.0, 2.0],
    [4.0, 3.0, 2.0],
    [0.0, 4.0, 2.0],
    [1.0, 4.0, 2.0],
    [2.0, 4.0, 2.0],
    [3.0, 4.0, 2.0],
    [4.0, 4.0, 2.0],
    [0.0, 5.0, 2.0],
    [1.0, 5.0, 2.0],
    [2.0, 5.0, 2.0],
    [3.0, 5.0, 2.0],
    [4.0, 5.0, 2.0],
    [0.0, 0.0, 3.0],
    [1.0, 0.0, 3.0],
    [0.0, 1.0, 3.0],
    [1.0, 1.0, 3.0],
    [0.0, 2.0, 3.0],
    [1.0, 2.0, 3.0],
    [0.0, 3.0, 3.0],
    [1.0, 3.0, 3.0],
    [0.0, 4.0, 3.0],
    [1.0, 4.0, 3.0],
    [2.0, 4.0, 3.0],
    [3.0, 4.0, 3.0],
    [4.0, 4.0, 3.0],
    [0.0, 5.0, 3.0],
    [1.0, 5.0, 3.0],
    [2.0, 5.0, 3.0],
    [3.0, 5.0, 3.0],
    [4.0, 5.0, 3.0],
];
const GOLD_DATA: [[[u8; NELZ]; NELY]; NELX] = [
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
    [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
    [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
];

fn assert_data_eq_gold(spn: Spn) {
    let data = spn.get_data();
    data.shape()
        .iter()
        .zip(NEL.iter())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    data.iter()
        .zip(GOLD_DATA.iter().flatten().flatten())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
}

#[test]
fn from_spn() {
    assert_data_eq_gold(Spn::from_spn("tests/input/f.spn", NEL));
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
fn write_npy() {
    Spn::from_spn("tests/input/f.spn", NEL).write_npy("target/f.npy");
    let spn = Spn::from_npy("target/f.npy");
    assert_data_eq_gold(spn);
}

mod from_npy {
    use super::*;

    #[test]
    #[should_panic(expected = "File type must be .npy")]
    fn file_unreadable() {
        let _ = Spn::from_npy("tests/input/f.txt");
    }

    #[test]
    #[should_panic(expected = "Could not find the .npy file")]
    fn file_nonexistent() {
        let _ = Spn::from_npy("tests/input/f_file_nonexistent.npy");
    }

    #[test]
    #[should_panic(expected = "Could not open the .npy file")]
    fn file_unopenable() {
        let _ = Spn::from_npy("tests/input/encrypted.npy");
    }

    #[test]
    fn success() {
        assert_data_eq_gold(Spn::from_npy("tests/input/f.npy"));
    }
}
