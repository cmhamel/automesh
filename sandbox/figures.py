"""This module demonstrates creating a pixel slice in the (x, y)
plane, and then appending layers in the z axis, to create a 3D
voxel lattice, as a precursor for a hexahedral finite element mesh.

This module assumes the virtual environment has been loaded.

Example:

    cd ~/autotwin/automesh
    source .venv/bin/activate
    python sandbox/figures.py

Ouput:
    The `output_npy` file data structure
    The `output_png` file visualization
"""

# standard library
import datetime
from pathlib import Path
from typing import Final, NamedTuple

# third-party libary
import matplotlib.pyplot as plt
from matplotlib.colors import LightSource
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
from numpy.typing import NDArray

# module library
# import sandbox.figures_data as fd  # why doesn't this work?


def hello_world() -> str:
    """Simple example of a function hooked to a command line entry point.

    Returns:
        The canonical "Hello world!" string.
    """

    return "Hello world!"


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
    included_ids = (1,)
    gold_lattice = None
    gold_mesh_lattice_connectivity = None
    gold_mesh_element_connectivity = None


COMMON_TITLE: Final[str] = "Lattice Index and Coordinates: "


class Single(Example):
    """A specific example of a single voxel."""

    figure_title: str = COMMON_TITLE + "Single"
    file_stem: str = "single"
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
    included_ids = (11,)
    gold_lattice = ((1, 2, 4, 3, 5, 6, 8, 7),)
    gold_mesh_lattice_connectivity = (
        (
            11,
            (1, 2, 4, 3, 5, 6, 8, 7),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            11,
            (1, 2, 4, 3, 5, 6, 8, 7),
        ),
    )


