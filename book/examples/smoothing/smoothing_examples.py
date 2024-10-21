"""This module contains data for the smoothing examples."""

import math
from typing import Final

import smoothing_types as ty

# Type alias for functional style methods
# https://docs.python.org/3/library/typing.html#type-aliases
SmoothingAlgorithm = ty.SmoothingAlgorithm
Example = ty.SmoothingExample
Vertex = ty.Vertex

DEG2RAD: Final[float] = math.pi / 180.0  # rad/deg

# L-bracket example
bracket = Example(
    vertices=(
        Vertex(0, 0, 0),
        Vertex(1, 0, 0),
        Vertex(2, 0, 0),
        Vertex(3, 0, 0),
        Vertex(4, 0, 0),
        Vertex(0, 1, 0),
        Vertex(1, 1, 0),
        Vertex(2, 1, 0),
        Vertex(3, 1, 0),
        Vertex(4, 1, 0),
        Vertex(0, 2, 0),
        Vertex(1, 2, 0),
        Vertex(2, 2, 0),
        Vertex(3, 2, 0),
        Vertex(4, 2, 0),
        Vertex(0, 3, 0),
        Vertex(1, 3, 0),
        Vertex(2, 3, 0),
        Vertex(0, 4, 0),
        Vertex(1, 4, 0),
        Vertex(2, 4, 0),
        Vertex(0, 0, 1),
        Vertex(1, 0, 1),
        Vertex(2, 0, 1),
        Vertex(3, 0, 1),
        Vertex(4, 0, 1),
        Vertex(0, 1, 1),
        Vertex(1, 1, 1),
        Vertex(2, 1, 1),
        Vertex(3, 1, 1),
        Vertex(4, 1, 1),
        Vertex(0, 2, 1),
        Vertex(1, 2, 1),
        Vertex(2, 2, 1),
        Vertex(3, 2, 1),
        Vertex(4, 2, 1),
        Vertex(0, 3, 1),
        Vertex(1, 3, 1),
        Vertex(2, 3, 1),
        Vertex(0, 4, 1),
        Vertex(1, 4, 1),
        Vertex(2, 4, 1),
    ),
    elements=(
        (1, 2, 7, 6, 22, 23, 28, 27),
        (2, 3, 8, 7, 23, 24, 29, 28),
        (3, 4, 9, 8, 24, 25, 30, 29),
        (4, 5, 10, 9, 25, 26, 31, 30),
        (6, 7, 12, 11, 27, 28, 33, 32),
        (7, 8, 13, 12, 28, 29, 34, 33),
        (8, 9, 14, 13, 29, 30, 35, 34),
        (9, 10, 15, 14, 30, 31, 36, 35),
        (11, 12, 17, 16, 32, 33, 38, 37),
        (12, 13, 18, 17, 33, 34, 39, 38),
        (16, 17, 20, 19, 37, 38, 41, 40),
        (17, 18, 21, 20, 38, 39, 42, 41),
    ),
    nelx=4,
    nely=4,
    nelz=1,
    neighbors=(
        (2, 6, 22),
        (1, 3, 7, 23),
        (2, 4, 8, 24),
        (3, 5, 9, 25),
        (4, 10, 26),
        #
        (1, 7, 11, 27),
        (2, 6, 8, 12, 28),
        (3, 7, 9, 13, 29),
        (4, 8, 10, 14, 30),
        (5, 9, 15, 31),
        #
        (6, 12, 16, 32),
        (7, 11, 13, 17, 33),
        (8, 12, 14, 18, 34),
        (9, 13, 15, 35),
        (10, 14, 36),
        #
        (11, 17, 19, 37),
        (12, 16, 18, 20, 38),
        (13, 17, 21, 39),
        #
        (16, 20, 40),
        (17, 19, 21, 41),
        (18, 20, 42),
        # top layer
        (1, 23, 27),
        (2, 22, 24, 28),
        (3, 23, 25, 29),
        (4, 24, 26, 30),
        (5, 25, 31),
        #
        (6, 22, 28, 32),
        (7, 23, 27, 29, 33),
        (8, 24, 28, 30, 34),
        (9, 25, 29, 31, 35),
        (10, 26, 30, 36),
        #
        (11, 27, 33, 37),
        (12, 28, 32, 34, 38),
        (13, 29, 33, 35, 39),
        (14, 30, 34, 36),
        (15, 31, 35),
        #
        (16, 32, 38, 40),
        (17, 33, 37, 39, 41),
        (18, 34, 38, 42),
        #
        (19, 37, 41),
        (20, 38, 40, 42),
        (21, 39, 41),
    ),
    hierarchy=(
        # hierarchy enum, node number, prescribed (x, y, z)
        2,  # 1 -> (0, 0, 0)
        2,  # 2 -> (1, 0, 0)
        2,  # 3 -> (2, 0, 0)
        2,  # 4 -> (3, 0, 0)
        2,  # 5 -> (4, 0, 0)
        2,  # 6 -> (0, 1, 0)
        1,  # 7
        1,  # 8
        1,  # 9
        2,  # 10 -> (4*cos(22.5 deg), 4*sin(22.5 deg), 0)
        2,  # 11 -> *(0, 2, 0)
        1,  # 12
        1,  # 13
        1,  # 14
        2,  # 15 -> (4*cos(45 deg), 4*sin(45 deg), 0)
        2,  # 16 -> (0, 3, 0)
        1,  # 17
        1,  # 18
        2,  # 19 -> (0, 4, 0)
        2,  # 20 -> (4*cos(67.5 deg), 4*sin(67.5 deg), 0)
        2,  # 21 -> (4*cos(45 deg), 4*sin(45 deg), 0)
        #
        2,  # 22 -> (0, 0, 1)
        2,  # 23 -> (1, 0, 1)
        2,  # 24 -> (2, 0, 1)
        2,  # 25 -> (3, 0, 1)
        2,  # 26 -> (4, 0, 1)
        2,  # 27 -> (0, 1, 1)
        1,  # 28
        1,  # 29
        1,  # 30
        2,  # 31 -> (4*cos(22.5 deg), 4*sin(22.5 deg), 1)
        2,  # 32 -> *(0, 2, 1)
        1,  # 33
        1,  # 34
        1,  # 35
        2,  # 36 -> (4*cos(45 deg), 4*sin(45 deg), 1)
        2,  # 37 -> (0, 3, 1)
        1,  # 38
        1,  # 39
        2,  # 40 -> (0, 4, 1)
        2,  # 41 -> (4*cos(67.5 deg), 4*sin(67.5 deg), 1)
        2,  # 42 -> (4*cos(45 deg), 4*sin(45 deg), 1)
    ),
    prescribed=(
        (1, Vertex(0, 0, 0)),
        (2, Vertex(1, 0, 0)),
        (3, Vertex(2, 0, 0)),
        (4, Vertex(3, 0, 0)),
        (5, Vertex(4, 0, 0)),
        (6, Vertex(0, 1, 0)),
        (
            10,
            Vertex(
                4 * math.cos(22.5 * DEG2RAD), 4 * math.sin(22.5 * DEG2RAD), 0
            ),
        ),
        (11, Vertex(0, 2, 0)),
        (
            15,
            Vertex(4 * math.cos(45 * DEG2RAD), 4 * math.sin(45 * DEG2RAD), 0),
        ),
        (16, Vertex(0, 3, 0)),
        (19, Vertex(0, 4, 0)),
        (
            20,
            Vertex(
                4 * math.cos(67.5 * DEG2RAD), 4 * math.sin(67.5 * DEG2RAD), 0
            ),
        ),
        (
            21,
            Vertex(4 * math.cos(45 * DEG2RAD), 4 * math.sin(45 * DEG2RAD), 0),
        ),
        (22, Vertex(0, 0, 1)),
        (23, Vertex(1, 0, 1)),
        (24, Vertex(2, 0, 1)),
        (25, Vertex(3, 0, 1)),
        (26, Vertex(4, 0, 1)),
        (27, Vertex(0, 1, 1)),
        (
            31,
            Vertex(
                4 * math.cos(22.5 * DEG2RAD), 4 * math.sin(22.5 * DEG2RAD), 1
            ),
        ),
        (32, Vertex(0, 2, 1)),
        (
            36,
            Vertex(4 * math.cos(45 * DEG2RAD), 4 * math.sin(45 * DEG2RAD), 1),
        ),
        (37, Vertex(0, 3, 1)),
        (40, Vertex(0, 4, 1)),
        (
            41,
            Vertex(
                4 * math.cos(67.5 * DEG2RAD), 4 * math.sin(67.5 * DEG2RAD), 1
            ),
        ),
        (
            42,
            Vertex(4 * math.cos(45 * DEG2RAD), 4 * math.sin(45 * DEG2RAD), 1),
        ),
    ),
    scale_lambda=0.3,
    scale_mu=-0.33,
    num_iters=10,
    algorithm=SmoothingAlgorithm.LAPLACE,
)

