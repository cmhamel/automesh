import numpy as np
from automesh import Voxels


gold_data = np.array(
    [
        [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
        [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
        [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
        [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
    ]
)


def assert_fem_data_from_spn_eq_gold(gold):
    """
    Asserts that the finite element method (FEM) data derived from a given
    SPN file matches the expected 'gold' standard data.

    This function performs the following steps:
    1. Creates a `Voxels` object from the SPN file specified in
       `gold.file_path` and `gold.nel`.
    2. Converts the `Voxels` object to finite elements using the scale and
       translation parameters from `gold`.
    3. Asserts that the element blocks, element connectivity, and nodal
       coordinates of the FEM data match the corresponding attributes in the
       `gold` object.

    Parameters:
    gold (object): An object containing the following attributes:
        - file_path (str): Path to the SPN file.
        - nel (int): Number of elements.
        - scale (float): Scaling factor for the finite elements.
        - translate (tuple): Translation vector for the finite elements.
        - element_blocks (numpy.ndarray): Expected element blocks.
        - element_connectivity (numpy.ndarray): Expected element connectivity.
        - element_coordinates (numpy.ndarray): Expected nodal coordinates.

    Raises:
    AssertionError: If any of the FEM data attributes do not match the
      corresponding 'gold' standard attributes.
    """
    voxels = Voxels.from_spn(gold.file_path, gold.nel)
    fem = voxels.as_finite_elements(gold.scale, gold.translate)
    assert (fem.element_blocks == gold.element_blocks).all()
    assert (fem.element_connectivity == gold.element_connectivity).all()
    assert (fem.nodal_coordinates == gold.element_coordinates).all()


class Gold:
    """A Gold class is a so-called gold standard, taken as a trusted result,
    used for testing purposes.
    """

    def __init__(
        self,
        element_blocks=None,
        element_connectivity=None,
        element_coordinates=None,
        file_path=None,
        nel=None,
        scale=[1.0, 1.0, 1.0],
        translate=[0.0, 0.0, 0.0],
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
        scale : list of three floats, optional
            The x, y, z scaling of the element coordinates.
            Default is [1.0, 1.0, 1.0].
        translate: list of three floats, optional
            The x, y, z translation of the element coordinates.
            Default is [0.0, 0.0, 0.0].

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
        scale : list of floats or None
            Stores the x, y, z scaling factors.
        translate: list of floats or None
            Stores the x, y, z translation deltas.
        """
        self.element_blocks = element_blocks
        self.element_connectivity = element_connectivity
        self.element_coordinates = element_coordinates
        self.file_path = file_path
        self.nel = nel
        self.scale = scale
        self.translate = translate


def test_single():
    """A single voxel lattice."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11],
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


def test_single_scaled_up():
    """A single voxel lattice scaled up [x, y, z] amount [10.0, 20.0, 30.0]."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11],
            element_connectivity=[[1, 2, 4, 3, 5, 6, 8, 7]],
            element_coordinates=[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 20.0, 0.0],
                [10.0, 20.0, 0.0],
                [0.0, 0.0, 30.0],
                [10.0, 0.0, 30.0],
                [0.0, 20.0, 30.0],
                [10.0, 20.0, 30.0],
            ],
            file_path="tests/input/single.spn",
            nel=[1, 1, 1],
            scale=[10.0, 20.0, 30.0],
        )
    )


def test_single_scaled_down():
    """A single voxel lattice scaled down [x, y, z] amount
    [0.5, 0.25, 0.125].
    """
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11],
            element_connectivity=[[1, 2, 4, 3, 5, 6, 8, 7]],
            element_coordinates=[
                [0.0, 0.0, 0.0],
                [0.5, 0.0, 0.0],
                [0.0, 0.25, 0.0],
                [0.5, 0.25, 0.0],
                [0.0, 0.0, 0.125],
                [0.5, 0.0, 0.125],
                [0.0, 0.25, 0.125],
                [0.5, 0.25, 0.125],
            ],
            file_path="tests/input/single.spn",
            nel=[1, 1, 1],
            scale=[0.5, 0.25, 0.125],
        )
    )


def test_single_translated_positive():
    """A single voxel lattice translated [x, y, z] amount [0.3, 0.6, 0.9]."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11],
            element_connectivity=[[1, 2, 4, 3, 5, 6, 8, 7]],
            element_coordinates=[
                [0.3, 0.6, 0.9],
                [1.3, 0.6, 0.9],
                [0.3, 1.6, 0.9],
                [1.3, 1.6, 0.9],
                [0.3, 0.6, 1.9],
                [1.3, 0.6, 1.9],
                [0.3, 1.6, 1.9],
                [1.3, 1.6, 1.9],
            ],
            file_path="tests/input/single.spn",
            nel=[1, 1, 1],
            translate=[0.3, 0.6, 0.9],
        )
    )


def test_single_translated_negative():
    """A single voxel lattice translated [x, y, z] amount
    [-1.0, -2.0, -3.0].
    """
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11],
            element_connectivity=[[1, 2, 4, 3, 5, 6, 8, 7]],
            element_coordinates=[
                [-1.0, -2.0, -3.0],
                [0.0, -2.0, -3.0],
                [-1.0, -1.0, -3.0],
                [0.0, -1.0, -3.0],
                [-1.0, -2.0, -2.0],
                [0.0, -2.0, -2.0],
                [-1.0, -1.0, -2.0],
                [0.0, -1.0, -2.0],
            ],
            file_path="tests/input/single.spn",
            nel=[1, 1, 1],
            translate=[-1.0, -2.0, -3.0],
        )
    )


