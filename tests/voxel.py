import numpy as np
from automesh import Voxels


def test_write_npy():
    Voxels.from_spn('tests/input/letter_f_3d.spn', [4, 5, 3]) \
          .write_npy('target/letter_f_3d.npy')
    assert (np.load('tests/input/letter_f_3d.npy') ==
            np.load('target/letter_f_3d.npy')).all()
