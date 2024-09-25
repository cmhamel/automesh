# Spheres - Continued

Now we use the fundamentals learned in the [previous example](../spheres/README.md) to a more sophisticated example:  Concentric, high-resolution spheres consisting of three materials.

## Problem Statement

### Given

Given three concentric spheres of radius 10, 11, and 12 cm, as shown in the figure below:

![spheres_cont_dim](spheres_cont_dim.png)

Figure: Three concentric spheres of radius 10, 11, and 12 cm.  Grid spacing is 1 cm.

### Find

Create finite element meshes of the following resolutions:

resolution (vox/cm) | element side length (cm) | `nelx` | # voxels
---: | :---: | ---: | ---:
1 | 1.0 | 24 | 13,824
2 | 0.5 | 48 | 110,592
4 | 025 | 96 | 884,736
10 | 0.1 | 240 | 13,824,000

## Solution

We use Python to create segmentations, then `Autotwin` to convert the segmentations into finite element meshes.

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
