use automesh::Spn;

const NELX: usize = 3;
const NELY: usize = 5;
const NELZ: usize = 4;

const GOLD: [[[u8; NELX]; NELY]; NELZ] = [
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
];

#[test]
fn read() {
    let spn = Spn::new("tests/spn/f.spn", NELX, NELY, NELZ);
    GOLD.iter()
        .zip(spn.get_data().iter())
        .for_each(|(gold_i, spn_i)| {
            gold_i
                .iter()
                .zip(spn_i.iter())
                .for_each(|(gold_ij, spn_ij)| {
                    gold_ij
                        .iter()
                        .zip(spn_ij.iter())
                        .for_each(|(gold_ijk, spn_ijk)| assert_eq!(gold_ijk, spn_ijk))
                })
        })
}
