# Introduction

Hello world!


## Conventions

We use the Exodus II convention for a hexahedral element
local node numbering:

![exodus_hex_numbering_scheme](fig/exodus_hex_numbering_scheme.png)
> Figure: Exodus II number scheme, taken from Schoof[^Schoof-1994].

## Testing

### Single

*The First Minimum Working Example*

The minimum working example (MWE) is a single voxel, used to create a single
mesh consisting of one block consisting of a single element.  The NumPy
input [single.npy](../tests/input/single.npy) contains the following
segmentation:

```bash
segmentation = np.array(
    [
        [
            [
                11,
            ],
        ],
    ],
    dtype=np.uint8,
)
```

where the segmenetation ID, `11`, will denote block `11` in the finite element
mesh.

Equivalently, the [single.spn](../tests/input/single.spn) contains a
single integer:

```bash
11
```

The resulting finite element mesh is visualized is shown in the following
figure:

![single.png](fig/single.png)
> Figure: The `single.png` visualization.

## References

[^Schoof-1994]: Schoof LA, Yarberry VR. EXODUS II: a finite element data model. Sandia National Lab.(SNL-NM), Albuquerque, NM (United States); 1994 Sep 1. [link](https://www.osti.gov/biblio/10102115)
