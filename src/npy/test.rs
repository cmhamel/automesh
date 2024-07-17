use super::{filter_data, Npy};

const NUM_ELEMENTS: usize = 39;

const BLOCKS_GOLD: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const VOXELS_GOLD: [[usize; 3]; NUM_ELEMENTS] = [
    [0, 0, 0],
    [0, 0, 1],
    [0, 0, 2],
    [0, 1, 0],
    [0, 1, 1],
    [0, 1, 2],
    [0, 2, 0],
    [0, 2, 1],
    [0, 2, 2],
    [0, 3, 0],
    [0, 3, 1],
    [0, 3, 2],
    [0, 4, 0],
    [0, 4, 1],
    [0, 4, 2],
    [1, 0, 0],
    [1, 0, 1],
    [1, 0, 2],
    [1, 1, 0],
    [1, 2, 0],
    [1, 2, 1],
    [1, 3, 0],
    [1, 4, 0],
    [2, 0, 0],
    [2, 0, 1],
    [2, 0, 2],
    [2, 1, 0],
    [2, 2, 0],
    [2, 2, 1],
    [2, 3, 0],
    [2, 4, 0],
    [3, 0, 0],
    [3, 0, 1],
    [3, 0, 2],
    [3, 1, 0],
    [3, 2, 0],
    [3, 2, 1],
    [3, 3, 0],
    [3, 4, 0],
];

#[test]
fn filter() {
    let npy = Npy::new("tests/npy/f.npy");
    let (filtered_voxel_data, element_blocks) = filter_data(npy.get_data());
    assert_eq!(element_blocks.len(), NUM_ELEMENTS);
    BLOCKS_GOLD
        .iter()
        .zip(element_blocks.iter())
        .for_each(|(gold_n, block_n)| assert_eq!(gold_n, block_n));
    assert_eq!(filtered_voxel_data.len(), NUM_ELEMENTS);
    VOXELS_GOLD
        .iter()
        .zip(filtered_voxel_data.iter())
        .for_each(|(gold_n, block_n)| {
            gold_n
                .iter()
                .zip(block_n.iter())
                .for_each(|(gold_n_i, block_n_i)| assert_eq!(gold_n_i, block_n_i))
        });
}
