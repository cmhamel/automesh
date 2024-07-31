import numpy as np
from automesh import Spn

gold = np.array([
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
    [[1, 1, 1], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 0, 0]],
])


def test_as_exodus():
    assert False


def test_from_npy():
    spn = Spn.from_npy('tests/input/f.npy')
    assert (spn.get_data() == gold).all()


def test_new():
    spn = Spn('tests/input/f.spn', 4, 5, 3)
    assert (spn.get_data() == gold).all()
