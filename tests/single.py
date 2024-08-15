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
    voxels = None
    gold_lattice = None


class Single(Example):
    """A specific example of a single voxel."""

    figure_title: str = "Single Element Global Node Numbers and Coordinates"
    file_stem: str = "single"
    voxels = np.array(
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


class Double(Example):
    """A specific example of a double voxel."""

    figure_title: str = "Double Element Global Node Numbers and Coordinates"
    file_stem: str = "double"
    voxels = np.array(
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


class DoubleY(Example):
    """A specific example of a double voxel, coursed along the y-axis."""

    figure_title: str = "Double Y Element Global Node Numbers and Coordinates"
    file_stem: str = "double_y"
    voxels = np.array(
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


class Triple(Example):
    """A specific example of a triple voxel."""

    figure_title: str = "Triple Element Global Node Numbers and Coordinates"
    file_stem: str = "triple"
    voxels = np.array(
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


class Quadruple(Example):
    """A specific example of a quadruple voxel."""

    figure_title: str = "Quadruple Element Global Node Numbers and Coordinates"
    file_stem: str = "quadruple"
    voxels = np.array(
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


class QuadrupleVoid(Example):
    """A specific example of a quadruple voxel with two of the intermediate
    voxels being void.
    """

    figure_title: str = (
        "Quadruple with Voids Element Global Node Numbers and Coordinates"
    )
    file_stem: str = "quadruple_void"
    voxels = np.array(
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


def lattice_connectivity(ex: Example) -> nt.ArrayLike:
    """Given an Example, prints the lattice connectivity."""
    offset = 0
    nz, ny, nx = ex.voxels.shape
    nzp, nyp, nxp = nz + 1, ny + 1, nx + 1
    # base = np.array([1, 2, 4, 3, 5, 6, 8, 7])

    nel = nx * ny * nz

    # Generate the lattice nodes
    lattice_nodes = []

    lattice_node = 0
    for k in range(nzp):
        for j in range(nyp):
            for i in range(nxp):
                lattice_node += 1
                lattice_nodes.append([lattice_node, i, j, k])

    # cs = np.array([[]], dtype=np.uint8)
    # cs = np.array([], dtype=np.uint8)
    cs = []

    offset = 0

    for vox in range(nel):

        i, j, k = lattice_nodes[vox][1:4]
        c = offset + np.array(
            [
                k * (nxp * nyp) + i + 1,
                k * (nxp * nyp) + i + 2,
                k * (nxp * nyp) + (j + 1) * nxp + i + 2,
                k * (nxp * nyp) + (j + 1) * nxp + i + 1,
                (k + 1) * (nxp * nyp) + i + 1,
                (k + 1) * (nxp * nyp) + i + 2,
                (k + 1) * (nxp * nyp) + (j + 1) * nxp + i + 2,
                (k + 1) * (nxp * nyp) + (j + 1) * nxp + i + 1,
            ]
        )
        # c = offset + np.array(
        #     [
        #         0 * nxp + 1,
        #         0 * nxp + 2,
        #         1 * nxp + 2,
        #         1 * nxp + 1,
        #         2 * nxp + 1,
        #         2 * nxp + 2,
        #         3 * nxp + 2,
        #         3 * nxp + 1,
        #     ]
        # )
        # cs = np.append(cs, c)
        # cs = np.concatenate(cs, c)
        cs.append(c)
        breakpoint()

    cs = np.vstack(cs)

    # voxel by voxel comparison
    vv = ex.gold_lattice == cs
    breakpoint()
    # assert np.all(vv)
    return cs


def main():
    """The main program."""

    # Create an instance of a specific example
    # user input begin
    # ex = Single()
    # ex = Double()
    ex = DoubleY()
    # ex = Triple()
    # ex = Quadruple()
    # ex = QuadrupleVoid()
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

    nelz, nely, nelx = ex.voxels.shape
    cc = lattice_connectivity(ex=ex)

    # save the numpy data as a .npy file
    np.save(output_npy, ex.voxels)
    print(f"Saved: {output_npy}")

    # to load the array back from the .npy file,
    # use the numpy.load function:
    loaded_array = np.load(output_npy)

    # verify the loaded array
    print(loaded_array)

    # assert loaded_array == ex.voxels

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

    # Label the voxels and materials

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
