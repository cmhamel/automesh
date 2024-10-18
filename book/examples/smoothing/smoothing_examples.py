"""This module contains data for the smoothing examples."""

import smoothing_types as ty

# Type alias for functional style methods
# https://docs.python.org/3/library/typing.html#type-aliases
SmoothingAlgorithm = ty.SmoothingAlgorithm
Example = ty.SmoothingExample
Vertex = ty.Vertex

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
    hierarchy=(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ),
    scale_lambda=0.3,
    scale_mu=-0.33,
    num_iters=2,
    algorithm=SmoothingAlgorithm.LAPLACE
)
