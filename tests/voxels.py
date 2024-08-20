"""This module demonstrates creating a pixel slice in the (x, y)
plane, and then appending layers in the z axis, to create a 3D
voxel lattice, as a precursor for a hexahedral finite element mesh.

This module assumes the virtual environment has been loaded.

Example:

    cd ~/autotwin/automesh
    source .venv/bin/activate
    python tests/voxels.py
    # python src/tests/voxels.py

Ouput:
    The `output_npy` file data structure
    The `output_png` file visualization
"""

# standard library
import datetime
from pathlib import Path
from typing import Final, Iterable, NamedTuple

# third-party libary
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
from numpy.typing import NDArray
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


COMMON_TITLE: Final[str] = "Lattice Index and Coordinates: "


class Single(Example):
    """A specific example of a single voxel."""

    figure_title: str = COMMON_TITLE + "Single"
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

    figure_title: str = COMMON_TITLE + "Double"
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

    figure_title: str = COMMON_TITLE + "DoubleY"
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

    figure_title: str = COMMON_TITLE + "Triple"
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

    figure_title: str = COMMON_TITLE + "Quadruple"
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

    figure_title: str = COMMON_TITLE + "QuadrupleVoid"
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

    figure_title: str = COMMON_TITLE + "Cube"
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


class LetterF(Example):
    """A minimal letter F example."""

    figure_title: str = COMMON_TITLE + "LetterF"
    file_stem: str = "letter_f"
    segmentation = np.array(
        [
            [
                [
                    1,
                    0,
                    0,
                ],
                [
                    1,
                    0,
                    0,
                ],
                [
                    1,
                    1,
                    0,
                ],
                [
                    1,
                    0,
                    0,
                ],
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
            [1, 2, 6, 5, 25, 26, 30, 29],
            [2, 3, 7, 6, 26, 27, 31, 30],
            [3, 4, 8, 7, 27, 28, 32, 31],
            [5, 6, 10, 9, 29, 30, 34, 33],
            [6, 7, 11, 10, 30, 31, 35, 34],
            [7, 8, 12, 11, 31, 32, 36, 35],
            [9, 10, 14, 13, 33, 34, 38, 37],
            [10, 11, 15, 14, 34, 35, 39, 38],
            [11, 12, 16, 15, 35, 36, 40, 39],
            [13, 14, 18, 17, 37, 38, 42, 41],
            [14, 15, 19, 18, 38, 39, 43, 42],
            [15, 16, 20, 19, 39, 40, 44, 43],
            [17, 18, 22, 21, 41, 42, 46, 45],
            [18, 19, 23, 22, 42, 43, 47, 46],
            [19, 20, 24, 23, 43, 44, 48, 47],
        ]
    )
    gold_elements = np.array(
        [
            [1, 2, 6, 5, 25, 26, 30, 29],
            # [2, 3, 7, 6, 26, 27, 31, 30],
            # [3, 4, 8, 7, 27, 28, 32, 31],
            [5, 6, 10, 9, 29, 30, 34, 33],
            # [6, 7, 11, 10, 30, 31, 35, 34],
            # [7, 8, 12, 11, 31, 32, 36, 35],
            [9, 10, 14, 13, 33, 34, 38, 37],
            [10, 11, 15, 14, 34, 35, 39, 38],
            # [11, 12, 16, 15, 35, 36, 40, 39],
            [13, 14, 18, 17, 37, 38, 42, 41],
            # [14, 15, 19, 18, 38, 39, 43, 42],
            # [15, 16, 20, 19, 39, 40, 44, 43],
            [17, 18, 22, 21, 41, 42, 46, 45],
            [18, 19, 23, 22, 42, 43, 47, 46],
            [19, 20, 24, 23, 43, 44, 48, 47],
        ]
    )
    included_ids = tuple(
        [
            1,
        ]
    )


def lattice_connectivity(ex: Example) -> NDArray[np.uint8]:
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
    lattice: np.ndarray,
) -> NDArray[np.uint8]:
    """Given an Example (in particular, the Example's voxel data structure,
    a segmentation) and the `lattice_connectivity`, create the connectivity
    for the finite element mesh.  A voxel with a segmentation id in the
    Example's excluded ids tuple is excluded from becoming a finite element."""

    # segmentation = ex.segmentation.flatten().squeeze()
    segmentation = ex.segmentation.flatten()

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
    for i, element in enumerate(lattice):
        if segmentation[i] in included_set:
            included_elements.append(element)

    return np.array(included_elements)


