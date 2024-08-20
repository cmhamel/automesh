//! Voxels testing:
//!
//! This module tests convertion from a segmentation to a lattice and a mesh.
//!

use ndarray::Array3;
use voxels_data::{Double, Single};

mod voxels_data;

#[test]
fn user() {
    let uu = voxels_data::User::new();
    assert!(uu.active);
    assert_eq!(uu.nsd, 3);
    assert_eq!(uu.nen, 8);
    assert_eq!(uu.username, "someone123");
    assert_eq!(uu.email, "someone@example.com");
}

#[test]
fn single() {
    // item
    let ii = Single::new();
    assert_eq!(ii.segmentation, Array3::from_elem((1, 1, 1), 1));
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    // lattice
    let ll = Single::gold_lattice();
    assert_eq!(ll, [[1, 2, 4, 3, 5, 6, 8, 7]]);
    // element
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
    assert_eq!(ee, [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]);
}

/// Adds two numbers together.  Why doesn't this appear in the documentation?
///
/// # Examples
///
/// ```rust
/// let result = chad_add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// ```rust,no_run
/// // This example will not be run as a test
/// let result = chad_add(2, 3);
/// println!("The result is {}", result);
/// ```
pub fn chad_add(a: i32, b: i32) -> i32 {
    a + b
}
