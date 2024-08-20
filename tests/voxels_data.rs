use ndarray::{Array2, Array3};

const NSD: usize = 3; // 3D only, not 2D or 1D
const NUM_NODES_ELEMENT: usize = 8; // linear hexahedral elements only

#[derive(Debug)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub nsd: usize,
    pub nen: usize,
}

impl User {
    // Constructor function to create a new User instance.
    pub fn new() -> Self {
        User {
            active: true,
            username: String::from("someone123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            nsd: NSD,
            nen: NUM_NODES_ELEMENT,
        }
    }
}

#[derive(Debug)]
pub struct Single {
    pub segmentation: Array3<u32>,
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

/// The single lattice and single element example.
/// ![Figure](../doc/fig/single.png)
///
/// Why doesn't this comment appear in the documentation?
///
impl Single {
    // Constructor function to create a new Single instance.
    pub fn new() -> Self {
        Single {
            segmentation: Array3::from_elem((1, 1, 1), 1),
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
        }
    }

    pub fn gold_lattice() -> [[usize; NUM_NODES_ELEMENT]; 1] {
        [[1, 2, 4, 3, 5, 6, 8, 7]]
    }

    pub fn gold_elements() -> [[usize; NUM_NODES_ELEMENT]; 1] {
        [[1, 2, 4, 3, 5, 6, 8, 7]]
    }
}

pub struct Double {
    pub segmentation: Array3<u32>,
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl Double {
    // Constructor function to create a new Double instance.
    pub fn new() -> Self {
        Double {
            segmentation: Array3::from_elem((1, 1, 2), 1),
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
        }
    }

    pub fn gold_lattice() -> [[usize; NUM_NODES_ELEMENT]; 2] {
        [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]
    }

    pub fn gold_elements() -> [[usize; NUM_NODES_ELEMENT]; 2] {
        [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]
    }
}

// The #[derive(Debug)] attribute automatically generates an
// implementation of the Debug trait for a struct or enum. The
// Debug trait is used to format a value using the {:?} formatter,
// which is useful for debugging purposes.
#[derive(Debug)]
pub struct TestCase {
    pub segmentation: Array3<u32>,
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
    // TODO: pub gold_blocks:
    // pub gold_connectivity: Array2<u32>,
    pub gold_connectivity: Array2<u32>,
    // pub gold_coordinates: Array2<f64>,
}

impl TestCase {
    // Constructor function to create a new instance
    fn new(connectivity: Array2<u32>) -> Self {
        TestCase {
            segmentation: Array3::from_elem((1, 1, 1), 1),
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
            gold_connectivity: connectivity,
        }
    }
}

// impl Default for TestCase {
//     fn default() -> Self {
//         TestCase {
//             segmentation: Array3::from_elem((1, 1, 1), 1),
//             scale: [1.0, 1.0, 1.0],
//             translate: [0.0, 0.0, 0.0],
//             gold_connectivity: array!([[1, 2, 4, 3, 5, 6, 8, 7]]),
//             gold_coordinates: [
//                 [0.0, 0.0, 0.0],
//                 [1.0, 0.0, 0.0],
//                 [0.0, 1.0, 0.0],
//                 [1.0, 1.0, 0.0],
//                 [0.0, 0.0, 1.0],
//                 [1.0, 0.0, 1.0],
//                 [0.0, 1.0, 1.0],
//                 [1.0, 1.0, 1.0],
//             ],
//         }
//     }
// }

const NELZ: usize = 4;
const NELY: usize = 5;
const NELX: usize = 3;
// const NSD: usize = 3;
const NEL: [usize; NSD] = [NELX, NELY, NELZ];
const NUM_ELEMENTS: usize = 39;
const NUM_NODES: usize = 102;
// const NUM_NODES_ELEMENT: usize = 8;
const SCALE: [f64; NSD] = [1.2, 2.3, 0.4];
const TRANSLATE: [f64; NSD] = [-0.3, 1.1, 0.5];

const GOLD_BLOCKS: [usize; NUM_ELEMENTS] = [1; NUM_ELEMENTS];
const GOLD_CONNECTIVITY: [[usize; NUM_NODES_ELEMENT]; NUM_ELEMENTS] = [
    [2, 1, 6, 7, 32, 31, 36, 37],
    [32, 31, 36, 37, 62, 61, 66, 67],
    [62, 61, 66, 67, 86, 85, 90, 91],
    [7, 6, 11, 12, 37, 36, 41, 42],
    [37, 36, 41, 42, 67, 66, 71, 72],
    [67, 66, 71, 72, 91, 90, 95, 96],
    [12, 11, 16, 17, 42, 41, 46, 47],
    [42, 41, 46, 47, 72, 71, 76, 77],
    [72, 71, 76, 77, 96, 95, 97, 98],
    [17, 16, 21, 22, 47, 46, 51, 52],
    [47, 46, 51, 52, 77, 76, 81, 82],
    [77, 76, 81, 82, 98, 97, 99, 100],
    [22, 21, 26, 27, 52, 51, 56, 57],
    [52, 51, 56, 57, 82, 81, 83, 84],
    [82, 81, 83, 84, 100, 99, 101, 102],
    [3, 2, 7, 8, 33, 32, 37, 38],
    [33, 32, 37, 38, 63, 62, 67, 68],
    [63, 62, 67, 68, 87, 86, 91, 92],
    [8, 7, 12, 13, 38, 37, 42, 43],
    [13, 12, 17, 18, 43, 42, 47, 48],
    [43, 42, 47, 48, 73, 72, 77, 78],
    [18, 17, 22, 23, 48, 47, 52, 53],
    [23, 22, 27, 28, 53, 52, 57, 58],
    [4, 3, 8, 9, 34, 33, 38, 39],
    [34, 33, 38, 39, 64, 63, 68, 69],
    [64, 63, 68, 69, 88, 87, 92, 93],
    [9, 8, 13, 14, 39, 38, 43, 44],
    [14, 13, 18, 19, 44, 43, 48, 49],
    [44, 43, 48, 49, 74, 73, 78, 79],
    [19, 18, 23, 24, 49, 48, 53, 54],
    [24, 23, 28, 29, 54, 53, 58, 59],
    [5, 4, 9, 10, 35, 34, 39, 40],
    [35, 34, 39, 40, 65, 64, 69, 70],
    [65, 64, 69, 70, 89, 88, 93, 94],
    [10, 9, 14, 15, 40, 39, 44, 45],
    [15, 14, 19, 20, 45, 44, 49, 50],
    [45, 44, 49, 50, 75, 74, 79, 80],
    [20, 19, 24, 25, 50, 49, 54, 55],
    [25, 24, 29, 30, 55, 54, 59, 60],
];
const GOLD_COORDINATES: [[f64; 3]; NUM_NODES] = [
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 2.0],
    [0.0, 0.0, 3.0],
    [0.0, 0.0, 4.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 2.0],
    [0.0, 1.0, 3.0],
    [0.0, 1.0, 4.0],
    [0.0, 2.0, 0.0],
    [0.0, 2.0, 1.0],
    [0.0, 2.0, 2.0],
    [0.0, 2.0, 3.0],
    [0.0, 2.0, 4.0],
    [0.0, 3.0, 0.0],
    [0.0, 3.0, 1.0],
    [0.0, 3.0, 2.0],
    [0.0, 3.0, 3.0],
    [0.0, 3.0, 4.0],
    [0.0, 4.0, 0.0],
    [0.0, 4.0, 1.0],
    [0.0, 4.0, 2.0],
    [0.0, 4.0, 3.0],
    [0.0, 4.0, 4.0],
    [0.0, 5.0, 0.0],
    [0.0, 5.0, 1.0],
    [0.0, 5.0, 2.0],
    [0.0, 5.0, 3.0],
    [0.0, 5.0, 4.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 0.0, 2.0],
    [1.0, 0.0, 3.0],
    [1.0, 0.0, 4.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 2.0],
    [1.0, 1.0, 3.0],
    [1.0, 1.0, 4.0],
    [1.0, 2.0, 0.0],
    [1.0, 2.0, 1.0],
    [1.0, 2.0, 2.0],
    [1.0, 2.0, 3.0],
    [1.0, 2.0, 4.0],
    [1.0, 3.0, 0.0],
    [1.0, 3.0, 1.0],
    [1.0, 3.0, 2.0],
    [1.0, 3.0, 3.0],
    [1.0, 3.0, 4.0],
    [1.0, 4.0, 0.0],
    [1.0, 4.0, 1.0],
    [1.0, 4.0, 2.0],
    [1.0, 4.0, 3.0],
    [1.0, 4.0, 4.0],
    [1.0, 5.0, 0.0],
    [1.0, 5.0, 1.0],
    [1.0, 5.0, 2.0],
    [1.0, 5.0, 3.0],
    [1.0, 5.0, 4.0],
    [2.0, 0.0, 0.0],
    [2.0, 0.0, 1.0],
    [2.0, 0.0, 2.0],
    [2.0, 0.0, 3.0],
    [2.0, 0.0, 4.0],
    [2.0, 1.0, 0.0],
    [2.0, 1.0, 1.0],
    [2.0, 1.0, 2.0],
    [2.0, 1.0, 3.0],
    [2.0, 1.0, 4.0],
    [2.0, 2.0, 0.0],
    [2.0, 2.0, 1.0],
    [2.0, 2.0, 2.0],
    [2.0, 2.0, 3.0],
    [2.0, 2.0, 4.0],
    [2.0, 3.0, 0.0],
    [2.0, 3.0, 1.0],
    [2.0, 3.0, 2.0],
    [2.0, 3.0, 3.0],
    [2.0, 3.0, 4.0],
    [2.0, 4.0, 0.0],
    [2.0, 4.0, 1.0],
    [2.0, 5.0, 0.0],
    [2.0, 5.0, 1.0],
    [3.0, 0.0, 0.0],
    [3.0, 0.0, 1.0],
    [3.0, 0.0, 2.0],
    [3.0, 0.0, 3.0],
    [3.0, 0.0, 4.0],
    [3.0, 1.0, 0.0],
    [3.0, 1.0, 1.0],
    [3.0, 1.0, 2.0],
    [3.0, 1.0, 3.0],
    [3.0, 1.0, 4.0],
    [3.0, 2.0, 0.0],
    [3.0, 2.0, 1.0],
    [3.0, 3.0, 0.0],
    [3.0, 3.0, 1.0],
    [3.0, 4.0, 0.0],
    [3.0, 4.0, 1.0],
    [3.0, 5.0, 0.0],
    [3.0, 5.0, 1.0],
];
const GOLD_DATA: [[[u8; NELX]; NELY]; NELZ] = [
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
];
