# Spheres - Continued

Now we use the fundamentals learned in the [previous example](../spheres/README.md) to a more sophisticated example:  Concentric, high-resolution spheres consisting of three materials.

## Problem Statement

Given three concentric spheres of radius 10, 11, and 12 cm, as shown in the figure below, create finite element meshes of the following resolutions:

1. One voxel per centimeter (element side length 1.0 cm),
2. Two voxels per centimeter (element side length 0.5 cm),
3. Four voxels per centimeter (element side length 0.25 cm), and
4. Ten voxels per centimeter (element side length 0.1 cm).

![spheres_cont_dim](spheres_cont_dim.png)

Figure: Three concentric spheres of radius 10, 11, and 12 cm.

## Solution

We use Python to create segmentations, then `autotwin` to convert the segmentations into finite element meshes.

### Python Segmentation

![spheres_cont](spheres_cont.png)


### Autotwin

```sh
automesh -i spheres_resolution_1.npy -o spheres_resolution_1.inp -x 24 -y 24 -z 24 -xtranslate -12 -ytranslate -12 -ztranslate -12

automesh -i spheres_resolution_1.npy -o spheres_resolution_1.inp -x 24 -y 24 -z 24 --xtranslate 24 --ytranslate 24 --ztranslate 24
```

But translating in a negative direction seems to be error prone:

```sh
automesh -i spheres_resolution_1.npy -o spheres_resolution_1.inp -x 24 -y 24 -z 24 --xtranslate -24 --ytranslate 24 --ztranslate 24

error: unexpected argument '-2' found
```


### Meshes
