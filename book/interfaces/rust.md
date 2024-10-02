# Rust Interface

```rust
# extern crate automesh;
use automesh::{Abaqus, FiniteElements, Voxels};
```

## Example

Convert a Numpy segmentation file to an Abaqus input file:

```rust,ignore
use automesh::{Abaqus, Voxels};

fn main() {
    let voxels = Voxels::from_npy("single.npy");
    let scale = [1.0, 1.0, 1.0];
    let translation = [0.0, 0.0, 0.0];
    let fem = voxels.into_finite_elements(&scale, &translation);
    fem.write_inp("single.inp");
}
```

The resulting Abaqus input file:

```sh
{{#include ../../target/single_from_cli.inp}}
```