def test_single_scaled_and_translated():
    """A single voxel lattice scaled [10, 11, 12] and
    translated [0.1, 0.2, 0.3].
    """
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11],
            element_connectivity=[[1, 2, 4, 3, 5, 6, 8, 7]],
            element_coordinates=[
                [0.1, 0.2, 0.3],
                [10.1, 0.2, 0.3],
                [0.1, 11.2, 0.3],
                [10.1, 11.2, 0.3],
                [0.1, 0.2, 12.3],
                [10.1, 0.2, 12.3],
                [0.1, 11.2, 12.3],
                [10.1, 11.2, 12.3],
            ],
            file_path="tests/input/single.spn",
            nel=[1, 1, 1],
            scale=[10.0, 11.0, 12.0],
            translate=[0.1, 0.2, 0.3],
        )
    )


def test_double_x():
    """A double voxel lattice, coursed along the x-axis."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 11],
            element_connectivity=[
                [1, 2, 5, 4, 7, 8, 11, 10],
                [2, 3, 6, 5, 8, 9, 12, 11],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/double.spn",
            nel=[2, 1, 1],
        )
    )


def test_double_y():
    """A double voxel lattice, coursed along the y-axis."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 11],
            element_connectivity=[
                [1, 2, 4, 3, 7, 8, 10, 9],
                [3, 4, 6, 5, 9, 10, 12, 11],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/double.spn",
            nel=[1, 2, 1],
        )
    )


def test_triple_x():
    """A triple voxel lattice, coursed along the x-axis."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 11, 11],
            element_connectivity=[
                [1, 2, 6, 5, 9, 10, 14, 13],
                [2, 3, 7, 6, 10, 11, 15, 14],
                [3, 4, 8, 7, 11, 12, 16, 15],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/triple.spn",
            nel=[3, 1, 1],
        )
    )


def test_quadruple_x():
    """A quadruple voxel lattice, coursed along the x-axis."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 11, 11, 11],
            element_connectivity=[
                [1, 2, 7, 6, 11, 12, 17, 16],
                [2, 3, 8, 7, 12, 13, 18, 17],
                [3, 4, 9, 8, 13, 14, 19, 18],
                [4, 5, 10, 9, 14, 15, 20, 19],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/quadruple.spn",
            nel=[4, 1, 1],
        )
    )


def test_quadruple_2_voids_x():
    """A quadruple voxel lattice, coursed along the x-axis, with two
    intermediate voxels in the segmentation being void.
    """
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 11],
            element_connectivity=[
                [1, 2, 6, 5, 9, 10, 14, 13],
                [3, 4, 8, 7, 11, 12, 16, 15],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/quadruple_2_voids.spn",
            nel=[4, 1, 1],
        )
    )


def test_quadruple_2_blocks():
    """A quadruple voxel lattice, with the two intermediate voxels in the
    segmentation being a second block.
    """
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 21, 21, 11],
            element_connectivity=[
                [1, 2, 7, 6, 11, 12, 17, 16],
                [2, 3, 8, 7, 12, 13, 18, 17],
                [3, 4, 9, 8, 13, 14, 19, 18],
                [4, 5, 10, 9, 14, 15, 20, 19],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/quadruple_2_blocks.spn",
            nel=[4, 1, 1],
        )
    )


