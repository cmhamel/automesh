use automesh::{FiniteElements, Voxels};

const NELX: usize = 4;
const NELY: usize = 5;
const NELZ: usize = 3;
const NEL: [usize; 3] = [NELX, NELY, NELZ];
const NUM_ELEMENTS: usize = 39;
const NUM_NODES: usize = 102;
const NUM_NODES_ELEMENT: usize = 8;
const SCALE: [f64; 3] = [1.2, 2.3, 0.4];
const SCALE_NONE: [f64; 3] = [1.0, 1.0, 1.0];
const TRANSLATE: [f64; 3] = [-0.3, 1.1, 0.5];
const TRANSLATE_NONE: [f64; 3] = [0.0, 0.0, 0.0];

const GOLD_BLOCKS: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const GOLD_CONNECTIVITY: [[usize; NUM_NODES_ELEMENT]; NUM_ELEMENTS] = [
    [1, 2, 7, 6, 31, 32, 37, 36],
    [2, 3, 8, 7, 32, 33, 38, 37],
    [3, 4, 9, 8, 33, 34, 39, 38],
    [4, 5, 10, 9, 34, 35, 40, 39],
    [6, 7, 12, 11, 36, 37, 42, 41],
    [7, 8, 13, 12, 37, 38, 43, 42],
    [8, 9, 14, 13, 38, 39, 44, 43],
    [9, 10, 15, 14, 39, 40, 45, 44],
    [11, 12, 17, 16, 41, 42, 47, 46],
    [12, 13, 18, 17, 42, 43, 48, 47],
    [13, 14, 19, 18, 43, 44, 49, 48],
    [14, 15, 20, 19, 44, 45, 50, 49],
    [16, 17, 22, 21, 46, 47, 52, 51],
    [17, 18, 23, 22, 47, 48, 53, 52],
    [18, 19, 24, 23, 48, 49, 54, 53],
    [19, 20, 25, 24, 49, 50, 55, 54],
    [21, 22, 27, 26, 51, 52, 57, 56],
    [22, 23, 28, 27, 52, 53, 58, 57],
    [23, 24, 29, 28, 53, 54, 59, 58],
    [24, 25, 30, 29, 54, 55, 60, 59],
    [31, 32, 37, 36, 61, 62, 64, 63],
    [36, 37, 42, 41, 63, 64, 66, 65],
    [41, 42, 47, 46, 65, 66, 71, 70],
    [42, 43, 48, 47, 66, 67, 72, 71],
    [43, 44, 49, 48, 67, 68, 73, 72],
    [44, 45, 50, 49, 68, 69, 74, 73],
    [46, 47, 52, 51, 70, 71, 76, 75],
    [51, 52, 57, 56, 75, 76, 81, 80],
    [52, 53, 58, 57, 76, 77, 82, 81],
    [53, 54, 59, 58, 77, 78, 83, 82],
    [54, 55, 60, 59, 78, 79, 84, 83],
    [61, 62, 64, 63, 85, 86, 88, 87],
    [63, 64, 66, 65, 87, 88, 90, 89],
    [65, 66, 71, 70, 89, 90, 92, 91],
    [70, 71, 76, 75, 91, 92, 94, 93],
    [75, 76, 81, 80, 93, 94, 99, 98],
    [76, 77, 82, 81, 94, 95, 100, 99],
    [77, 78, 83, 82, 95, 96, 101, 100],
    [78, 79, 84, 83, 96, 97, 102, 101],
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

fn assert_data_eq_gold(spn: Voxels) {
    let data = spn.get_data();
    data.shape()
        .iter()
        .zip(NEL.iter())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
    data.iter()
        .zip(GOLD_DATA.iter().flatten().flatten())
        .for_each(|(entry, gold)| assert_eq!(entry, gold));
}

fn assert_data_eq_gold_1d<T>(data: &Vec<T>, gold: &[T])
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    assert_eq!(data.len(), gold.len());
    data.iter()
        .zip(gold.iter())
        .for_each(|(data_entry, gold_entry)| assert_eq!(data_entry, gold_entry));
}

fn assert_data_eq_gold_2d<const N: usize, T>(data: &Vec<Vec<T>>, gold: &[[T; N]])
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    assert_eq!(data.len(), gold.len());
    assert_eq!(data[0].len(), gold[0].len());
    data.iter()
        .flatten()
        .zip(gold.iter().flatten())
        .for_each(|(data_entry, gold_entry)| assert_eq!(data_entry, gold_entry));
}

fn assert_fem_data_eq_gold(
    fem: &FiniteElements,
    gold_blocks: &[usize],
    gold_connectivity: &[[usize; 8]],
    gold_coordinates: &[[f64; 3]],
) {
    assert_data_eq_gold_1d(fem.get_element_blocks(), gold_blocks);
    assert_data_eq_gold_2d(fem.get_element_connectivity(), gold_connectivity);
    assert_data_eq_gold_2d(fem.get_nodal_coordinates(), gold_coordinates);
}

#[test]
fn from_spn() {
    let voxels = Voxels::from_spn("tests/input/f.spn", NEL);
    assert_data_eq_gold(voxels);
}

mod into_finite_elements {
    use super::*;
    #[test]
    fn single() {
        const BLOCK_CONNECTIVITY: [usize; 1] = [1];
        const ELEMENT_CONNECTIVITY: [[usize; 8]; 1] = [[1, 2, 4, 3, 5, 6, 8, 7]];
        const NODAL_COORDINATES: [[f64; 3]; 8] = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
        ];
        let voxels = Voxels::from_spn("tests/input/single.spn", [1, 1, 1]);
        let fem = voxels.into_finite_elements(&SCALE_NONE, &TRANSLATE_NONE);
        assert_fem_data_eq_gold(
            &fem,
            &BLOCK_CONNECTIVITY,
            &ELEMENT_CONNECTIVITY,
            &NODAL_COORDINATES,
        );
    }
    #[test]
    fn letter_f() {
        let voxels = Voxels::from_npy("tests/input/f.npy");
        let fem = voxels.into_finite_elements(&SCALE, &TRANSLATE);
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
}

#[test]
fn write_npy() {
    Voxels::from_spn("tests/input/f.spn", NEL).write_npy("target/f.npy");
    let voxels = Voxels::from_npy("target/f.npy");
    assert_data_eq_gold(voxels);
}

mod from_npy {
    use super::*;
    #[test]
    #[should_panic(expected = "File type must be .npy")]
    fn file_unreadable() {
        let _ = Voxels::from_npy("tests/input/f.txt");
    }
    #[test]
    #[should_panic(expected = "Could not find the .npy file")]
    fn file_nonexistent() {
        let _ = Voxels::from_npy("tests/input/f_file_nonexistent.npy");
    }
    #[test]
    #[should_panic(expected = "Could not open the .npy file")]
    fn file_unopenable() {
        let _ = Voxels::from_npy("tests/input/encrypted.npy");
    }
    #[test]
    fn success() {
        let voxels = Voxels::from_npy("tests/input/f.npy");
        assert_data_eq_gold(voxels);
    }
}
