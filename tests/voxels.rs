//! Voxels testing:
//!
//! This module tests convertion from a segmentation to a lattice and a mesh.
//!

use voxels_data::{Cube, Double, DoubleY, LetterF, Quadruple, QuadrupleVoid, Single, Triple};

mod voxels_data;

#[test]
fn single() {
    // item
    let ii = Single::new();
    assert_eq!(ii.segmentation, [[[1]]]);
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
    assert_eq!(ii.segmentation, [[[1, 1]]]);
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = Double::gold_lattice();
    assert_eq!(ll, [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]);
    let ee = Double::gold_elements();
    assert_eq!(ee, [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]);
}

#[test]
fn double_y() {
    let ii = DoubleY::new();
    assert_eq!(ii.segmentation, [[[1], [1]]]);
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = DoubleY::gold_lattice();
    assert_eq!(ll, [[1, 2, 4, 3, 7, 8, 10, 9], [3, 4, 6, 5, 9, 10, 12, 11]]);
    let ee = DoubleY::gold_elements();
    assert_eq!(ee, [[1, 2, 4, 3, 7, 8, 10, 9], [3, 4, 6, 5, 9, 10, 12, 11]]);
}

#[test]
fn triple() {
    let ii = Triple::new();
    assert_eq!(ii.segmentation, [[[1, 1, 1]]]);
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = Triple::gold_lattice();
    assert_eq!(
        ll,
        [
            [1, 2, 6, 5, 9, 10, 14, 13],
            [2, 3, 7, 6, 10, 11, 15, 14],
            [3, 4, 8, 7, 11, 12, 16, 15],
        ]
    );
    let ee = Triple::gold_elements();
    assert_eq!(
        ee,
        [
            [1, 2, 6, 5, 9, 10, 14, 13],
            [2, 3, 7, 6, 10, 11, 15, 14],
            [3, 4, 8, 7, 11, 12, 16, 15],
        ]
    );
}

#[test]
fn quadruple() {
    let ii = Quadruple::new();
    assert_eq!(ii.segmentation, [[[1, 1, 1, 1]]]);
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = Quadruple::gold_lattice();
    assert_eq!(
        ll,
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    );
    let ee = Quadruple::gold_elements();
    assert_eq!(
        ee,
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    );
}

#[test]
fn quadruple_void() {
    let ii = QuadrupleVoid::new();
    assert_eq!(ii.segmentation, [[[1, 0, 0, 1]]]);
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = QuadrupleVoid::gold_lattice();
    assert_eq!(
        ll,
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    );
    let ee = QuadrupleVoid::gold_elements();
    assert_eq!(
        ee,
        [[1, 2, 7, 6, 11, 12, 17, 16], [4, 5, 10, 9, 14, 15, 20, 19],]
    );
}

#[test]
fn cube() {
    let ii = Cube::new();
    assert_eq!(ii.segmentation, [[[1, 1], [1, 1]], [[1, 1], [1, 1]]]);
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = Cube::gold_lattice();
    assert_eq!(
        ll,
        [
            [1, 2, 5, 4, 10, 11, 14, 13],
            [2, 3, 6, 5, 11, 12, 15, 14],
            [4, 5, 8, 7, 13, 14, 17, 16],
            [5, 6, 9, 8, 14, 15, 18, 17],
            [10, 11, 14, 13, 19, 20, 23, 22],
            [11, 12, 15, 14, 20, 21, 24, 23],
            [13, 14, 17, 16, 22, 23, 26, 25],
            [14, 15, 18, 17, 23, 24, 27, 26],
        ]
    );
    let ee = Cube::gold_elements();
    assert_eq!(
        ee,
        [
            [1, 2, 5, 4, 10, 11, 14, 13],
            [2, 3, 6, 5, 11, 12, 15, 14],
            [4, 5, 8, 7, 13, 14, 17, 16],
            [5, 6, 9, 8, 14, 15, 18, 17],
            [10, 11, 14, 13, 19, 20, 23, 22],
            [11, 12, 15, 14, 20, 21, 24, 23],
            [13, 14, 17, 16, 22, 23, 26, 25],
            [14, 15, 18, 17, 23, 24, 27, 26],
        ]
    );
}

#[test]
fn letter_f() {
    let ii = LetterF::new();
    assert_eq!(
        ii.segmentation,
        [[[1, 0, 0,], [1, 0, 0,], [1, 1, 0,], [1, 0, 0,], [1, 1, 1,],]]
    );
    assert_eq!(ii.scale, [1.0, 1.0, 1.0]);
    assert_eq!(ii.translate, [0.0, 0.0, 0.0]);
    let ll = LetterF::gold_lattice();
    assert_eq!(
        ll,
        [
            [1, 2, 6, 5, 25, 26, 30, 29],
            [2, 3, 7, 6, 26, 27, 31, 30],
            [3, 4, 8, 7, 27, 28, 32, 31],
            [5, 6, 10, 9, 29, 30, 34, 33],
            [6, 7, 11, 10, 30, 31, 35, 34],
            [7, 8, 12, 11, 31, 32, 36, 35],
            [9, 10, 14, 13, 33, 34, 38, 37],
            [10, 11, 15, 14, 34, 35, 39, 38],
            [11, 12, 16, 15, 35, 36, 40, 39],
            [13, 14, 18, 17, 37, 38, 42, 41],
            [14, 15, 19, 18, 38, 39, 43, 42],
            [15, 16, 20, 19, 39, 40, 44, 43],
            [17, 18, 22, 21, 41, 42, 46, 45],
            [18, 19, 23, 22, 42, 43, 47, 46],
            [19, 20, 24, 23, 43, 44, 48, 47],
        ]
    );
    let ee = LetterF::gold_elements();
    assert_eq!(
        ee,
        [
            [1, 2, 6, 5, 25, 26, 30, 29],
            // [2, 3, 7, 6, 26, 27, 31, 30],
            // [3, 4, 8, 7, 27, 28, 32, 31],
            [5, 6, 10, 9, 29, 30, 34, 33],
            // [6, 7, 11, 10, 30, 31, 35, 34],
            // [7, 8, 12, 11, 31, 32, 36, 35],
            [9, 10, 14, 13, 33, 34, 38, 37],
            [10, 11, 15, 14, 34, 35, 39, 38],
            // [11, 12, 16, 15, 35, 36, 40, 39],
            [13, 14, 18, 17, 37, 38, 42, 41],
            // [14, 15, 19, 18, 38, 39, 43, 42],
            // [15, 16, 20, 19, 39, 40, 44, 43],
            [17, 18, 22, 21, 41, 42, 46, 45],
            [18, 19, 23, 22, 42, 43, 47, 46],
            [19, 20, 24, 23, 43, 44, 48, 47],
        ]
    );
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
