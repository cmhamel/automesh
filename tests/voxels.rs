use ndarray::Array3;
use voxel_data::{Double, Single};

mod voxel_data;

#[test]
fn user() {
    let uu = voxel_data::User::new();
    assert!(uu.active);
    assert_eq!(uu.nsd, 3);
    assert_eq!(uu.nen, 8);
    assert_eq!(uu.username, "someone123");
    assert_eq!(uu.email, "someone@example.com");
}

#[test]
fn single() {
    let ii = Single::new();
    assert_eq!(ii.segmentation, Array3::from_elem((1, 1, 1), 1));
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = Single::gold_lattice();
    assert_eq!(ll, [[1, 2, 4, 3, 5, 6, 8, 7]]);
    let ee = Single::gold_elements();
    assert_eq!(ee, [[1, 2, 4, 3, 5, 6, 8, 7]]);
}

#[test]
fn double() {
    let ii = Double::new();
    assert_eq!(ii.segmentation, Array3::from_elem((1, 1, 2), 1));
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = Double::gold_lattice();
    assert_eq!(ll, [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]);
    let ee = Double::gold_elements();
    assert_eq!(ll, [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]);
}
