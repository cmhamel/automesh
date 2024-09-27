# Python Interface

```py
from automesh import FiniteElements, Voxels
```

## Example

Convert a NumPy segmentation file to an Abaqus input file:

```py
from automesh import Voxels

voxels = Voxels.from_npy("single.npy")
fem = voxels.as_finite_elements()
fem.write_inp("single.inp")
```

<!-- cmdrun wget https://github.com/autotwin/automesh/raw/main/tests/input/single.npy -O ../../target/single_for_py.npy -->
<!-- cmdrun python3 -c 'from automesh import Voxels; voxels = Voxels.from_npy("../../target/single_for_py.npy"); fem = voxels.as_finite_elements(); fem.write_inp("../../target/single_from_py.inp")' -->

The resulting Abaqus input file:

```sh
<!-- cmdrun cat ../../target/single_from_py.inp -->
```