def test_quadruple_2_blocks_void():
    """A quadruple voxel lattice, with the first intermediate voxel being
    the second block and the second intermediate voxel being void."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 21, 11],
            element_connectivity=[
                [1, 2, 7, 6, 11, 12, 17, 16],
                [2, 3, 8, 7, 12, 13, 18, 17],
                [4, 5, 10, 9, 14, 15, 20, 19],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/quadruple_2_blocks_void.spn",
            nel=[4, 1, 1],
        )
    )


def test_cube():
    """A (2 x 2 x 2) voxel cube."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[11, 11, 11, 11, 11, 11, 11, 11],
            element_connectivity=[
                [1, 2, 5, 4, 10, 11, 14, 13],
                [2, 3, 6, 5, 11, 12, 15, 14],
                [4, 5, 8, 7, 13, 14, 17, 16],
                [5, 6, 9, 8, 14, 15, 18, 17],
                [10, 11, 14, 13, 19, 20, 23, 22],
                [11, 12, 15, 14, 20, 21, 24, 23],
                [13, 14, 17, 16, 22, 23, 26, 25],
                [14, 15, 18, 17, 23, 24, 27, 26],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/cube.spn",
            nel=[2, 2, 2],
        )
    )


def test_cube_multi():
    """A (2 x 2 x 2) voxel cube with two voids and six elements."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[82, 2, 2, 2, 31, 44],
            element_connectivity=[
                [1, 2, 5, 4, 10, 11, 14, 13],
                [2, 3, 6, 5, 11, 12, 15, 14],
                [4, 5, 8, 7, 13, 14, 17, 16],
                [5, 6, 9, 8, 14, 15, 18, 17],
                [11, 12, 15, 14, 19, 20, 22, 21],
                [14, 15, 18, 17, 21, 22, 24, 23],
            ],
            element_coordinates=[
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
                [1.0, 0.0, 2.0],
                [2.0, 0.0, 2.0],
                [1.0, 1.0, 2.0],
                [2.0, 1.0, 2.0],
                [1.0, 2.0, 2.0],
                [2.0, 2.0, 2.0],
            ],
            file_path="tests/input/cube_multi.spn",
            nel=[2, 2, 2],
        )
    )


def test_letter_f():
    """A minimal letter F example."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=np.ones(8) * 11,
            element_connectivity=[
                [1, 2, 4, 3, 19, 20, 22, 21],
                [3, 4, 6, 5, 21, 22, 24, 23],
                [5, 6, 9, 8, 23, 24, 27, 26],
                [6, 7, 10, 9, 24, 25, 28, 27],
                [8, 9, 12, 11, 26, 27, 30, 29],
                [11, 12, 16, 15, 29, 30, 34, 33],
                [12, 13, 17, 16, 30, 31, 35, 34],
                [13, 14, 18, 17, 31, 32, 36, 35],
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/letter_f.spn",
            nel=[3, 5, 1],
        )
    )


