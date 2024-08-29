import numpy as np
from automesh import Voxels
from typing import NamedTuple


scale_none = [1, 1, 1]
translate_none = [0, 0, 0]

gold_data = np.array(
    [
        [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
        [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
        [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
        [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
    ]
)


def old_assert_fem_data_from_spn_eq_gold(
    file_path, gold_blocks, gold_connectivity, gold_coordinates, nel
):
    voxels = Voxels.from_spn(file_path, nel)
    fem = voxels.as_finite_elements(scale_none, translate_none)
    assert (fem.element_blocks == gold_blocks).all()
    assert (fem.element_connectivity == gold_connectivity).all()
    assert (fem.nodal_coordinates == gold_coordinates).all()


def assert_fem_data_from_spn_eq_gold(gold):
    voxels = Voxels.from_spn(gold.file_path, gold.nel)
    fem = voxels.as_finite_elements(gold.scale, gold.translate)
    assert (fem.element_blocks == gold.element_blocks).all()
    assert (fem.element_connectivity == gold.element_connectivity).all()
    assert (fem.nodal_coordinates == gold.element_coordinates).all()


class Gold:
    """A Gold class is a so-called gold standard, taken as a trusted result,
    used for testing purposes.
    """

    translate = [0.0, 0.0, 0.0]
    scale = [1.0, 1.0, 1.0]

    def __init__(
        self,
        element_blocks=None,
        element_connectivity=None,
        element_coordinates=None,
        file_path=None,
        nel=None,
    ):
        """
        Initialize a Gold object.

        Parameters
        ----------
        element_blocks : list, optional
            A list of element blocks, where each block contains elements.
            Default is None.
        element_connectivity : list of lists, optional
            A list of lists defining the connectivity of elements.
            Default is None.
        element_coordinates : list of lists, optional
            A list of lists of coordinates for each element.
            Default is None.
        file_path : str, optional
            The file path to the gold standard data.
            Default is None.
        nel : int, optional
            The number of elements. Default is None.

        Attributes
        ----------
        element_blocks : list or None
            Stores the element blocks.
        element_connectivity : list of lists or None
            Stores the connectivity of elements.
        element_coordinates : list of lists or None
            Stores the coordinates of elements.
        file_path : str or None
            Stores the file path to the gold standard data.
        nel : int or None
            Stores the number of elements.
        """
        self.element_blocks = element_blocks
        self.element_connectivity = element_connectivity
        self.element_coordinates = element_coordinates
        self.file_path = file_path
        self.nel = nel


def test_single():
    """A single voxel lattice."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[1],
            element_connectivity=[[1, 2, 4, 3, 5, 6, 8, 7]],
            element_coordinates=[
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
                [0.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
            ],
            file_path="tests/input/single.spn",
            nel=[1, 1, 1],
        )
    )


def test_double_x():
    """A double voxel lattice, coursed along the x-axis."""
    file_path = "tests/input/double.spn"
    gold_blocks = [1, 1]
    gold_connectivity = [
        [1, 2, 5, 4, 7, 8, 11, 10],
        [2, 3, 6, 5, 8, 9, 12, 11]
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [2.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
    ]
    nel = [2, 1, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_double_y():
    """A double voxel lattice, coursed along the y-axis."""
    file_path = "tests/input/double.spn"
    gold_blocks = [1, 1]
    gold_connectivity = [
        [1, 2, 4, 3, 7, 8, 10, 9],
        [3, 4, 6, 5, 9, 10, 12, 11]
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 2.0, 0.0],
        [1.0, 2.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 2.0, 1.0],
        [1.0, 2.0, 1.0],
    ]
    nel = [1, 2, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_triple_x():
    """A triple voxel lattice, coursed along the x-axis."""
    file_path = "tests/input/triple.spn"
    gold_blocks = [1, 1, 1]
    gold_connectivity = [
        [1, 2, 6, 5, 9, 10, 14, 13],
        [2, 3, 7, 6, 10, 11, 15, 14],
        [3, 4, 8, 7, 11, 12, 16, 15],
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 1.0, 0.0],
        [3.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [2.0, 0.0, 1.0],
        [3.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
        [3.0, 1.0, 1.0],
    ]
    nel = [3, 1, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_quadruple_x():
    """A quadruple voxel lattice, coursed along the x-axis."""
    file_path = "tests/input/quadruple.spn"
    gold_blocks = [1, 1, 1, 1]
    gold_connectivity = [
        [1, 2, 7, 6, 11, 12, 17, 16],
        [2, 3, 8, 7, 12, 13, 18, 17],
        [3, 4, 9, 8, 13, 14, 19, 18],
        [4, 5, 10, 9, 14, 15, 20, 19],
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [4.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 1.0, 0.0],
        [3.0, 1.0, 0.0],
        [4.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [2.0, 0.0, 1.0],
        [3.0, 0.0, 1.0],
        [4.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
        [3.0, 1.0, 1.0],
        [4.0, 1.0, 1.0],
    ]
    nel = [4, 1, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_quadruple_2_voids_x():
    """A quadruple voxel lattice, coursed along the x-axis, with two
    intermediate voxels in the segmentation being void.
    """
    file_path = "tests/input/quadruple_2_voids.spn"
    gold_blocks = [1, 1]
    gold_connectivity = [
        [1, 2, 6, 5, 9, 10, 14, 13],
        [3, 4, 8, 7, 11, 12, 16, 15],
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [4.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [3.0, 1.0, 0.0],
        [4.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [3.0, 0.0, 1.0],
        [4.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [3.0, 1.0, 1.0],
        [4.0, 1.0, 1.0],
    ]
    nel = [4, 1, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_quadruple_2_blocks():
    """A quadruple voxel lattice, with the two intermediate voxels in the
    segmentation being a second block.
    """
    file_path = "tests/input/quadruple_2_blocks.spn"
    gold_blocks = [11, 21, 21, 11]
    gold_connectivity = [
        [1, 2, 7, 6, 11, 12, 17, 16],
        [2, 3, 8, 7, 12, 13, 18, 17],
        [3, 4, 9, 8, 13, 14, 19, 18],
        [4, 5, 10, 9, 14, 15, 20, 19],
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [4.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 1.0, 0.0],
        [3.0, 1.0, 0.0],
        [4.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [2.0, 0.0, 1.0],
        [3.0, 0.0, 1.0],
        [4.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
        [3.0, 1.0, 1.0],
        [4.0, 1.0, 1.0],
    ]
    nel = [4, 1, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_quadruple_2_blocks_void():
    """A quadruple voxel lattice, with the first intermediate voxel being
    the second block and the second intermediate voxel being void."""
    file_path = "tests/input/quadruple_2_blocks_void.spn"
    gold_blocks = [11, 21, 11]
    gold_connectivity = [
        [1, 2, 7, 6, 11, 12, 17, 16],
        [2, 3, 8, 7, 12, 13, 18, 17],
        [4, 5, 10, 9, 14, 15, 20, 19],
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [4.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 1.0, 0.0],
        [3.0, 1.0, 0.0],
        [4.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [2.0, 0.0, 1.0],
        [3.0, 0.0, 1.0],
        [4.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
        [3.0, 1.0, 1.0],
        [4.0, 1.0, 1.0],
    ]
    nel = [4, 1, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_cube():
    """A (2 x 2 x 2) voxel cube."""
    file_path = "tests/input/cube.spn"
    gold_blocks = [1, 1, 1, 1, 1, 1, 1, 1]
    gold_connectivity = [
        [1, 2, 5, 4, 10, 11, 14, 13],
        [2, 3, 6, 5, 11, 12, 15, 14],
        [4, 5, 8, 7, 13, 14, 17, 16],
        [5, 6, 9, 8, 14, 15, 18, 17],
        [10, 11, 14, 13, 19, 20, 23, 22],
        [11, 12, 15, 14, 20, 21, 24, 23],
        [13, 14, 17, 16, 22, 23, 26, 25],
        [14, 15, 18, 17, 23, 24, 27, 26],
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 1.0, 0.0],
        [0.0, 2.0, 0.0],
        [1.0, 2.0, 0.0],
        [2.0, 2.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [2.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
        [0.0, 2.0, 1.0],
        [1.0, 2.0, 1.0],
        [2.0, 2.0, 1.0],
        [0.0, 0.0, 2.0],
        [1.0, 0.0, 2.0],
        [2.0, 0.0, 2.0],
        [0.0, 1.0, 2.0],
        [1.0, 1.0, 2.0],
        [2.0, 1.0, 2.0],
        [0.0, 2.0, 2.0],
        [1.0, 2.0, 2.0],
        [2.0, 2.0, 2.0],
    ]
    nel = [2, 2, 2]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


# TODO: cube_multi()
# A (2 x 2 x 2) voxel cube with two voids and six elements.


def test_letter_f():
    """A minimal letter F example."""
    file_path = "tests/input/letter_f.spn"
    gold_blocks = np.ones(8)
    gold_connectivity = [
        [1, 2, 4, 3, 19, 20, 22, 21],
        [3, 4, 6, 5, 21, 22, 24, 23],
        [5, 6, 9, 8, 23, 24, 27, 26],
        [6, 7, 10, 9, 24, 25, 28, 27],
        [8, 9, 12, 11, 26, 27, 30, 29],
        [11, 12, 16, 15, 29, 30, 34, 33],
        [12, 13, 17, 16, 30, 31, 35, 34],
        [13, 14, 18, 17, 31, 32, 36, 35],
    ]
    gold_coordinates = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 2.0, 0.0],
        [1.0, 2.0, 0.0],
        [2.0, 2.0, 0.0],
        [0.0, 3.0, 0.0],
        [1.0, 3.0, 0.0],
        [2.0, 3.0, 0.0],
        [0.0, 4.0, 0.0],
        [1.0, 4.0, 0.0],
        [2.0, 4.0, 0.0],
        [3.0, 4.0, 0.0],
        [0.0, 5.0, 0.0],
        [1.0, 5.0, 0.0],
        [2.0, 5.0, 0.0],
        [3.0, 5.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 2.0, 1.0],
        [1.0, 2.0, 1.0],
        [2.0, 2.0, 1.0],
        [0.0, 3.0, 1.0],
        [1.0, 3.0, 1.0],
        [2.0, 3.0, 1.0],
        [0.0, 4.0, 1.0],
        [1.0, 4.0, 1.0],
        [2.0, 4.0, 1.0],
        [3.0, 4.0, 1.0],
        [0.0, 5.0, 1.0],
        [1.0, 5.0, 1.0],
        [2.0, 5.0, 1.0],
        [3.0, 5.0, 1.0],
    ]
    nel = [3, 5, 1]
    old_assert_fem_data_from_spn_eq_gold(
        file_path,
        gold_blocks,
        gold_connectivity,
        gold_coordinates,
        nel,
    )


def test_letter_f_3d():
    """A three dimensional variation of the letter F, in a non-standard
    orientation."""
    gold_blocks = np.ones(39)
    gold_connectivity = np.array(
        [
            [1, 2, 7, 6, 31, 32, 37, 36],
            [2, 3, 8, 7, 32, 33, 38, 37],
            [3, 4, 9, 8, 33, 34, 39, 38],
            [4, 5, 10, 9, 34, 35, 40, 39],
            [6, 7, 12, 11, 36, 37, 42, 41],
            [7, 8, 13, 12, 37, 38, 43, 42],
            [8, 9, 14, 13, 38, 39, 44, 43],
            [9, 10, 15, 14, 39, 40, 45, 44],
            [11, 12, 17, 16, 41, 42, 47, 46],
            [12, 13, 18, 17, 42, 43, 48, 47],
            [13, 14, 19, 18, 43, 44, 49, 48],
            [14, 15, 20, 19, 44, 45, 50, 49],
            [16, 17, 22, 21, 46, 47, 52, 51],
            [17, 18, 23, 22, 47, 48, 53, 52],
            [18, 19, 24, 23, 48, 49, 54, 53],
            [19, 20, 25, 24, 49, 50, 55, 54],
            [21, 22, 27, 26, 51, 52, 57, 56],
            [22, 23, 28, 27, 52, 53, 58, 57],
            [23, 24, 29, 28, 53, 54, 59, 58],
            [24, 25, 30, 29, 54, 55, 60, 59],
            [31, 32, 37, 36, 61, 62, 64, 63],
            [36, 37, 42, 41, 63, 64, 66, 65],
            [41, 42, 47, 46, 65, 66, 71, 70],
            [42, 43, 48, 47, 66, 67, 72, 71],
            [43, 44, 49, 48, 67, 68, 73, 72],
            [44, 45, 50, 49, 68, 69, 74, 73],
            [46, 47, 52, 51, 70, 71, 76, 75],
            [51, 52, 57, 56, 75, 76, 81, 80],
            [52, 53, 58, 57, 76, 77, 82, 81],
            [53, 54, 59, 58, 77, 78, 83, 82],
            [54, 55, 60, 59, 78, 79, 84, 83],
            [61, 62, 64, 63, 85, 86, 88, 87],
            [63, 64, 66, 65, 87, 88, 90, 89],
            [65, 66, 71, 70, 89, 90, 92, 91],
            [70, 71, 76, 75, 91, 92, 94, 93],
            [75, 76, 81, 80, 93, 94, 99, 98],
            [76, 77, 82, 81, 94, 95, 100, 99],
            [77, 78, 83, 82, 95, 96, 101, 100],
            [78, 79, 84, 83, 96, 97, 102, 101],
        ]
    )
    gold_coordinates = np.array(
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [2.0, 1.0, 0.0],
            [3.0, 1.0, 0.0],
            [4.0, 1.0, 0.0],
            [0.0, 2.0, 0.0],
            [1.0, 2.0, 0.0],
            [2.0, 2.0, 0.0],
            [3.0, 2.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 3.0, 0.0],
            [1.0, 3.0, 0.0],
            [2.0, 3.0, 0.0],
            [3.0, 3.0, 0.0],
            [4.0, 3.0, 0.0],
            [0.0, 4.0, 0.0],
            [1.0, 4.0, 0.0],
            [2.0, 4.0, 0.0],
            [3.0, 4.0, 0.0],
            [4.0, 4.0, 0.0],
            [0.0, 5.0, 0.0],
            [1.0, 5.0, 0.0],
            [2.0, 5.0, 0.0],
            [3.0, 5.0, 0.0],
            [4.0, 5.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [2.0, 0.0, 1.0],
            [3.0, 0.0, 1.0],
            [4.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [2.0, 1.0, 1.0],
            [3.0, 1.0, 1.0],
            [4.0, 1.0, 1.0],
            [0.0, 2.0, 1.0],
            [1.0, 2.0, 1.0],
            [2.0, 2.0, 1.0],
            [3.0, 2.0, 1.0],
            [4.0, 2.0, 1.0],
            [0.0, 3.0, 1.0],
            [1.0, 3.0, 1.0],
            [2.0, 3.0, 1.0],
            [3.0, 3.0, 1.0],
            [4.0, 3.0, 1.0],
            [0.0, 4.0, 1.0],
            [1.0, 4.0, 1.0],
            [2.0, 4.0, 1.0],
            [3.0, 4.0, 1.0],
            [4.0, 4.0, 1.0],
            [0.0, 5.0, 1.0],
            [1.0, 5.0, 1.0],
            [2.0, 5.0, 1.0],
            [3.0, 5.0, 1.0],
            [4.0, 5.0, 1.0],
            [0.0, 0.0, 2.0],
            [1.0, 0.0, 2.0],
            [0.0, 1.0, 2.0],
            [1.0, 1.0, 2.0],
            [0.0, 2.0, 2.0],
            [1.0, 2.0, 2.0],
            [2.0, 2.0, 2.0],
            [3.0, 2.0, 2.0],
            [4.0, 2.0, 2.0],
            [0.0, 3.0, 2.0],
            [1.0, 3.0, 2.0],
            [2.0, 3.0, 2.0],
            [3.0, 3.0, 2.0],
            [4.0, 3.0, 2.0],
            [0.0, 4.0, 2.0],
            [1.0, 4.0, 2.0],
            [2.0, 4.0, 2.0],
            [3.0, 4.0, 2.0],
            [4.0, 4.0, 2.0],
            [0.0, 5.0, 2.0],
            [1.0, 5.0, 2.0],
            [2.0, 5.0, 2.0],
            [3.0, 5.0, 2.0],
            [4.0, 5.0, 2.0],
            [0.0, 0.0, 3.0],
            [1.0, 0.0, 3.0],
            [0.0, 1.0, 3.0],
            [1.0, 1.0, 3.0],
            [0.0, 2.0, 3.0],
            [1.0, 2.0, 3.0],
            [0.0, 3.0, 3.0],
            [1.0, 3.0, 3.0],
            [0.0, 4.0, 3.0],
            [1.0, 4.0, 3.0],
            [2.0, 4.0, 3.0],
            [3.0, 4.0, 3.0],
            [4.0, 4.0, 3.0],
            [0.0, 5.0, 3.0],
            [1.0, 5.0, 3.0],
            [2.0, 5.0, 3.0],
            [3.0, 5.0, 3.0],
            [4.0, 5.0, 3.0],
        ]
    )
    nel = [4, 5, 3]
    voxels = Voxels.from_spn("tests/input/letter_f_3d.spn", nel)
    fem = voxels.as_finite_elements(scale_none, translate_none)
    assert (fem.element_blocks == gold_blocks).all()
    assert (fem.element_connectivity == gold_connectivity).all()
    assert (fem.nodal_coordinates == gold_coordinates).all()


def test_from_npy():
    """
    Test that voxels can be read from a .npy file.

    This function tests the `from_npy` method of the `Voxels` class to ensure
    that it correctly reads voxel data from a .npy file. The test compares the
    data read from the file to a predefined gold standard data to verify
    correctness.

    Raises
    ------
    AssertionError
        If the voxel data read from the .npy file does not match the gold
        standard data.
    """
    voxels = Voxels.from_npy("tests/input/letter_f_3d.npy")
    assert (voxels.data == gold_data).all()


def test_write_npy():
    """
    Test that voxels can be written to a .npy file.

    This function tests the `write_npy` method of the `Voxels` class to ensure
    that it correctly writes voxel data to a .npy file. The test involves
    reading voxel data from an input .npy file, writing it to a target .npy
    file, and then reading the data back from the target file to verify that
    it matches the predefined gold standard data.

    Raises
    ------
    AssertionError
        If the voxel data read from the target .npy file does not match the
        gold standard data.
    """
    Voxels.from_npy("tests/input/letter_f_3d.npy").write_npy(
        "target/letter_f_3d.npy"
    )
    voxels = Voxels.from_npy("target/letter_f_3d.npy")
    assert (voxels.data == gold_data).all()
