use automesh::Spn;

const NELZ: usize = 4;
const NELY: usize = 5;
const NELX: usize = 3;
const NUM_ELEMENTS: usize = 39;
const NUM_NODES: usize = 102;

const GOLD_BLOCKS: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const GOLD_CONNECTIVITY: [[usize; 8]; NUM_ELEMENTS] = [
    [1, 2, 7, 6, 31, 32, 37, 36],
    [31, 32, 37, 36, 61, 62, 67, 66],
    [61, 62, 67, 66, 85, 86, 91, 90],
    [6, 7, 12, 11, 36, 37, 42, 41],
    [36, 37, 42, 41, 66, 67, 72, 71],
    [66, 67, 72, 71, 90, 91, 96, 95],
    [11, 12, 17, 16, 41, 42, 47, 46],
    [41, 42, 47, 46, 71, 72, 77, 76],
    [71, 72, 77, 76, 95, 96, 98, 97],
    [16, 17, 22, 21, 46, 47, 52, 51],
    [46, 47, 52, 51, 76, 77, 82, 81],
    [76, 77, 82, 81, 97, 98, 100, 99],
    [21, 22, 27, 26, 51, 52, 57, 56],
    [51, 52, 57, 56, 81, 82, 84, 83],
    [81, 82, 84, 83, 99, 100, 102, 101],
    [2, 3, 8, 7, 32, 33, 38, 37],
    [32, 33, 38, 37, 62, 63, 68, 67],
    [62, 63, 68, 67, 86, 87, 92, 91],
    [7, 8, 13, 12, 37, 38, 43, 42],
    [12, 13, 18, 17, 42, 43, 48, 47],
    [42, 43, 48, 47, 72, 73, 78, 77],
    [17, 18, 23, 22, 47, 48, 53, 52],
    [22, 23, 28, 27, 52, 53, 58, 57],
    [3, 4, 9, 8, 33, 34, 39, 38],
    [33, 34, 39, 38, 63, 64, 69, 68],
    [63, 64, 69, 68, 87, 88, 93, 92],
    [8, 9, 14, 13, 38, 39, 44, 43],
    [13, 14, 19, 18, 43, 44, 49, 48],
    [43, 44, 49, 48, 73, 74, 79, 78],
    [18, 19, 24, 23, 48, 49, 54, 53],
    [23, 24, 29, 28, 53, 54, 59, 58],
    [4, 5, 10, 9, 34, 35, 40, 39],
    [34, 35, 40, 39, 64, 65, 70, 69],
    [64, 65, 70, 69, 88, 89, 94, 93],
    [9, 10, 15, 14, 39, 40, 45, 44],
    [14, 15, 20, 19, 44, 45, 50, 49],
    [44, 45, 50, 49, 74, 75, 80, 79],
    [19, 20, 25, 24, 49, 50, 55, 54],
    [24, 25, 30, 29, 54, 55, 60, 59],
];
const GOLD_DATA: [[[u8; NELX]; NELY]; NELZ] = [
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
];

fn assert_data_eq_gold(spn: Spn) {
    let data = spn.get_data();
    vec![NELZ, NELY, NELX]
        .iter()
        .zip(data.shape().iter())
        .for_each(|(gold_n, data_n)| assert_eq!(gold_n, data_n));
    GOLD_DATA
        .iter()
        .zip(data.outer_iter())
        .for_each(|(gold_i, spn_i)| {
            gold_i
                .iter()
                .zip(spn_i.outer_iter())
                .for_each(|(gold_ij, spn_ij)| {
                    gold_ij
                        .iter()
                        .zip(spn_ij.iter())
                        .for_each(|(gold_ijk, spn_ijk)| assert_eq!(gold_ijk, spn_ijk))
                })
        })
}

#[test]
fn from_npy() {
    let spn = Spn::from_npy("tests/input/f.npy");
    assert_data_eq_gold(spn);
}

#[test]
fn into_exodus() {
    let spn = Spn::from_npy("tests/input/f.npy");
    let exo = spn.into_exodus();
    let blocks = exo.get_element_blocks();
    assert_eq!(GOLD_BLOCKS.len(), NUM_ELEMENTS);
    assert_eq!(blocks.len(), NUM_ELEMENTS);
    GOLD_BLOCKS
        .iter()
        .zip(blocks.iter())
        .for_each(|(gold, block)| assert_eq!(gold, block));
    let connectivity = exo.get_element_connectivity();
    assert_eq!(GOLD_BLOCKS.len(), NUM_ELEMENTS);
    assert_eq!(connectivity.len(), NUM_ELEMENTS);
    GOLD_CONNECTIVITY
        .iter()
        .flatten()
        .zip(connectivity.iter().flatten())
        .for_each(|(gold, entry)| assert_eq!(gold, entry));
    let nodal_coordinates = exo.get_nodal_coordinates();
    // todo!("number of nodes in GOLD == number of unique nodal ids in connectivity");
    assert_eq!(nodal_coordinates.len(), NUM_NODES);
    todo!("coordinates");
}

#[test]
fn new() {
    let spn = Spn::new("tests/input/f.spn", NELZ, NELY, NELX);
    assert_data_eq_gold(spn);
}
