//! Automatic mesh generation.
//!
//! This program converts a segmentation to a finite element mesh.
//!
//! * The segmentation is composed of non-negative integers of type `uint8`
//! saved to a binary file in [NumPy](https://numpy.org) `.npy` format.
//!  * The segmentation can be either a semantic segmentation or an instance
//! segmentation.
//!     * A semantic segmentation classifies each pixel in an image
//! to as belonging to a specific class (or category).  For example, an image
//! containing three dogs will contain two classes: `dog` and `background`.
//!     * An instance segmentation goes beyond semantic segmentation,
//! classifying pixels not only by class, but also but instance.  Unlike
//! semantic segmentation, image segmentation can classify individual instances
//! of object of the same class.  For example, an image containing three dogs
//! will contain four classes: `dog_1`, `dog_2`, `dog_3`, and `background`.
//! * The finite element mesh is saved as an Exodus finite element mesh file
//! in `.exo` format.
//!
//! # Example:
//!
//! ## Step 1
//!
//! In Python, create a 3D "letter F" with an endcap as a 3D voxelated domain.
//! Save the domain, a simple two-material semantic segmentation, as a NumPy file.
//! The integer `0` is used to denote `void` (e.g., air).  The integer `1` is
//! used to denote `solid`.
//!
//! ```Python
//! import numpy as np
//!
//! # Step 1a: Create a NumPy array
//! letter_f = np.array(
//!      [[[1, 1, 1],
//!        [1, 1, 1],
//!        [1, 1, 1],
//!        [1, 1, 1],
//!        [1, 1, 1]],
//!
//!       [[1, 1, 1],
//!        [1, 0, 0],
//!        [1, 1, 0],
//!        [1, 0, 0],
//!        [1, 0, 0]],
//!
//!       [[1, 1, 1],
//!        [1, 0, 0],
//!        [1, 1, 0],
//!        [1, 0, 0],
//!        [1, 0, 0]],
//!
//!       [[1, 1, 1],
//!        [1, 0, 0],
//!        [1, 1, 0],
//!        [1, 0, 0],
//!        [1, 0, 0]]], dtype=np.uint8)
//!
//! # Step 1b: Save the array to a .npy file
//! np.save('letter_f.npy', letter_f)
//! ```
//!
//! ![letter F voxel](../../../doc/fig/letter_f_voxel.png)
//!
//! ## Step 2
//! On the command line, convert the `.npy` file to an `.exo` file.
//!
//! ```bash
//! automesh --input letter_f.npy --output letter_f.exo
//! ```
//!
//! The program also works directly on `.spn files`, e.g.,
//!
//! ```bash
//! # TODO: work in progress
//! automesh --input letter_f.spn -nx 3 -ny 5 -nz 4 --output letter_f.exo
//! ```
//!
//! with the default `z`, `y`, `x` ordering.
//!
//! # See Also
//!
//! The automesh online help:
//!
//! ```bash
//! automesh --help
//! ```

#[cfg(feature = "python")]
mod py;

mod exodus;
mod spn;

pub use exodus::Exodus;
pub use spn::Spn;

type ElementBlocks = Vec<usize>;
type ElementConnectivity = Vec<Vec<usize>>;
type NodalCoordinates = Vec<Vec<f64>>;
