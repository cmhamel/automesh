#![feature(test)]

extern crate test;
use test::Bencher;

use automesh::Voxels;

const NEL: [usize; 3] = [10, 10, 10];
const REMOVE: Option<Vec<u8>> = None;
const SCALE: [f64; 3] = [1.0, 1.0, 1.0];
const TRANSLATE: [f64; 3] = [0.0, 0.0, 0.0];

// some seem to scale better than others
// so need both small and large benches to show what

#[bench]
fn from_npy(bencher: &mut Bencher) {
    bencher.iter(|| Voxels::from_npy("benches/block.npy"));
}

#[bench]
fn from_npy_into_finite_elements(bencher: &mut Bencher) {
    bencher.iter(|| {
        Voxels::from_npy("benches/block.npy")
            .unwrap()
            .into_finite_elements(REMOVE, &TRANSLATE, &SCALE)
    });
}

#[bench]
fn from_spn(bencher: &mut Bencher) {
    bencher.iter(|| Voxels::from_spn("benches/block.spn", NEL));
}

#[bench]
fn from_spn_into_finite_elements(bencher: &mut Bencher) {
    bencher.iter(|| {
        Voxels::from_spn("benches/block.spn", NEL)
            .unwrap()
            .into_finite_elements(REMOVE, &TRANSLATE, &SCALE)
    });
}

#[bench]
fn write_npy(bencher: &mut Bencher) -> Result<(), String> {
    let voxels = Voxels::from_spn("benches/block.spn", NEL)?;
    bencher.iter(|| voxels.write_spn("target/block.npy"));
    Ok(())
}

#[bench]
fn write_spn(bencher: &mut Bencher) -> Result<(), String> {
    let voxels = Voxels::from_spn("benches/block.spn", NEL)?;
    bencher.iter(|| voxels.write_spn("target/block.spn"));
    Ok(())
}

// #[bench]
// fn from_npy(bencher: &mut Bencher) {
//     let voxels = Voxels::from_npy("block.npy");
//     bencher.iter(||
//         voxels.
//     );
// }
