use ndarray::Array3;

const NSD: usize = 3; // 3D only, not 2D or 1D
const NUM_NODES_ELEMENT: usize = 8; // linear hexahedral elements only

// The #[derive(Debug)] attribute automatically generates an
// implementation of the Debug trait for a struct or enum. The
// Debug trait is used to format a value using the {:?} formatter,
// which is useful for debugging purposes.
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

/// The Single lattice and element example.
/// ![Figure](../doc/fig/single.png)
///
/// Why doesn't this comment appear in the documentation?
///
#[derive(Debug)]
pub struct Single {
    pub segmentation: [[[u8; 1]; 1]; 1], // [[[u8; NELX]; NELY]; NELZ]
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl Single {
    // Constructor function to create a new Single instance.
    pub fn new() -> Self {
        Single {
            segmentation: [[[1]]],
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

/// The Double lattice and element example.
/// ![Figure](../doc/fig/double.png)
#[derive(Debug)]
pub struct Double {
    pub segmentation: [[[u8; 2]; 1]; 1], // [[[u8; NELX]; NELY]; NELZ]
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl Double {
    // Constructor function to create a new Double instance.
    pub fn new() -> Self {
        Double {
            segmentation: [[[1, 1]]],
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

/// The DoubleY lattice and element example.
/// ![Figure](../doc/fig/double_y.png)
#[derive(Debug)]
pub struct DoubleY {
    pub segmentation: [[[u8; 1]; 2]; 1], // [[[u8; NELX]; NELY]; NELZ]
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl DoubleY {
    // Constructor function to create a new DoubleY instance.
    pub fn new() -> Self {
        DoubleY {
            segmentation: [[[1], [1]]],
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
        }
    }

    pub fn gold_lattice() -> [[usize; NUM_NODES_ELEMENT]; 2] {
        [[1, 2, 4, 3, 7, 8, 10, 9], [3, 4, 6, 5, 9, 10, 12, 11]]
    }

    pub fn gold_elements() -> [[usize; NUM_NODES_ELEMENT]; 2] {
        [[1, 2, 4, 3, 7, 8, 10, 9], [3, 4, 6, 5, 9, 10, 12, 11]]
    }
}

/// The Triple lattice and element example.
/// ![Figure](../doc/fig/triple.png)
#[derive(Debug)]
pub struct Triple {
    pub segmentation: [[[u8; 3]; 1]; 1], // [[[u8; NELX]; NELY]; NELZ]
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl Triple {
    // Constructor function to create a new Triple instance.
    pub fn new() -> Self {
        Triple {
            segmentation: [[[1, 1, 1]]],
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
        }
    }
    pub fn gold_lattice() -> [[usize; NUM_NODES_ELEMENT]; 3] {
        [
            [1, 2, 6, 5, 9, 10, 14, 13],
            [2, 3, 7, 6, 10, 11, 15, 14],
            [3, 4, 8, 7, 11, 12, 16, 15],
        ]
    }

    pub fn gold_elements() -> [[usize; NUM_NODES_ELEMENT]; 3] {
        [
            [1, 2, 6, 5, 9, 10, 14, 13],
            [2, 3, 7, 6, 10, 11, 15, 14],
            [3, 4, 8, 7, 11, 12, 16, 15],
        ]
    }
}

// The Quadruple lattice and element example.
/// ![Figure](../doc/fig/quadruple.png)
#[derive(Debug)]
pub struct Quadruple {
    pub segmentation: [[[u8; 4]; 1]; 1], // [[[u8; NELX]; NELY]; NELZ]
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl Quadruple {
    // Constructor function to create a new Quadruple instance.
    pub fn new() -> Self {
        Quadruple {
            segmentation: [[[1, 1, 1, 1]]],
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
        }
    }
    pub fn gold_lattice() -> [[usize; NUM_NODES_ELEMENT]; 4] {
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    }

    pub fn gold_elements() -> [[usize; NUM_NODES_ELEMENT]; 4] {
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    }
}

/// The QuadrupleVoid lattice and element example.
/// ![Figure](../doc/fig/quadruple_void.png)
#[derive(Debug)]
pub struct QuadrupleVoid {
    pub segmentation: [[[u8; 4]; 1]; 1], // [[[u8; NELX]; NELY]; NELZ]
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl QuadrupleVoid {
    // Constructor function to create a new QuadrupleVoid instance.
    pub fn new() -> Self {
        QuadrupleVoid {
            segmentation: [[[1, 0, 0, 1]]],
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
        }
    }
    pub fn gold_lattice() -> [[usize; NUM_NODES_ELEMENT]; 4] {
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    }

    pub fn gold_elements() -> [[usize; NUM_NODES_ELEMENT]; 2] {
        [[1, 2, 7, 6, 11, 12, 17, 16], [4, 5, 10, 9, 14, 15, 20, 19]]
    }
}

/// The Cube lattice and element example.
/// ![Figure](../doc/fig/cube.png)
#[derive(Debug)]
pub struct Cube {
    pub segmentation: Array3<u32>,
    pub scale: [f64; NSD],
    pub translate: [f64; NSD],
}

impl Cube {
    // Constructor function to create a new Cube instance.
    pub fn new() -> Self {
        Cube {
            segmentation: Array3::from_elem((1, 2, 2), 1),
            scale: [1.0, 1.0, 1.0],
            translate: [0.0, 0.0, 0.0],
        }
    }
    pub fn gold_lattice() -> [[usize; NUM_NODES_ELEMENT]; 8] {
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
    }

    pub fn gold_elements() -> [[usize; NUM_NODES_ELEMENT]; 8] {
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
    }
}
