use automesh::Npy;

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
fn test_read_npy() {
    let npy = Npy::new("tests/npy/f.npy");
    GOLD.iter()
        .flatten()
        .flatten()
        .zip(npy.get_data().iter())
        .for_each(|(gold_i, npy_i)| assert_eq!(gold_i, npy_i));
}
