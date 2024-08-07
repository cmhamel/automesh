use automesh::{Abaqus, Spn};

const NELZ: usize = 4;
const NELY: usize = 5;
const NELX: usize = 3;
const NEL: [usize; 3] = [NELX, NELY, NELZ];
const SCALE: [f64; 3] = [1.2, 2.3, 0.4];
const TRANSLATE: [f64; 3] = [-0.3, 1.1, 0.5];

#[test]
fn write_inp() {
    let spn = Spn::new("tests/input/f.spn", NEL);
    let fem = spn.into_finite_elements(&SCALE, &TRANSLATE);
    fem.write_inp("target/f.inp");
    todo!("Need a gold tests/input/f.inp to compare against?")
}