class DoubleX(Example):
    """A specific example of a double voxel, coursed along the x-axis."""

    figure_title: str = COMMON_TITLE + "DoubleX"
    file_stem: str = "double_x"
    segmentation = np.array(
        [
            [
                [
                    11,
                    11,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (11,)
    gold_lattice = (
        (1, 2, 5, 4, 7, 8, 11, 10),
        (2, 3, 6, 5, 8, 9, 12, 11),
    )
    gold_mesh_lattice_connectivity = (
        (
            11,
            (1, 2, 5, 4, 7, 8, 11, 10),
            (2, 3, 6, 5, 8, 9, 12, 11),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            11,
            (1, 2, 5, 4, 7, 8, 11, 10),
            (2, 3, 6, 5, 8, 9, 12, 11),
        ),
    )


class DoubleY(Example):
    """A specific example of a double voxel, coursed along the y-axis."""

    figure_title: str = COMMON_TITLE + "DoubleY"
    file_stem: str = "double_y"
    segmentation = np.array(
        [
            [
                [
                    11,
                ],
                [
                    11,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (11,)
    gold_lattice = (
        (1, 2, 4, 3, 7, 8, 10, 9),
        (3, 4, 6, 5, 9, 10, 12, 11),
    )
    gold_mesh_lattice_connectivity = (
        (
            11,
            (1, 2, 4, 3, 7, 8, 10, 9),
            (3, 4, 6, 5, 9, 10, 12, 11),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            11,
            (1, 2, 4, 3, 7, 8, 10, 9),
            (3, 4, 6, 5, 9, 10, 12, 11),
        ),
    )


class TripleX(Example):
    """A triple voxel lattice, coursed along the x-axis."""

    figure_title: str = COMMON_TITLE + "Triple"
    file_stem: str = "triple_x"
    segmentation = np.array(
        [
            [
                [
                    11,
                    11,
                    11,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (11,)
    gold_lattice = (
        (1, 2, 6, 5, 9, 10, 14, 13),
        (2, 3, 7, 6, 10, 11, 15, 14),
        (3, 4, 8, 7, 11, 12, 16, 15),
    )
    gold_mesh_lattice_connectivity = (
        (
            11,
            (1, 2, 6, 5, 9, 10, 14, 13),
            (2, 3, 7, 6, 10, 11, 15, 14),
            (3, 4, 8, 7, 11, 12, 16, 15),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            11,
            (1, 2, 6, 5, 9, 10, 14, 13),
            (2, 3, 7, 6, 10, 11, 15, 14),
            (3, 4, 8, 7, 11, 12, 16, 15),
        ),
    )


class QuadrupleX(Example):
    """A quadruple voxel lattice, coursed along the x-axis."""

    figure_title: str = COMMON_TITLE + "Quadruple"
    file_stem: str = "quadruple_x"
    segmentation = np.array(
        [
            [
                [
                    11,
                    11,
                    11,
                    11,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (11,)
    gold_lattice = (
        (1, 2, 7, 6, 11, 12, 17, 16),
        (2, 3, 8, 7, 12, 13, 18, 17),
        (3, 4, 9, 8, 13, 14, 19, 18),
        (4, 5, 10, 9, 14, 15, 20, 19),
    )
    gold_mesh_lattice_connectivity = (
        (
            11,
            (1, 2, 7, 6, 11, 12, 17, 16),
            (2, 3, 8, 7, 12, 13, 18, 17),
            (3, 4, 9, 8, 13, 14, 19, 18),
            (4, 5, 10, 9, 14, 15, 20, 19),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            11,
            (1, 2, 7, 6, 11, 12, 17, 16),
            (2, 3, 8, 7, 12, 13, 18, 17),
            (3, 4, 9, 8, 13, 14, 19, 18),
            (4, 5, 10, 9, 14, 15, 20, 19),
        ),
    )


class Quadruple2VoidsX(Example):
    """A quadruple voxel lattice, coursed along the x-axis, with two
    intermediate voxels in the segmentation being void.
    """

    figure_title: str = COMMON_TITLE + "Quadruple2VoidsX"
    file_stem: str = "quadruple_2_voids_x"
    segmentation = np.array(
        [
            [
                [
                    99,
                    0,
                    0,
                    99,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (99,)
    gold_lattice = (
        (1, 2, 7, 6, 11, 12, 17, 16),
        (2, 3, 8, 7, 12, 13, 18, 17),
        (3, 4, 9, 8, 13, 14, 19, 18),
        (4, 5, 10, 9, 14, 15, 20, 19),
    )
    gold_mesh_lattice_connectivity = (
        (
            99,
            (1, 2, 7, 6, 11, 12, 17, 16),
            (4, 5, 10, 9, 14, 15, 20, 19),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            99,
            (1, 2, 6, 5, 9, 10, 14, 13),
            (3, 4, 8, 7, 11, 12, 16, 15),
        ),
    )


class Quadruple2Blocks(Example):
    """A quadruple voxel lattice, with the first intermediate voxel being
    the second block and the second intermediate voxel being void.
    """

    figure_title: str = COMMON_TITLE + "Quadruple2Blocks"
    file_stem: str = "quadruple_2_blocks"
    segmentation = np.array(
        [
            [
                [
                    100,
                    101,
                    101,
                    100,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (
        100,
        101,
    )
    gold_lattice = (
        (1, 2, 7, 6, 11, 12, 17, 16),
        (2, 3, 8, 7, 12, 13, 18, 17),
        (3, 4, 9, 8, 13, 14, 19, 18),
        (4, 5, 10, 9, 14, 15, 20, 19),
    )
    gold_mesh_lattice_connectivity = (
        (
            100,
            (1, 2, 7, 6, 11, 12, 17, 16),
            (4, 5, 10, 9, 14, 15, 20, 19),
        ),
        (
            101,
            (2, 3, 8, 7, 12, 13, 18, 17),
            (3, 4, 9, 8, 13, 14, 19, 18),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            100,
            (1, 2, 7, 6, 11, 12, 17, 16),
            (4, 5, 10, 9, 14, 15, 20, 19),
        ),
        (
            101,
            (2, 3, 8, 7, 12, 13, 18, 17),
            (3, 4, 9, 8, 13, 14, 19, 18),
        ),
    )


class Quadruple2BlocksVoid(Example):
    """A quadruple voxel lattice, with the first intermediate voxel being
    the second block and the second intermediate voxel being void.
    """

    figure_title: str = COMMON_TITLE + "Quadruple2BlocksVoid"
    file_stem: str = "quadruple_2_blocks_void"
    segmentation = np.array(
        [
            [
                [
                    102,
                    103,
                    0,
                    102,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (
        102,
        103,
    )
    gold_lattice = (
        (1, 2, 7, 6, 11, 12, 17, 16),
        (2, 3, 8, 7, 12, 13, 18, 17),
        (3, 4, 9, 8, 13, 14, 19, 18),
        (4, 5, 10, 9, 14, 15, 20, 19),
    )
    gold_mesh_lattice_connectivity = (
        (
            102,
            (1, 2, 7, 6, 11, 12, 17, 16),
            (4, 5, 10, 9, 14, 15, 20, 19),
        ),
        (
            103,
            (2, 3, 8, 7, 12, 13, 18, 17),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            102,
            (1, 2, 7, 6, 11, 12, 17, 16),
            (4, 5, 10, 9, 14, 15, 20, 19),
        ),
        (
            103,
            (2, 3, 8, 7, 12, 13, 18, 17),
        ),
    )


class Cube(Example):
    """A (2 x 2 x 2) voxel cube."""

    figure_title: str = COMMON_TITLE + "Cube"
    file_stem: str = "cube"
    segmentation = np.array(
        [
            [
                [
                    11,
                    11,
                ],
                [
                    11,
                    11,
                ],
            ],
            [
                [
                    11,
                    11,
                ],
                [
                    11,
                    11,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (11,)
    gold_lattice = (
        (1, 2, 5, 4, 10, 11, 14, 13),
        (2, 3, 6, 5, 11, 12, 15, 14),
        (4, 5, 8, 7, 13, 14, 17, 16),
        (5, 6, 9, 8, 14, 15, 18, 17),
        (10, 11, 14, 13, 19, 20, 23, 22),
        (11, 12, 15, 14, 20, 21, 24, 23),
        (13, 14, 17, 16, 22, 23, 26, 25),
        (14, 15, 18, 17, 23, 24, 27, 26),
    )
    gold_mesh_lattice_connectivity = (
        (
            11,
            (1, 2, 5, 4, 10, 11, 14, 13),
            (2, 3, 6, 5, 11, 12, 15, 14),
            (4, 5, 8, 7, 13, 14, 17, 16),
            (5, 6, 9, 8, 14, 15, 18, 17),
            (10, 11, 14, 13, 19, 20, 23, 22),
            (11, 12, 15, 14, 20, 21, 24, 23),
            (13, 14, 17, 16, 22, 23, 26, 25),
            (14, 15, 18, 17, 23, 24, 27, 26),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            11,
            (1, 2, 5, 4, 10, 11, 14, 13),
            (2, 3, 6, 5, 11, 12, 15, 14),
            (4, 5, 8, 7, 13, 14, 17, 16),
            (5, 6, 9, 8, 14, 15, 18, 17),
            (10, 11, 14, 13, 19, 20, 23, 22),
            (11, 12, 15, 14, 20, 21, 24, 23),
            (13, 14, 17, 16, 22, 23, 26, 25),
            (14, 15, 18, 17, 23, 24, 27, 26),
        ),
    )


class CubeMulti(Example):
    """A (2 x 2 x 2) voxel cube with two voids and six elements."""

    figure_title: str = COMMON_TITLE + "CubeMulti"
    file_stem: str = "cube_multi"
    segmentation = np.array(
        [
            [
                [
                    82,
                    2,
                ],
                [
                    2,
                    2,
                ],
            ],
            [
                [
                    0,
                    31,
                ],
                [
                    0,
                    44,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (
        82,
        2,
        31,
        44,
    )
    gold_lattice = (
        (1, 2, 5, 4, 10, 11, 14, 13),
        (2, 3, 6, 5, 11, 12, 15, 14),
        (4, 5, 8, 7, 13, 14, 17, 16),
        (5, 6, 9, 8, 14, 15, 18, 17),
        (10, 11, 14, 13, 19, 20, 23, 22),
        (11, 12, 15, 14, 20, 21, 24, 23),
        (13, 14, 17, 16, 22, 23, 26, 25),
        (14, 15, 18, 17, 23, 24, 27, 26),
    )
    gold_mesh_lattice_connectivity = (
        # (
        #   0,
        #   (10, 11, 14, 13, 19, 20, 23, 22),
        # ),
        # (
        #   0,
        #   (13, 14, 17, 16, 22, 23, 26, 25),
        (
            2,
            (2, 3, 6, 5, 11, 12, 15, 14),
            (4, 5, 8, 7, 13, 14, 17, 16),
            (5, 6, 9, 8, 14, 15, 18, 17),
        ),
        (
            31,
            (11, 12, 15, 14, 20, 21, 24, 23),
        ),
        (
            44,
            (14, 15, 18, 17, 23, 24, 27, 26),
        ),
        (
            82,
            (1, 2, 5, 4, 10, 11, 14, 13),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            2,
            (2, 3, 6, 5, 11, 12, 15, 14),
            (4, 5, 8, 7, 13, 14, 17, 16),
            (5, 6, 9, 8, 14, 15, 18, 17),
        ),
        (
            31,
            (11, 12, 15, 14, 19, 20, 22, 21),
        ),
        (
            44,
            (14, 15, 18, 17, 21, 22, 24, 23),
        ),
        (
            82,
            (1, 2, 5, 4, 10, 11, 14, 13),
        ),
    )


class LetterF(Example):
    """A minimal letter F example."""

    figure_title: str = COMMON_TITLE + "LetterF"
    file_stem: str = "letter_f"
    segmentation = np.array(
        [
            [
                [
                    11,
                    0,
                    0,
                ],
                [
                    11,
                    0,
                    0,
                ],
                [
                    11,
                    11,
                    0,
                ],
                [
                    11,
                    0,
                    0,
                ],
                [
                    11,
                    11,
                    11,
                ],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (11,)
    gold_lattice = (
        (1, 2, 6, 5, 25, 26, 30, 29),
        (2, 3, 7, 6, 26, 27, 31, 30),
        (3, 4, 8, 7, 27, 28, 32, 31),
        (5, 6, 10, 9, 29, 30, 34, 33),
        (6, 7, 11, 10, 30, 31, 35, 34),
        (7, 8, 12, 11, 31, 32, 36, 35),
        (9, 10, 14, 13, 33, 34, 38, 37),
        (10, 11, 15, 14, 34, 35, 39, 38),
        (11, 12, 16, 15, 35, 36, 40, 39),
        (13, 14, 18, 17, 37, 38, 42, 41),
        (14, 15, 19, 18, 38, 39, 43, 42),
        (15, 16, 20, 19, 39, 40, 44, 43),
        (17, 18, 22, 21, 41, 42, 46, 45),
        (18, 19, 23, 22, 42, 43, 47, 46),
        (19, 20, 24, 23, 43, 44, 48, 47),
    )
    gold_mesh_lattice_connectivity = (
        (
            11,
            (1, 2, 6, 5, 25, 26, 30, 29),
            # (2, 3, 7, 6, 26, 27, 31, 30),
            # (3, 4, 8, 7, 27, 28, 32, 31),
            (5, 6, 10, 9, 29, 30, 34, 33),
            # (6, 7, 11, 10, 30, 31, 35, 34),
            # (7, 8, 12, 11, 31, 32, 36, 35),
            (9, 10, 14, 13, 33, 34, 38, 37),
            (10, 11, 15, 14, 34, 35, 39, 38),
            # (11, 12, 16, 15, 35, 36, 40, 39),
            (13, 14, 18, 17, 37, 38, 42, 41),
            # (14, 15, 19, 18, 38, 39, 43, 42),
            # (15, 16, 20, 19, 39, 40, 44, 43),
            (17, 18, 22, 21, 41, 42, 46, 45),
            (18, 19, 23, 22, 42, 43, 47, 46),
            (19, 20, 24, 23, 43, 44, 48, 47),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            11,
            (1, 2, 4, 3, 19, 20, 22, 21),
            #
            #
            (3, 4, 6, 5, 21, 22, 24, 23),
            #
            #
            (5, 6, 9, 8, 23, 24, 27, 26),
            (6, 7, 10, 9, 24, 25, 28, 27),
            #
            (8, 9, 12, 11, 26, 27, 30, 29),
            #
            #
            (11, 12, 16, 15, 29, 30, 34, 33),
            (12, 13, 17, 16, 30, 31, 35, 34),
            (13, 14, 18, 17, 31, 32, 36, 35),
        ),
    )


class LetterF3D(Example):
    """A three dimensional variation of the letter F, in a non-standard
    orientation.
    """

    figure_title: str = COMMON_TITLE + "LetterF3D"
    file_stem: str = "letter_f_3d"
    segmentation = np.array(
        [
            [
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
            ],
            [
                [1, 0, 0, 0],
                [1, 0, 0, 0],
                [1, 1, 1, 1],
                [1, 0, 0, 0],
                [1, 1, 1, 1],
            ],
            [
                [1, 0, 0, 0],
                [1, 0, 0, 0],
                [1, 0, 0, 0],
                [1, 0, 0, 0],
                [1, 1, 1, 1],
            ],
        ],
        dtype=np.uint8,
    )
    included_ids = (1,)
    gold_lattice = (
        (1, 2, 7, 6, 31, 32, 37, 36),
        (2, 3, 8, 7, 32, 33, 38, 37),
        (3, 4, 9, 8, 33, 34, 39, 38),
        (4, 5, 10, 9, 34, 35, 40, 39),
        (6, 7, 12, 11, 36, 37, 42, 41),
        (7, 8, 13, 12, 37, 38, 43, 42),
        (8, 9, 14, 13, 38, 39, 44, 43),
        (9, 10, 15, 14, 39, 40, 45, 44),
        (11, 12, 17, 16, 41, 42, 47, 46),
        (12, 13, 18, 17, 42, 43, 48, 47),
        (13, 14, 19, 18, 43, 44, 49, 48),
        (14, 15, 20, 19, 44, 45, 50, 49),
        (16, 17, 22, 21, 46, 47, 52, 51),
        (17, 18, 23, 22, 47, 48, 53, 52),
        (18, 19, 24, 23, 48, 49, 54, 53),
        (19, 20, 25, 24, 49, 50, 55, 54),
        (21, 22, 27, 26, 51, 52, 57, 56),
        (22, 23, 28, 27, 52, 53, 58, 57),
        (23, 24, 29, 28, 53, 54, 59, 58),
        (24, 25, 30, 29, 54, 55, 60, 59),
        (31, 32, 37, 36, 61, 62, 67, 66),
        (32, 33, 38, 37, 62, 63, 68, 67),
        (33, 34, 39, 38, 63, 64, 69, 68),
        (34, 35, 40, 39, 64, 65, 70, 69),
        (36, 37, 42, 41, 66, 67, 72, 71),
        (37, 38, 43, 42, 67, 68, 73, 72),
        (38, 39, 44, 43, 68, 69, 74, 73),
        (39, 40, 45, 44, 69, 70, 75, 74),
        (41, 42, 47, 46, 71, 72, 77, 76),
        (42, 43, 48, 47, 72, 73, 78, 77),
        (43, 44, 49, 48, 73, 74, 79, 78),
        (44, 45, 50, 49, 74, 75, 80, 79),
        (46, 47, 52, 51, 76, 77, 82, 81),
        (47, 48, 53, 52, 77, 78, 83, 82),
        (48, 49, 54, 53, 78, 79, 84, 83),
        (49, 50, 55, 54, 79, 80, 85, 84),
        (51, 52, 57, 56, 81, 82, 87, 86),
        (52, 53, 58, 57, 82, 83, 88, 87),
        (53, 54, 59, 58, 83, 84, 89, 88),
        (54, 55, 60, 59, 84, 85, 90, 89),
        (61, 62, 67, 66, 91, 92, 97, 96),
        (62, 63, 68, 67, 92, 93, 98, 97),
        (63, 64, 69, 68, 93, 94, 99, 98),
        (64, 65, 70, 69, 94, 95, 100, 99),
        (66, 67, 72, 71, 96, 97, 102, 101),
        (67, 68, 73, 72, 97, 98, 103, 102),
        (68, 69, 74, 73, 98, 99, 104, 103),
        (69, 70, 75, 74, 99, 100, 105, 104),
        (71, 72, 77, 76, 101, 102, 107, 106),
        (72, 73, 78, 77, 102, 103, 108, 107),
        (73, 74, 79, 78, 103, 104, 109, 108),
        (74, 75, 80, 79, 104, 105, 110, 109),
        (76, 77, 82, 81, 106, 107, 112, 111),
        (77, 78, 83, 82, 107, 108, 113, 112),
        (78, 79, 84, 83, 108, 109, 114, 113),
        (79, 80, 85, 84, 109, 110, 115, 114),
        (81, 82, 87, 86, 111, 112, 117, 116),
        (82, 83, 88, 87, 112, 113, 118, 117),
        (83, 84, 89, 88, 113, 114, 119, 118),
        (84, 85, 90, 89, 114, 115, 120, 119),
    )
    gold_mesh_lattice_connectivity = (
        (
            1,
            (1, 2, 7, 6, 31, 32, 37, 36),
            (2, 3, 8, 7, 32, 33, 38, 37),
            (3, 4, 9, 8, 33, 34, 39, 38),
            (4, 5, 10, 9, 34, 35, 40, 39),
            (6, 7, 12, 11, 36, 37, 42, 41),
            (7, 8, 13, 12, 37, 38, 43, 42),
            (8, 9, 14, 13, 38, 39, 44, 43),
            (9, 10, 15, 14, 39, 40, 45, 44),
            (11, 12, 17, 16, 41, 42, 47, 46),
            (12, 13, 18, 17, 42, 43, 48, 47),
            (13, 14, 19, 18, 43, 44, 49, 48),
            (14, 15, 20, 19, 44, 45, 50, 49),
            (16, 17, 22, 21, 46, 47, 52, 51),
            (17, 18, 23, 22, 47, 48, 53, 52),
            (18, 19, 24, 23, 48, 49, 54, 53),
            (19, 20, 25, 24, 49, 50, 55, 54),
            (21, 22, 27, 26, 51, 52, 57, 56),
            (22, 23, 28, 27, 52, 53, 58, 57),
            (23, 24, 29, 28, 53, 54, 59, 58),
            (24, 25, 30, 29, 54, 55, 60, 59),
            (31, 32, 37, 36, 61, 62, 67, 66),
            (36, 37, 42, 41, 66, 67, 72, 71),
            (41, 42, 47, 46, 71, 72, 77, 76),
            (42, 43, 48, 47, 72, 73, 78, 77),
            (43, 44, 49, 48, 73, 74, 79, 78),
            (44, 45, 50, 49, 74, 75, 80, 79),
            (46, 47, 52, 51, 76, 77, 82, 81),
            (51, 52, 57, 56, 81, 82, 87, 86),
            (52, 53, 58, 57, 82, 83, 88, 87),
            (53, 54, 59, 58, 83, 84, 89, 88),
            (54, 55, 60, 59, 84, 85, 90, 89),
            (61, 62, 67, 66, 91, 92, 97, 96),
            (66, 67, 72, 71, 96, 97, 102, 101),
            (71, 72, 77, 76, 101, 102, 107, 106),
            (76, 77, 82, 81, 106, 107, 112, 111),
            (81, 82, 87, 86, 111, 112, 117, 116),
            (82, 83, 88, 87, 112, 113, 118, 117),
            (83, 84, 89, 88, 113, 114, 119, 118),
            (84, 85, 90, 89, 114, 115, 120, 119),
        ),
    )
    gold_mesh_element_connectivity = (
        (
            1,
            (1, 2, 7, 6, 31, 32, 37, 36),
            (2, 3, 8, 7, 32, 33, 38, 37),
            (3, 4, 9, 8, 33, 34, 39, 38),
            (4, 5, 10, 9, 34, 35, 40, 39),
            (6, 7, 12, 11, 36, 37, 42, 41),
            (7, 8, 13, 12, 37, 38, 43, 42),
            (8, 9, 14, 13, 38, 39, 44, 43),
            (9, 10, 15, 14, 39, 40, 45, 44),
            (11, 12, 17, 16, 41, 42, 47, 46),
            (12, 13, 18, 17, 42, 43, 48, 47),
            (13, 14, 19, 18, 43, 44, 49, 48),
            (14, 15, 20, 19, 44, 45, 50, 49),
            (16, 17, 22, 21, 46, 47, 52, 51),
            (17, 18, 23, 22, 47, 48, 53, 52),
            (18, 19, 24, 23, 48, 49, 54, 53),
            (19, 20, 25, 24, 49, 50, 55, 54),
            (21, 22, 27, 26, 51, 52, 57, 56),
            (22, 23, 28, 27, 52, 53, 58, 57),
            (23, 24, 29, 28, 53, 54, 59, 58),
            (24, 25, 30, 29, 54, 55, 60, 59),
            (31, 32, 37, 36, 61, 62, 64, 63),
            (36, 37, 42, 41, 63, 64, 66, 65),
            (41, 42, 47, 46, 65, 66, 71, 70),
            (42, 43, 48, 47, 66, 67, 72, 71),
            (43, 44, 49, 48, 67, 68, 73, 72),
            (44, 45, 50, 49, 68, 69, 74, 73),
            (46, 47, 52, 51, 70, 71, 76, 75),
            (51, 52, 57, 56, 75, 76, 81, 80),
            (52, 53, 58, 57, 76, 77, 82, 81),
            (53, 54, 59, 58, 77, 78, 83, 82),
            (54, 55, 60, 59, 78, 79, 84, 83),
            (61, 62, 64, 63, 85, 86, 88, 87),
            (63, 64, 66, 65, 87, 88, 90, 89),
            (65, 66, 71, 70, 89, 90, 92, 91),
            (70, 71, 76, 75, 91, 92, 94, 93),
            (75, 76, 81, 80, 93, 94, 99, 98),
            (76, 77, 82, 81, 94, 95, 100, 99),
            (77, 78, 83, 82, 95, 96, 101, 100),
            (78, 79, 84, 83, 96, 97, 102, 101),
        ),
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

    print("processing indices...")
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


def mesh_lattice_connectivity(
    ex: Example,
    lattice: np.ndarray,
) -> tuple:
    """Given an Example (in particular, the Example's voxel data structure,
    a segmentation) and the `lattice_connectivity`, create the connectivity
    for the mesh with lattice node numbers.  A voxel with a segmentation id not
    in the Example's included ids tuple is excluded from the mesh.
    """

    # segmentation = ex.segmentation.flatten().squeeze()
    segmentation = ex.segmentation.flatten()

    # breakpoint()

    # assert that the list of included ids is equal
    included_set_unordered = set(ex.included_ids)
    included_list_ordered = sorted(included_set_unordered)
    # breakpoint()
    seg_set = set(segmentation)
    for item in included_list_ordered:
        assert (
            item in seg_set
        ), f"Error: `included_ids` item {item} is not in the segmentation"

    # Create a list of finite elements from the lattice elements.  If the
    # lattice element has a segmentation id that is not in the included_ids,
    # exlude the voxel element from the collected list to create the finite
    # element list
    blocks = ()  # empty tuple
    # breakpoint()
    for bb in included_list_ordered:
        # included_elements = []
        elements = ()  # empty tuple
        elements = elements + (bb,)  # insert the block number
        for i, element in enumerate(lattice):
            if bb == segmentation[i]:
                # breakpoint()
                elements = elements + (tuple(element.tolist()),)  # overwrite

        blocks = blocks + (elements,)  # overwrite

    # breakpoint()

    # return np.array(blocks)
    return blocks


def renumber(source: tuple, old: tuple, new: tuple) -> tuple:
    """Given a source tuple, composed of a list of positive integers,
    a tuple of `old` numbers that maps into `new` numbers, return the
    source tuple with the `new` numbers."""

    # the old and the new tuples musts have the same length
    err = "Tuples `old` and `new` must have equal length."
    assert len(old) == len(new), err

    result = ()
    for item in source:
        idx = old.index(item)
        new_value = new[idx]
        result = result + (new_value,)

    return result


def mesh_element_connectivity(mesh_with_lattice_connectivity: tuple):
    """Given a mesh with lattice connectivity, return a mesh with finite
    element connectivity.
    """
    # create a list of unordered lattice node numbers
    ln = []
    for item in mesh_with_lattice_connectivity:
        # print(f"item is {item}")
        # The first item is the block number
        # block = item[0]
        # The second and onward items are the elements
        elements = item[1:]
        for element in elements:
            ln += list(element)

    ln_set = set(ln)  # sets are not necessarily ordered
    ln_ordered = tuple(sorted(ln_set))  # now these unique integers are ordered

    # and they will map into the new compressed unique interger list `mapsto`
    mapsto = tuple(range(1, len(ln_ordered) + 1))

    # now build a mesh_with_element_connectivity
    mesh = ()  # empty tuple
    # breakpoint()
    for item in mesh_with_lattice_connectivity:
        # The first item is the block number
        block_number = item[0]
        block_and_elements = ()  # empty tuple
        # insert the block number
        block_and_elements = block_and_elements + (block_number,)
        for element in item[1:]:
            new_element = renumber(source=element, old=ln_ordered, new=mapsto)
            # overwrite
            block_and_elements = block_and_elements + (new_element,)

        mesh = mesh + (block_and_elements,)  # overwrite

    return mesh


def flatten_tuple(t):
    """Uses recursion to convert nested tuples into a single-sevel tuple.

    Example:
        nested_tuple = (1, (2, 3), (4, (5, 6)), 7)
        flattened_tuple = flatten_tuple(nested_tuple)
        print(flattened_tuple)  # Output: (1, 2, 3, 4, 5, 6, 7)
    """
    flat_list = []
    for item in t:
        if isinstance(item, tuple):
            flat_list.extend(flatten_tuple(item))
        else:
            flat_list.append(item)
    # breakpoint()
    return tuple(flat_list)


def elements_without_block_ids(mesh: tuple) -> tuple:
    """Given a mesh, removes the block ids and returns only just the
    element connectivities.
    """

    aa = ()
    for item in mesh:
        bb = item[1:]
        aa = aa + bb

    return aa


def main():
    """The main program."""

    # Create an instance of a specific example
    # user input begin
    examples = [
        Single(),
        DoubleX(),
        DoubleY(),
        TripleX(),
        QuadrupleX(),
        Quadruple2VoidsX(),
        Quadruple2Blocks(),
        Quadruple2BlocksVoid(),
        Cube(),
        CubeMulti(),
        LetterF(),
        LetterF3D(),
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
        #
        # colors
        # cmap = cm.get_cmap("viridis")  # viridis colormap
        # cmap = plt.get_cmap(name="viridis")
        cmap = plt.get_cmap(name="tab10")
        # number of discrete colors
        num_colors = len(ex.included_ids)
        colors = cmap(np.linspace(0, 1, num_colors))
        # breakpoint()
        voxel_alpha: Final[float] = 0.1
        # voxel_alpha: Final[float] = 0.7
        # azimuth (deg):
        #   0 is east  (from +y-axis looking back toward origin)
        #  90 is north (from +x-axis looking back toward origin)
        # 180 is west  (from -y-axis looking back toward origin)
        # 270 is south (from -x-axis looking back toward origin)
        # elevation (deg): 0 is horizontal, 90 is vertical (+z-axis up)
        lightsource = LightSource(azdeg=325, altdeg=45)  # azimuth, elevation
        nodes_shown: Final[bool] = True
        # nodes_shown: Final[bool] = False

        # io: if the output directory does not already exist, create it
        output_path = Path(output_dir).expanduser()
        if not output_path.exists():
            print(f"Could not find existing output directory: {output_path}")
            Path.mkdir(output_path)
            print(f"Created: {output_path}")
            assert output_path.exists()

        nelz, nely, nelx = ex.segmentation.shape
        lc = lattice_connectivity(ex=ex)

        mesh_w_lattice_conn = mesh_lattice_connectivity(ex=ex, lattice=lc)
        # breakpoint()
        err = "Calculated lattice connectivity error."
        assert mesh_w_lattice_conn == ex.gold_mesh_lattice_connectivity, err

        mesh_w_element_conn = mesh_element_connectivity(mesh_w_lattice_conn)
        err = "Calcualted element connectivity error."  # overwrite
        assert mesh_w_element_conn == ex.gold_mesh_element_connectivity, err

        # save the numpy data as a .npy file
        np.save(output_npy, ex.segmentation)
        print(f"Saved: {output_npy}")

        # to load the array back from the .npy file,
        # use the numpy.load function:
        loaded_array = np.load(output_npy)

        # verify the loaded array
        print(f"segmentation loaded from saved file: {loaded_array}")

        assert np.all(loaded_array == ex.segmentation)

        # now that the .npy file has been created and verified,
        # move it to the repo at ~/autotwin/automesh/tests/input

        if not visualize:
            return

        # visualization

        # Define the dimensions of the lattice
        nxp, nyp, nzp = (nelx + 1, nely + 1, nelz + 1)

        # Create a figure and a 3D axis
        # fig = plt.figure()
        fig = plt.figure(figsize=(10, 5))  # Adjust the figure size
        # fig = plt.figure(figsize=(8, 4))  # Adjust the figure size
        # ax = fig.add_subplot(111, projection="3d")
        # figure with 1 row, 2 columns
        ax = fig.add_subplot(1, 2, 1, projection="3d")  # r1, c2, 1st subplot
        ax2 = fig.add_subplot(1, 2, 2, projection="3d")  # r1, c2, 2nd subplot

        # For 3D plotting of voxels in matplotlib, we must swap the 'x' and the
        # 'z' axes.  The original axes in the segmentation are (z, y, x) and
        # are numbered (0, 1, 2).  We want new exists as (x, y, z) and thus
        # with numbering (2, 1, 0).
        vox = np.transpose(ex.segmentation, (2, 1, 0))
        # add voxels for each of the included materials
        for i, block_id in enumerate(ex.included_ids):
            # breakpoint()
            solid = vox == block_id
            # ax.voxels(solid, facecolors=voxel_color, alpha=voxel_alpha)
            # ax.voxels(solid, facecolors=colors[i], alpha=voxel_alpha)
            ax.voxels(
                solid,
                facecolors=colors[i],
                edgecolor=colors[i],
                alpha=voxel_alpha,
                lightsource=lightsource,
            )
            # plot the same voxels on the 2nd axis
            ax2.voxels(
                solid,
                facecolors=colors[i],
                edgecolor=colors[i],
                alpha=voxel_alpha,
                lightsource=lightsource,
            )

        # breakpoint()

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
        # breakpoint()
        # ec_set = set(flatten_tuple(mesh_w_lattice_conn))  # bug!
        # bug fix:
        ec_set = set(
            flatten_tuple(elements_without_block_ids(mesh_w_lattice_conn))
        )

        # breakpoint()

        lattice_ijk = 0
        # gnn = global node number
        gnn = 0
        gnn_labels = []

        for k in range(nzp):
            for j in range(nyp):
                for i in range(nxp):
                    x.append(i)
                    y.append(j)
                    z.append(k)
                    if lattice_ijk + 1 in ec_set:
                        gnn += 1
                        xel.append(i)
                        yel.append(j)
                        zel.append(k)
                        gnn_labels.append(f" {gnn}")
                    lattice_ijk += 1
                    labels.append(f" {lattice_ijk}: ({i},{j},{k})")

        if nodes_shown:
            # Plot the lattice coordinates
            ax.scatter(
                x,
                y,
                z,
                s=20,
                facecolors="red",
                edgecolors="none",
            )

            # Label the lattice coordinates
            for n, label in enumerate(labels):
                ax.text(x[n], y[n], z[n], label, color="darkgray", fontsize=8)

            # Plot the nodes included in the finite element connectivity
            ax2.scatter(
                xel,
                yel,
                zel,
                s=30,
                facecolors="blue",
                edgecolors="blue",
            )

            # Label the global node numbers
            for n, label in enumerate(gnn_labels):
                ax2.text(
                    xel[n], yel[n], zel[n], label, color="darkblue", fontsize=8
                )

        # Set labels for the axes
        ax.set_xlabel("x")
        ax.set_ylabel("y")
        ax.set_zlabel("z")
        # repeat for the 2nd axis
        ax2.set_xlabel("x")
        ax2.set_ylabel("y")
        ax2.set_zlabel("z")

        x_ticks = list(range(nxp))
        y_ticks = list(range(nyp))
        z_ticks = list(range(nzp))

        ax.set_xticks(x_ticks)
        ax.set_yticks(y_ticks)
        ax.set_zticks(z_ticks)
        # repeat for the 2nd axis
        ax2.set_xticks(x_ticks)
        ax2.set_yticks(y_ticks)
        ax2.set_zticks(z_ticks)

        ax.set_xlim(float(x_ticks[0]), float(x_ticks[-1]))
        ax.set_ylim(float(y_ticks[0]), float(y_ticks[-1]))
        ax.set_zlim(float(z_ticks[0]), float(z_ticks[-1]))
        # repeat for the 2nd axis
        ax2.set_xlim(float(x_ticks[0]), float(x_ticks[-1]))
        ax2.set_ylim(float(y_ticks[0]), float(y_ticks[-1]))
        ax2.set_zlim(float(z_ticks[0]), float(z_ticks[-1]))

        # Set the camera view
        ax.set_aspect("equal")
        ax.view_init(elev=el, azim=az, roll=roll)
        # repeat for the 2nd axis
        ax2.set_aspect("equal")
        ax2.view_init(elev=el, azim=az, roll=roll)

        # Adjust the distance of the camera.  The default value is 10.
        # Increasing/decreasing this value will zoom in/out, respectively.
        # ax.dist = 5  # Change the distance of the camera
        # Doesn't seem to work, and the title is clipping the uppermost node
        # and lattice numbers, so suppress the titles for now.

        # Set the title
        # ax.set_title(ex.figure_title)

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