# Double X two-element example
double_x = Example(
    vertices=(
        Vertex(0.0, 0.0, 0.0),
        Vertex(1.0, 0.0, 0.0),
        Vertex(2.0, 0.0, 0.0),
        Vertex(0.0, 1.0, 0.0),
        Vertex(1.0, 1.0, 0.0),
        Vertex(2.0, 1.0, 0.0),
        Vertex(0.0, 0.0, 1.0),
        Vertex(1.0, 0.0, 1.0),
        Vertex(2.0, 0.0, 1.0),
        Vertex(0.0, 1.0, 1.0),
        Vertex(1.0, 1.0, 1.0),
        Vertex(2.0, 1.0, 1.0),
    ),
    elements=(
        (1, 2, 5, 4, 7, 8, 11, 10),
        (2, 3, 6, 5, 8, 9, 12, 11),
    ),
    nelx=2,
    nely=1,
    nelz=1,
    neighbors=(
        (2, 4, 7),
        (1, 3, 5, 8),
        (2, 6, 9),
        (1, 5, 10),
        (2, 4, 6, 11),
        (3, 5, 12),
        (1, 8, 10),
        (2, 7, 9, 11),
        (3, 8, 12),
        (4, 7, 11),
        (5, 8, 10, 12),
        (6, 9, 11),
    ),
    hierarchy=(
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
    ),
    prescribed=None,
    scale_lambda=0.3,
    scale_mu=-0.33,
    num_iters=2,
    algorithm=SmoothingAlgorithm.LAPLACE,
)
