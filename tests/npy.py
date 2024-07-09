import numpy as np
from automesh import Npy

gold = np.array([
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
], dtype='uint8')


def test_new():
    npy = Npy('tests/npy/f.npy')
    assert (npy.get_data() == gold).all()


def test_temporary():
    npy = Npy('tests/npy/f.npy')
    exo = npy.exodus()
    print(exo.element_connectivity)
    print("Temporary test.")
