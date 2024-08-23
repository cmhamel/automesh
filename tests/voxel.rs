use automesh::Voxels;

const NELX: usize = 4;
const NELY: usize = 5;
const NELZ: usize = 3;
const NEL: [usize; 3] = [NELX, NELY, NELZ];
const SCALE_NONE: [f64; 3] = [1.0, 1.0, 1.0];
const TRANSLATE_NONE: [f64; 3] = [0.0, 0.0, 0.0];

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

fn assert_fem_data_from_spn_eq_gold(
    file_path: &str,
    gold_blocks: &[usize],
    gold_connectivity: &[[usize; 8]],
    gold_coordinates: &[[f64; 3]],
    nel: [usize; 3],
) {
    let voxels = Voxels::from_spn(file_path, nel);
    let fem = voxels.into_finite_elements(&SCALE_NONE, &TRANSLATE_NONE);
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
        let file_path = "tests/input/single.spn";
        let gold_blocks = [1];
        let gold_connectivity = [[1, 2, 4, 3, 5, 6, 8, 7]];
        let gold_coordinates = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
        ];
        let nel = [1, 1, 1];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
    }
    #[test]
    fn double_x() {
        let file_path = "tests/input/double.spn";
        let gold_blocks = [1, 1];
        let gold_connectivity = [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]];
        let gold_coordinates = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [2.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [2.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [2.0, 1.0, 1.0],
        ];
        let nel = [2, 1, 1];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
    }
    #[test]
    fn double_y() {
        let file_path = "tests/input/double.spn";
        let gold_blocks = [1, 1];
        let gold_connectivity = [[1, 2, 4, 3, 7, 8, 10, 9], [3, 4, 6, 5, 9, 10, 12, 11]];
        let gold_coordinates = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 2.0, 0.0],
            [1.0, 2.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 2.0, 1.0],
            [1.0, 2.0, 1.0],
        ];
        let nel = [1, 2, 1];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
    }
    #[test]
    fn triple_x() {
        let file_path = "tests/input/triple.spn";
        let gold_blocks = [1, 1, 1];
        let gold_connectivity = [
            [1, 2, 6, 5, 9, 10, 14, 13],
            [2, 3, 7, 6, 10, 11, 15, 14],
            [3, 4, 8, 7, 11, 12, 16, 15],
        ];
        let gold_coordinates = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [2.0, 1.0, 0.0],
            [3.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [2.0, 0.0, 1.0],
            [3.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [2.0, 1.0, 1.0],
            [3.0, 1.0, 1.0],
        ];
        let nel = [3, 1, 1];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
    }
    #[test]
    fn quadruple_x() {
        let file_path = "tests/input/quadruple.spn";
        let gold_blocks = [1, 1, 1, 1];
        let gold_connectivity = [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ];
        let gold_coordinates = [
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
        ];
        let nel = [4, 1, 1];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
    }
    #[test]
    fn quadruple_2_voids_x() {
        let file_path = "tests/input/quadruple_2_voids.spn";
        let gold_blocks = [1, 1];
        let gold_connectivity = [[1, 2, 6, 5, 9, 10, 14, 13], [3, 4, 8, 7, 11, 12, 16, 15]];
        let gold_coordinates = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [3.0, 1.0, 0.0],
            [4.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [3.0, 0.0, 1.0],
            [4.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [3.0, 1.0, 1.0],
            [4.0, 1.0, 1.0],
        ];
        let nel = [4, 1, 1];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
    }
    #[test]
    fn cube() {
        let file_path = "tests/input/cube.spn";
        let gold_blocks = [1, 1, 1, 1, 1, 1, 1, 1];
        let gold_connectivity = [
            [1, 2, 5, 4, 10, 11, 14, 13],
            [2, 3, 6, 5, 11, 12, 15, 14],
            [4, 5, 8, 7, 13, 14, 17, 16],
            [5, 6, 9, 8, 14, 15, 18, 17],
            [10, 11, 14, 13, 19, 20, 23, 22],
            [11, 12, 15, 14, 20, 21, 24, 23],
            [13, 14, 17, 16, 22, 23, 26, 25],
            [14, 15, 18, 17, 23, 24, 27, 26],
        ];
        let gold_coordinates = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [2.0, 1.0, 0.0],
            [0.0, 2.0, 0.0],
            [1.0, 2.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [2.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [2.0, 1.0, 1.0],
            [0.0, 2.0, 1.0],
            [1.0, 2.0, 1.0],
            [2.0, 2.0, 1.0],
            [0.0, 0.0, 2.0],
            [1.0, 0.0, 2.0],
            [2.0, 0.0, 2.0],
            [0.0, 1.0, 2.0],
            [1.0, 1.0, 2.0],
            [2.0, 1.0, 2.0],
            [0.0, 2.0, 2.0],
            [1.0, 2.0, 2.0],
            [2.0, 2.0, 2.0],
        ];
        let nel = [2, 2, 2];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
    }
    #[test]
    fn letter_f() {
        let file_path = "tests/input/f.spn";
        let gold_blocks = [1; 39];
        let gold_connectivity = [
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
        let gold_coordinates = [
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
        let nel = [4, 5, 3];
        assert_fem_data_from_spn_eq_gold(
            file_path,
            &gold_blocks,
            &gold_connectivity,
            &gold_coordinates,
            nel,
        );
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
