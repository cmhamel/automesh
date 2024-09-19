# Spheres

We segment a sphere into coarse voxel meshes.

## Segmentation

Using [spheres.py](spheres.py),

```python
<!-- cmdrun cat spheres.py -->
```

create very coarse spheres of varying
resolution (`radius=1`, `radius=3`, and `radius=5`), as shown below:

![spheres.png](spheres.png)

For the `radius=1` case, the underyling data structure appears as:

```python
spheres["sphere_1"]

array([[[0, 0, 0],
        [0, 1, 0],
        [0, 0, 0]],

       [[0, 1, 0],
        [1, 1, 1],
        [0, 1, 0]],

       [[0, 0, 0],
        [0, 1, 0],
        [0, 0, 0]]], dtype=uint8)
```

For the `radius=3` case, the underyling data structure appears as:

```python
spheres["sphere_3"]

array([[[0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0]],

       [[0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0]],

       [[0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0]],

       [[0, 0, 0, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 1, 0, 0, 0]],

       [[0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0]],

       [[0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0]],

       [[0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0]]], dtype=uint8)
```

Because of the large size of `sphere_5`, its data structure is not shown
here.

These data structures are saved to

* [spheres_sphere_1.npy](spheres_sphere_1.npy)
* [spheres_sphere_3.npy](spheres_sphere_3.npy)
* [spheres_sphere_5.npy](spheres_sphere_5.npy)

## Autotwin

```sh
cargo run -- -i book/examples/spheres/spheres_sphere_1.npy -o book/examples/spheres/spheres_sphere_1.inp -x 3 -y 3 -z 3
```

```sh
cargo run -- -i book/examples/spheres/spheres_sphere_3.npy -o book/examples/spheres/spheres_sphere_3.inp -x 7 -y 7 -z 7
```

```sh
cargo run -- -i book/examples/spheres/spheres_sphere_5.npy -o book/examples/spheres/spheres_sphere_5_.inp -x 11 -y 11 -z 11
```

## Mesh

The `spheres_sphere_1.inp` file:

```sh
<!-- cmdrun cat spheres_sphere_1.inp -->
```

The `spheres_sphere_3.inp` file:

```sh
<!-- cmdrun cat spheres_sphere_3.inp -->
```

The `spheres_sphere_5.inp` file:

```sh
<!-- cmdrun cat spheres_sphere_5.inp -->
```
