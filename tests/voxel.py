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
