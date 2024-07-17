use super::{filter_data, Npy};

const NUM_BLOCKS: usize = 39;

const BLOCKS_GOLD: [usize; 39] = [1; 39];

#[test]
fn filter() {
    let npy = Npy::new("tests/npy/f.npy");
    let (lattice_data, element_blocks) = filter_data(npy.get_data());
    BLOCKS_GOLD
        .iter()
        .zip(element_blocks.iter())
        .for_each(|(gold_i, block_i)| assert_eq!(gold_i, block_i));
}
