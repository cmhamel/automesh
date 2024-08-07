use automesh::{Abaqus, FiniteElements, Spn};

const SCALE: [f64; 3] = [1.2, 2.3, 0.4];
const TRANSLATE: [f64; 3] = [-0.3, 1.1, 0.5];

#[test]
fn asdf() {
    let spn = Spn::from_npy("tests/input/f.npy");
    let fem = spn.into_finite_elements(&SCALE, &TRANSLATE);
    fem.write_inp("target/f.inp");
    todo!()
}