def test_letter_f_3d():
    """A three dimensional variation of the letter F, in a non-standard
    orientation."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=np.ones(39),
            element_connectivity=[
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
            ],
            element_coordinates=[
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
            ],
            file_path="tests/input/letter_f_3d.spn",
            nel=[4, 5, 3],
        )
    )


def test_sparse():
    """A random 5x5x5 domain composed void and two materials."""
    assert_fem_data_from_spn_eq_gold(
        Gold(
            element_blocks=[
                2, 1, 2, 1, 1, 2, 1, 1, 2, 1,
                1, 2, 1, 1, 1, 1, 2, 2, 1, 1,
                1, 1, 2, 1, 2, 2, 2, 2, 1, 1,
                2, 1, 1, 1, 2, 2, 1, 2, 2, 2,
                2, 1, 1, 2, 2, 2, 2, 2, 2, 2,
                1, 2, 2, 1, 1, 1, 2, 1,
            ],
            element_connectivity=[
                [1, 2, 4, 3, 29, 30, 36, 35],
                [3, 4, 10, 9, 35, 36, 42, 41],
                [5, 6, 12, 11, 37, 38, 44, 43],
                [6, 7, 13, 12, 38, 39, 45, 44],
                [8, 9, 15, 14, 40, 41, 47, 46],
                [9, 10, 16, 15, 41, 42, 48, 47],
                [11, 12, 18, 17, 43, 44, 50, 49],
                [15, 16, 22, 21, 47, 48, 54, 53],
                [17, 18, 24, 23, 49, 50, 56, 55],
                [18, 19, 25, 24, 50, 51, 57, 56],
                [20, 21, 27, 26, 52, 53, 59, 58],
                [21, 22, 28, 27, 53, 54, 60, 59],
                [31, 32, 38, 37, 64, 65, 71, 70],
                [32, 33, 39, 38, 65, 66, 72, 71],
                [34, 35, 41, 40, 67, 68, 74, 73],
                [35, 36, 42, 41, 68, 69, 75, 74],
                [40, 41, 47, 46, 73, 74, 80, 79],
                [43, 44, 50, 49, 76, 77, 83, 82],
                [44, 45, 51, 50, 77, 78, 84, 83],
                [46, 47, 53, 52, 79, 80, 86, 85],
                [49, 50, 56, 55, 82, 83, 89, 88],
                [54, 55, 61, 60, 87, 88, 93, 92],
                [62, 63, 69, 68, 96, 97, 102, 101],
                [63, 64, 70, 69, 97, 98, 103, 102],
                [64, 65, 71, 70, 98, 99, 104, 103],
                [70, 71, 77, 76, 103, 104, 110, 109],
                [75, 76, 82, 81, 108, 109, 114, 113],
                [76, 77, 83, 82, 109, 110, 115, 114],
                [81, 82, 88, 87, 113, 114, 119, 118],
                [82, 83, 89, 88, 114, 115, 120, 119],
                [86, 87, 92, 91, 117, 118, 123, 122],
                [88, 89, 94, 93, 119, 120, 125, 124],
                [89, 90, 95, 94, 120, 121, 126, 125],
                [98, 99, 104, 103, 130, 131, 137, 136],
                [99, 100, 105, 104, 131, 132, 138, 137],
                [101, 102, 108, 107, 134, 135, 141, 140],
                [102, 103, 109, 108, 135, 136, 142, 141],
                [106, 107, 112, 111, 139, 140, 146, 145],
                [108, 109, 114, 113, 141, 142, 148, 147],
                [111, 112, 117, 116, 145, 146, 151, 150],
                [112, 113, 118, 117, 146, 147, 152, 151],
                [114, 115, 120, 119, 148, 149, 154, 153],
                [118, 119, 124, 123, 152, 153, 159, 158],
                [120, 121, 126, 125, 154, 155, 161, 160],
                [127, 128, 134, 133, 162, 163, 168, 167],
                [129, 130, 136, 135, 164, 165, 170, 169],
                [130, 131, 137, 136, 165, 166, 171, 170],
                [133, 134, 140, 139, 167, 168, 174, 173],
                [134, 135, 141, 140, 168, 169, 175, 174],
                [135, 136, 142, 141, 169, 170, 176, 175],
                [136, 137, 143, 142, 170, 171, 177, 176],
                [137, 138, 144, 143, 171, 172, 178, 177],
                [141, 142, 148, 147, 175, 176, 180, 179],
                [147, 148, 153, 152, 179, 180, 185, 184],
                [148, 149, 154, 153, 180, 181, 186, 185],
                [150, 151, 157, 156, 182, 183, 189, 188],
                [151, 152, 158, 157, 183, 184, 190, 189],
                [154, 155, 161, 160, 186, 187, 192, 191],
            ],
            element_coordinates=[
                [1.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [2.0, 1.0, 0.0],
                [3.0, 1.0, 0.0],
                [4.0, 1.0, 0.0],
                [5.0, 1.0, 0.0],
                [0.0, 2.0, 0.0],
                [1.0, 2.0, 0.0],
                [2.0, 2.0, 0.0],
                [3.0, 2.0, 0.0],
                [4.0, 2.0, 0.0],
                [5.0, 2.0, 0.0],
                [0.0, 3.0, 0.0],
                [1.0, 3.0, 0.0],
                [2.0, 3.0, 0.0],
                [3.0, 3.0, 0.0],
                [4.0, 3.0, 0.0],
                [5.0, 3.0, 0.0],
                [0.0, 4.0, 0.0],
                [1.0, 4.0, 0.0],
                [2.0, 4.0, 0.0],
                [3.0, 4.0, 0.0],
                [4.0, 4.0, 0.0],
                [5.0, 4.0, 0.0],
                [0.0, 5.0, 0.0],
                [1.0, 5.0, 0.0],
                [2.0, 5.0, 0.0],
                [1.0, 0.0, 1.0],
                [2.0, 0.0, 1.0],
                [3.0, 0.0, 1.0],
                [4.0, 0.0, 1.0],
                [5.0, 0.0, 1.0],
                [0.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [2.0, 1.0, 1.0],
                [3.0, 1.0, 1.0],
                [4.0, 1.0, 1.0],
                [5.0, 1.0, 1.0],
                [0.0, 2.0, 1.0],
                [1.0, 2.0, 1.0],
                [2.0, 2.0, 1.0],
                [3.0, 2.0, 1.0],
                [4.0, 2.0, 1.0],
                [5.0, 2.0, 1.0],
                [0.0, 3.0, 1.0],
                [1.0, 3.0, 1.0],
                [2.0, 3.0, 1.0],
                [3.0, 3.0, 1.0],
                [4.0, 3.0, 1.0],
                [5.0, 3.0, 1.0],
                [0.0, 4.0, 1.0],
                [1.0, 4.0, 1.0],
                [2.0, 4.0, 1.0],
                [3.0, 4.0, 1.0],
                [4.0, 4.0, 1.0],
                [5.0, 4.0, 1.0],
                [0.0, 5.0, 1.0],
                [1.0, 5.0, 1.0],
                [2.0, 5.0, 1.0],
                [3.0, 5.0, 1.0],
                [1.0, 0.0, 2.0],
                [2.0, 0.0, 2.0],
                [3.0, 0.0, 2.0],
                [4.0, 0.0, 2.0],
                [5.0, 0.0, 2.0],
                [0.0, 1.0, 2.0],
                [1.0, 1.0, 2.0],
                [2.0, 1.0, 2.0],
                [3.0, 1.0, 2.0],
                [4.0, 1.0, 2.0],
                [5.0, 1.0, 2.0],
                [0.0, 2.0, 2.0],
                [1.0, 2.0, 2.0],
                [2.0, 2.0, 2.0],
                [3.0, 2.0, 2.0],
                [4.0, 2.0, 2.0],
                [5.0, 2.0, 2.0],
                [0.0, 3.0, 2.0],
                [1.0, 3.0, 2.0],
                [2.0, 3.0, 2.0],
                [3.0, 3.0, 2.0],
                [4.0, 3.0, 2.0],
                [5.0, 3.0, 2.0],
                [0.0, 4.0, 2.0],
                [1.0, 4.0, 2.0],
                [2.0, 4.0, 2.0],
                [3.0, 4.0, 2.0],
                [4.0, 4.0, 2.0],
                [5.0, 4.0, 2.0],
                [1.0, 5.0, 2.0],
                [2.0, 5.0, 2.0],
                [3.0, 5.0, 2.0],
                [4.0, 5.0, 2.0],
                [5.0, 5.0, 2.0],
                [1.0, 0.0, 3.0],
                [2.0, 0.0, 3.0],
                [3.0, 0.0, 3.0],
                [4.0, 0.0, 3.0],
                [5.0, 0.0, 3.0],
                [1.0, 1.0, 3.0],
                [2.0, 1.0, 3.0],
                [3.0, 1.0, 3.0],
                [4.0, 1.0, 3.0],
                [5.0, 1.0, 3.0],
                [0.0, 2.0, 3.0],
                [1.0, 2.0, 3.0],
                [2.0, 2.0, 3.0],
                [3.0, 2.0, 3.0],
                [4.0, 2.0, 3.0],
                [0.0, 3.0, 3.0],
                [1.0, 3.0, 3.0],
                [2.0, 3.0, 3.0],
                [3.0, 3.0, 3.0],
                [4.0, 3.0, 3.0],
                [0.0, 4.0, 3.0],
                [1.0, 4.0, 3.0],
                [2.0, 4.0, 3.0],
                [3.0, 4.0, 3.0],
                [4.0, 4.0, 3.0],
                [5.0, 4.0, 3.0],
                [1.0, 5.0, 3.0],
                [2.0, 5.0, 3.0],
                [3.0, 5.0, 3.0],
                [4.0, 5.0, 3.0],
                [5.0, 5.0, 3.0],
                [0.0, 0.0, 4.0],
                [1.0, 0.0, 4.0],
                [2.0, 0.0, 4.0],
                [3.0, 0.0, 4.0],
                [4.0, 0.0, 4.0],
                [5.0, 0.0, 4.0],
                [0.0, 1.0, 4.0],
                [1.0, 1.0, 4.0],
                [2.0, 1.0, 4.0],
                [3.0, 1.0, 4.0],
                [4.0, 1.0, 4.0],
                [5.0, 1.0, 4.0],
                [0.0, 2.0, 4.0],
                [1.0, 2.0, 4.0],
                [2.0, 2.0, 4.0],
                [3.0, 2.0, 4.0],
                [4.0, 2.0, 4.0],
                [5.0, 2.0, 4.0],
                [0.0, 3.0, 4.0],
                [1.0, 3.0, 4.0],
                [2.0, 3.0, 4.0],
                [3.0, 3.0, 4.0],
                [4.0, 3.0, 4.0],
                [0.0, 4.0, 4.0],
                [1.0, 4.0, 4.0],
                [2.0, 4.0, 4.0],
                [3.0, 4.0, 4.0],
                [4.0, 4.0, 4.0],
                [5.0, 4.0, 4.0],
                [0.0, 5.0, 4.0],
                [1.0, 5.0, 4.0],
                [2.0, 5.0, 4.0],
                [3.0, 5.0, 4.0],
                [4.0, 5.0, 4.0],
                [5.0, 5.0, 4.0],
                [0.0, 0.0, 5.0],
                [1.0, 0.0, 5.0],
                [2.0, 0.0, 5.0],
                [3.0, 0.0, 5.0],
                [4.0, 0.0, 5.0],
                [0.0, 1.0, 5.0],
                [1.0, 1.0, 5.0],
                [2.0, 1.0, 5.0],
                [3.0, 1.0, 5.0],
                [4.0, 1.0, 5.0],
                [5.0, 1.0, 5.0],
                [0.0, 2.0, 5.0],
                [1.0, 2.0, 5.0],
                [2.0, 2.0, 5.0],
                [3.0, 2.0, 5.0],
                [4.0, 2.0, 5.0],
                [5.0, 2.0, 5.0],
                [2.0, 3.0, 5.0],
                [3.0, 3.0, 5.0],
                [4.0, 3.0, 5.0],
                [0.0, 4.0, 5.0],
                [1.0, 4.0, 5.0],
                [2.0, 4.0, 5.0],
                [3.0, 4.0, 5.0],
                [4.0, 4.0, 5.0],
                [5.0, 4.0, 5.0],
                [0.0, 5.0, 5.0],
                [1.0, 5.0, 5.0],
                [2.0, 5.0, 5.0],
                [4.0, 5.0, 5.0],
                [5.0, 5.0, 5.0],
            ],
            file_path="tests/input/sparse.spn",
            nel=[5, 5, 5],
        )
    )


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
    gold_data = np.array(
        [
            [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
            [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
            [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
            [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
        ]
    )
    voxels = Voxels.from_npy("tests/input/letter_f_3d.npy")
    assert (voxels.data == gold_data).all()


def test_write_npy_letter_f_3d():
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
    gold_data = np.array(
        [
            [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
            [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
            [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
            [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
        ]
    )
    Voxels.from_npy("tests/input/letter_f_3d.npy").write_npy(
        "target/letter_f_3d.npy"
    )
    voxels = Voxels.from_npy("target/letter_f_3d.npy")
    assert (voxels.data == gold_data).all()


def test_write_npy_sparse():
    """A test of the random 5x5x5 domain composed void and two materials"""
    gold_data = np.array(
        [
            [
                [0, 0, 0, 0, 2],
                [0, 1, 0, 0, 2],
                [1, 2, 0, 2, 0],
                [0, 1, 0, 2, 0],
                [1, 0, 0, 0, 1],
            ],
            [
                [2, 0, 2, 0, 0],
                [1, 1, 0, 2, 2],
                [2, 0, 0, 0, 0],
                [1, 0, 0, 2, 0],
                [2, 0, 2, 0, 2],
            ],
            [
                [0, 0, 1, 0, 2],
                [0, 0, 0, 1, 2],
                [0, 0, 2, 2, 2],
                [0, 0, 1, 0, 1],
                [0, 1, 0, 1, 0],
            ],
            [
                [0, 1, 2, 1, 2],
                [2, 0, 2, 0, 1],
                [1, 2, 2, 0, 0],
                [2, 1, 1, 1, 1],
                [0, 0, 1, 0, 0],
            ],
            [
                [0, 1, 0, 2, 0],
                [1, 0, 0, 0, 2],
                [0, 1, 0, 0, 0],
                [1, 0, 0, 0, 0],
                [0, 0, 1, 2, 1],
            ],
        ]
    )
    Voxels.from_npy("tests/input/sparse.npy").write_npy("target/sparse.npy")
    voxels = Voxels.from_npy("target/sparse.npy")
    assert (voxels.data == gold_data).all()
