use automesh::Spn;

const NELZ: usize = 4;
const NELY: usize = 5;
const NELX: usize = 3;
const NUM_ELEMENTS: usize = 39;

const GOLD_BLOCKS: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const GOLD_CONNECTIVITY: [[usize; 8]; NUM_ELEMENTS] = [
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6, 7, 8],
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
    assert_eq!(GOLD_BLOCKS.len(), blocks.len());
    GOLD_BLOCKS
        .iter()
        .zip(blocks.iter())
        .for_each(|(gold, block)| assert_eq!(gold, block));
    let connectivity = exo.get_element_connectivity();
    assert_eq!(GOLD_BLOCKS.len(), NUM_ELEMENTS);
    assert_eq!(GOLD_CONNECTIVITY.len(), connectivity.len());
    GOLD_CONNECTIVITY
        .iter()
        .flatten()
        .zip(connectivity.iter().flatten())
        .for_each(|(gold, connective)| assert_eq!(gold, connective));
    let nodal_coordinates = exo.get_nodal_coordinates();
    todo!("number of nodes in GOLD == number of unique nodal ids in connectivity");
    todo!("number of nodes in nodal_coordinates == number of unique nodal ids in connectivity");
    todo!("coordinates");
}

#[test]
fn new() {
    let spn = Spn::new("tests/input/f.spn", NELZ, NELY, NELX);
    assert_data_eq_gold(spn);
}