def element_edges(connectivity: np.ndarray):
    """Given a single finite element connectivity, returns a tuple of the
    eight edges of that element.  The li"""

    cc = connectivity

    # edges = [pairwise_circular(x) for x in connectivity]
    # bottom edges
    e1 = (cc[0], cc[1])
    e2 = (cc[1], cc[2])
    e3 = (cc[2], cc[3])
    e4 = (cc[3], cc[0])
    # top edges
    e5 = (cc[4], cc[5])
    e6 = (cc[5], cc[6])
    e7 = (cc[6], cc[7])
    e8 = (cc[7], cc[4])
    # vertical edges
    e9 = (cc[0], cc[4])
    e10 = (cc[1], cc[5])
    e11 = (cc[2], cc[6])
    e12 = (cc[3], cc[7])

    aa = tuple([e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12])
    return aa


def test_element_edges(element: np.array):
    """Tests the element edges functionality."""
    element = np.array([1, 2, 4, 3, 5, 6, 8, 7])

    result = element_edges(element)

    #     7-----8
    #    /|    /|
    #   5-----6 |
    #   | |   | |
    #   | 3---|-4
    #   |/    |/
    #   1-----2
    #  (1, 2), (1, 3), (1, 5), (2, 4), (2, 6), (3, 4)
    #  (3, 7), (4, 8), (5, 6), (5, 7), (6, 8), (7, 8)

    assert result == (
        (1, 2),  # e1
        (1, 3),  # e4
        (1, 5),  # e9
        (2, 4),  # e2
        (2, 6),  # e10
        (3, 4),  # e3
        (3, 7),  # e12
        (4, 8),  # e11
        (5, 6),  # e5
        (5, 7),  # e8
        (6, 8),  # e6
        (7, 8),  # e7
    )


def main():
    """The main program."""

    # Create an instance of a specific example
    # user input begin
    examples = [
        Single(),
        Double(),
        DoubleY(),
        Triple(),
        Quadruple(),
        QuadrupleVoid(),
        Cube(),
        LetterF(),
    ]
    for ex in examples:

        # computation
        output_dir: Final[str] = "~/scratch"
        output_npy: Final[Path] = (
            Path(output_dir).expanduser().joinpath(ex.file_stem + ".npy")
        )

        # visualization
        visualize: bool = True  # True performs post-processing visualization
        dpi: Final[int] = 300  # resolution, dots per inch
        output_png_short = ex.file_stem + ".png"
        output_png: Final[Path] = (
            Path(output_dir).expanduser().joinpath(output_png_short)
        )
        # el, az, roll = 25, -115, 0
        # el, az, roll = 28, -115, 0
        el, az, roll = 63, -110, 0
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

        assert np.all(
            ec == ex.gold_elements
        ), "Calculated element connectivity error."

        # TODO: Optimization: construct edges and draw each edge only
        # once.
        # edges = [element_edges(connectivity=x) for x in ec]

        # save the numpy data as a .npy file
        np.save(output_npy, ex.segmentation)
        print(f"Saved: {output_npy}")

        # to load the array back from the .npy file,
        # use the numpy.load function:
        loaded_array = np.load(output_npy)

        # verify the loaded array
        print(loaded_array)

        assert np.all(loaded_array == ex.segmentation)

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

        # Generate the element points
        xel = []
        yel = []
        zel = []
        # generate a set from the element connectivity
        ec_set = set(ec.flatten())

        lattice_ijk = 0
        for k in range(nzp):
            for j in range(nyp):
                for i in range(nxp):
                    x.append(i)
                    y.append(j)
                    z.append(k)
                    if lattice_ijk + 1 in ec_set:
                        xel.append(i)
                        yel.append(j)
                        zel.append(k)
                    lattice_ijk += 1
                    labels.append(f" {lattice_ijk}: ({i},{j},{k})")

        # Plot the lattice coordinates
        ax.scatter(
            x,
            y,
            z,
            s=10,
            facecolors="blue",
            edgecolors="none",
        )

        # Label the lattice coordinates
        for idx, label in enumerate(labels):
            # ax.text(x[idx], y[idx], z[idx], label, color="darkgray",
            # fontsize=8)
            ax.text(x[idx], y[idx], z[idx], label, color="darkgray")

        # Plot the nodes included in the finite element connectivity
        ax.scatter(
            xel,
            yel,
            zel,
            s=50,
            facecolors="none",
            edgecolors="blue",
        )

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

        # Add a footnote
        # Get the current date and time in UTC
        now_utc = datetime.datetime.now(datetime.UTC)
        # Format the date and time as a string
        timestamp_utc = now_utc.strftime("%Y-%m-%d %H:%M:%S UTC")
        fn = f"Figure: {output_png_short} "
        fn += f"created with {__file__}\non {timestamp_utc}."
        fig.text(0.5, 0.01, fn, ha="center", fontsize=8)

        # Show the plot
        plt.show()

        # plt.show()
        fig.savefig(output_png, dpi=dpi)
        print(f"Saved: {output_png}")


if __name__ == "__main__":
    main()
