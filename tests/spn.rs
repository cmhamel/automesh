use automesh::Spn;

const NELZ: usize = 4;
const NELY: usize = 5;
const NELX: usize = 3;

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
    // blocks
    // connectivity
    // coordinates
    todo!("Needs to be tested here?")
}

#[test]
fn new() {
    let spn = Spn::new("tests/input/f.spn", NELZ, NELY, NELX);
    assert_data_eq_gold(spn);
}
