use automesh::Npy;

const NELZ: usize = 4;
const NELY: usize = 5;
const NELX: usize = 3;

const DATA_GOLD: [[[u8; NELX]; NELY]; NELZ] = [
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
];

#[test]
fn new() {
    let npy = Npy::new("tests/npy/f.npy");
    DATA_GOLD
        .iter()
        .zip(npy.get_data().outer_iter())
        .for_each(|(gold_k, npy_k)| {
            gold_k
                .iter()
                .zip(npy_k.outer_iter())
                .for_each(|(gold_kj, npy_kj)| {
                    gold_kj
                        .iter()
                        .zip(npy_kj.iter())
                        .for_each(|(gold_kji, npy_kji)| assert_eq!(gold_kji, npy_kji))
                })
        })
}
