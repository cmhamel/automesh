"""This module demonstrates creating a pixel slice in the (x, y)
plane, and a single layer in the z axis, to create a single
voxel, as a precursor for a single hexahedral finite element.

This module assumes the virtual environment has been loaded.

Example:

    cd ~/autotwin/automesh
    source .venv/bin/activate
    python tests/single.py

Ouput:
    The `output_npy` file data structure
    The `output_png` file visualization
"""

# standard library
from pathlib import Path
from typing import Final, NamedTuple

# third-party libary
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
import numpy.typing as nt
from PIL import Image

# module library
# none


class Example(NamedTuple):
    """A base class that has all of the fields required to specialize into a
    specific example."""

    figure_title: str = "Figure Title"
    file_stem: str = "filename"
    segmentation = np.array(
        [
            [
                [
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = None
    gold_elements = None
    included_ids = tuple(
        [
            1,
        ]
    )


class Single(Example):
    """A specific example of a single voxel."""

    figure_title: str = "Single Element Global Node Numbers and Coordinates"
    file_stem: str = "single"
    segmentation = np.array(
        [
            [
                [
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = np.array([[1, 2, 4, 3, 5, 6, 8, 7]])
    gold_elements = np.array([[1, 2, 4, 3, 5, 6, 8, 7]])
    included_ids = tuple(
        [
            1,
        ]
    )


class Double(Example):
    """A specific example of a double voxel."""

    figure_title: str = "Double Element Global Node Numbers and Coordinates"
    file_stem: str = "double"
    segmentation = np.array(
        [
            [
                [
                    1,
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = np.array(
        [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]
    )
    gold_elements = np.array(
        [[1, 2, 5, 4, 7, 8, 11, 10], [2, 3, 6, 5, 8, 9, 12, 11]]
    )
    included_ids = tuple(
        [
            1,
        ]
    )


class DoubleY(Example):
    """A specific example of a double voxel, coursed along the y-axis."""

    figure_title: str = "Double Y Element Global Node Numbers and Coordinates"
    file_stem: str = "double_y"
    segmentation = np.array(
        [
            [
                [
                    1,
                ],
                [
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = np.array(
        [[1, 2, 4, 3, 7, 8, 10, 9], [3, 4, 6, 5, 9, 10, 12, 11]]
    )
    gold_elements = np.array(
        [[1, 2, 4, 3, 7, 8, 10, 9], [3, 4, 6, 5, 9, 10, 12, 11]]
    )
    included_ids = tuple(
        [
            1,
        ]
    )


class Triple(Example):
    """A specific example of a triple voxel."""

    figure_title: str = "Triple Element Global Node Numbers and Coordinates"
    file_stem: str = "triple"
    segmentation = np.array(
        [
            [
                [
                    1,
                    1,
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = np.array(
        [
            [1, 2, 6, 5, 9, 10, 14, 13],
            [2, 3, 7, 6, 10, 11, 15, 14],
            [3, 4, 8, 7, 11, 12, 16, 15],
        ]
    )
    gold_elements = np.array(
        [
            [1, 2, 6, 5, 9, 10, 14, 13],
            [2, 3, 7, 6, 10, 11, 15, 14],
            [3, 4, 8, 7, 11, 12, 16, 15],
        ]
    )
    included_ids = tuple(
        [
            1,
        ]
    )


class Quadruple(Example):
    """A specific example of a quadruple voxel."""

    figure_title: str = "Quadruple Element Global Node Numbers and Coordinates"
    file_stem: str = "quadruple"
    segmentation = np.array(
        [
            [
                [
                    1,
                    1,
                    1,
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = np.array(
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    )
    gold_elements = np.array(
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    )
    included_ids = tuple(
        [
            1,
        ]
    )


class QuadrupleVoid(Example):
    """A specific example of a quadruple voxel with two of the intermediate
    segmentation being void.
    """

    figure_title: str = (
        "Quadruple with Voids Element Global Node Numbers and Coordinates"
    )
    file_stem: str = "quadruple_void"
    segmentation = np.array(
        [
            [
                [
                    1,
                    0,
                    0,
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = np.array(
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [2, 3, 8, 7, 12, 13, 18, 17],
            [3, 4, 9, 8, 13, 14, 19, 18],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    )
    gold_elements = np.array(
        [
            [1, 2, 7, 6, 11, 12, 17, 16],
            [4, 5, 10, 9, 14, 15, 20, 19],
        ]
    )
    included_ids = tuple(
        [
            1,
        ]
    )


class Cube(Example):
    """A specific example of a (2 x 2 x 2) voxel cube."""

    figure_title: str = "Cube Element Global Node Numbers and Coordinates"
    file_stem: str = "cube"
    segmentation = np.array(
        [
            [
                [
                    1,
                    1,
                ],
                [
                    1,
                    1,
                ],
            ],
            [
                [
                    1,
                    1,
                ],
                [
                    1,
                    1,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    gold_lattice = np.array(
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
    )
    gold_elements = np.array(
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
    )
    included_ids = tuple(
        [
            1,
        ]
    )


def lattice_connectivity(ex: Example) -> nt.ArrayLike:
    """Given an Example, prints the lattice connectivity."""
    offset = 0
    nz, ny, nx = ex.segmentation.shape
    nzp, nyp, nxp = nz + 1, ny + 1, nx + 1

    # Generate the lattice nodes
    lattice_nodes = []

    lattice_node = 0
    for k in range(nzp):
        for j in range(nyp):
            for i in range(nxp):
                lattice_node += 1
                lattice_nodes.append([lattice_node, i, j, k])

    # connectivity for each voxel
    cvs = []

    offset = 0

    for iz in range(nz):
        for iy in range(ny):
            for ix in range(nx):
                print(f"(ix, iy, iz) = ({ix}, {iy}, {iz})")
                cv = offset + np.array(
                    [
                        (iz + 0) * (nxp * nyp) + (iy + 0) * nxp + ix + 1,
                        (iz + 0) * (nxp * nyp) + (iy + 0) * nxp + ix + 2,
                        (iz + 0) * (nxp * nyp) + (iy + 1) * nxp + ix + 2,
                        (iz + 0) * (nxp * nyp) + (iy + 1) * nxp + ix + 1,
                        (iz + 1) * (nxp * nyp) + (iy + 0) * nxp + ix + 1,
                        (iz + 1) * (nxp * nyp) + (iy + 0) * nxp + ix + 2,
                        (iz + 1) * (nxp * nyp) + (iy + 1) * nxp + ix + 2,
                        (iz + 1) * (nxp * nyp) + (iy + 1) * nxp + ix + 1,
                    ]
                )
                cvs.append(cv)

    cs = np.vstack(cvs)

    # voxel by voxel comparison
    vv = ex.gold_lattice == cs
    assert np.all(vv)
    return cs


def element_connectivity(
    ex: Example,
    lattice: nt.ArrayLike,
) -> nt.ArrayLike:
    """Given an Example (in particular, the Example's voxel data structure,
    a segmentation) and the `lattice_connectivity`, create the connectivity
    for the finite element mesh.  A voxel with a segmentation id in the
    Example's excluded ids tuple is excluded from becoming a finite element."""

    segmentation = ex.segmentation.flatten().squeeze()
    lc = lattice

    # assert that the list of included ids is equal
    included_set = set(ex.included_ids)
    seg_set = set(segmentation)
    for item in included_set:
        assert (
            item in seg_set
        ), f"Error: `included_ids` item {item} is not in the segmentation"

    # Create a list of finite elements from the lattice elements.  If the
    # lattice element has a segmentation id that is not in the included_ids,
    # exlude the voxel element from the collected list to create the finite
    # element list
    included_elements = []
    for i, element in enumerate(lc):
        if segmentation[i] in included_set:
            included_elements.append(element)

    return np.array(included_elements)


def main():
    """The main program."""

    # Create an instance of a specific example
    # user input begin
    # ex = Single()
    # ex = Double()
    # ex = DoubleY()
    # ex = Triple()
    # ex = Quadruple()
    ex = QuadrupleVoid()
    # ex = Cube()
    # user input end

    # computation
    output_dir: Final[str] = "~/scratch"
    output_npy: Final[Path] = (
        Path(output_dir).expanduser().joinpath(ex.file_stem + ".npy")
    )

    # visualization
    visualize: bool = True  # True performs post-processing visualization
    dpi: Final[int] = 150  # resolution, dots per inch
    output_png: Final[Path] = (
        Path(output_dir).expanduser().joinpath(ex.file_stem + ".png")
    )
    # el, az, roll = 25, -115, 0
    el, az, roll = 28, -115, 0
    # el, az, roll = 60, -121, 0
    # el, az, roll = 42, -120, 0

    # io: if the output directory does not already exist, create it
    output_path = Path(output_dir).expanduser()
    if not output_path.exists():
        print(f"Could not find existing output directory: {output_path}")
        Path.mkdir(output_path)
        print(f"Created: {output_path}")
        assert output_path.exists()

    nelz, nely, nelx = ex.segmentation.shape
    lc = lattice_connectivity(ex=ex)

    ec = element_connectivity(ex=ex, lattice=lc)

    # save the numpy data as a .npy file
    np.save(output_npy, ex.segmentation)
    print(f"Saved: {output_npy}")

    # to load the array back from the .npy file,
    # use the numpy.load function:
    loaded_array = np.load(output_npy)

    # verify the loaded array
    print(loaded_array)

    # assert loaded_array == ex.segmentation

    # now that the .npy file has been created and verified,
    # move it to the repo at ~/autotwin/automesh/tests/input

    if not visualize:
        return

    # visualization

    # Define the dimensions of the lattice
    nxp, nyp, nzp = (nelx + 1, nely + 1, nelz + 1)

    # Create a figure and a 3D axis
    fig = plt.figure()
    ax = fig.add_subplot(111, projection="3d")

    # Generate the lattice points
    x = []
    y = []
    z = []
    labels = []

    lattice_number = 0
    for k in range(nzp):
        for j in range(nyp):
            for i in range(nxp):
                lattice_number += 1
                x.append(i)
                y.append(j)
                z.append(k)
                labels.append(f"{lattice_number}: ({i},{j},{k})")

    # Plot the lattice coordinates
    ax.scatter(x, y, z, c="blue", marker="o", alpha=0.5, edgecolors="none")

    # Label the lattice coordinates
    for idx, label in enumerate(labels):
        ax.text(x[idx], y[idx], z[idx], label, color="red")

    # Label the segmentation and materials

    # Set labels for the axes
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("z")

    x_ticks = list(range(nxp))
    y_ticks = list(range(nyp))
    z_ticks = list(range(nzp))

    ax.set_xticks(x_ticks)
    ax.set_yticks(y_ticks)
    ax.set_zticks(z_ticks)

    ax.set_xlim(float(x_ticks[0]), float(x_ticks[-1]))
    ax.set_ylim(float(y_ticks[0]), float(y_ticks[-1]))
    ax.set_zlim(float(z_ticks[0]), float(z_ticks[-1]))

    # Set the camera view
    ax.set_aspect("equal")
    ax.view_init(elev=el, azim=az, roll=roll)

    # Set the title
    ax.set_title(ex.figure_title)

    # Show the plot
    plt.show()

    # plt.show()
    fig.savefig(output_png, dpi=dpi)
    print(f"Saved: {output_png}")


if __name__ == "__main__":
    main()
