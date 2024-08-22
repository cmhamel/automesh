import numpy as np
from automesh import Spn

nel = [4, 5, 3]
number_of_elements = 39
scale = [1.2, 2.3, 0.4]
translate = [-0.3, 1.1, 0.5]

gold_blocks = np.ones(number_of_elements)
gold_connectivity = np.array([
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
])
gold_coordinates = np.array([
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
])
gold_data = np.array([
    [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
    [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
    [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
    [[1, 0, 0], [1, 0, 0], [1, 1, 0], [1, 0, 0], [1, 1, 1]],
])
for i in range(3):
    gold_coordinates[:, i] *= scale[i]
    gold_coordinates[:, i] += translate[i]


def test_as_finite_elements():
    spn = Spn.from_npy('tests/input/f.npy')
    fem = spn.as_finite_elements(scale, translate)
    assert (fem.element_blocks == gold_blocks).all()
    assert (fem.element_connectivity == gold_connectivity).all()
    assert (fem.nodal_coordinates == gold_coordinates).all()


def test_from_npy():
    spn = Spn.from_npy('tests/input/f.npy')
    assert (spn.data == gold_data).all()


def test_new():
    spn = Spn.from_spn('tests/input/f.spn', nel)
    assert (spn.data == gold_data).all()


def test_write_npy():
    Spn.from_spn('tests/input/f.spn', nel).write_npy('target/f.npy')
    spn = Spn.from_npy('target/f.npy')
    assert (spn.data == gold_data).all()
