r"""This module tests the smoothing modules.
Example:
    cd ~/autotwin/automesh
    Activate the venv with one of the following:
    source .venv/bin/activate       # for bash shell
    source .venv/bin/activate.csh   # for c shell
    source .venv/bin/activate.fish  # for fish shell
    .\.venv\Scripts\activate        # for powershell
    cd book/examples/smoothing
    python -m pytest smoothing_test.py
Reference: DoubleX unit test
    https://autotwin.github.io/automesh/examples/unit_tests/index.html#double-x
"""

from typing import Final

# import sandbox.smoothing as sm
# import sandbox.smoothing_types as ty
import smoothing as sm
import smoothing_examples as examples
import smoothing_types as ty

# Type alias for functional style methods
# https://docs.python.org/3/library/typing.html#type-aliases
# DofSet = ty.DofSet
# Elements = ty.Elements
Hexes = ty.Hexes
Hierarchy = ty.Hierarchy
Neighbors = ty.Neighbors
NodeHierarchy = ty.NodeHierarchy
Vertex = ty.Vertex
Vertices = ty.Vertices
SmoothingAlgorithm = ty.SmoothingAlgorithm


def test_average_position():
    """Unit test for average_position"""
    v1 = Vertex(x=1.0, y=2.0, z=3.0)
    v2 = Vertex(x=4.0, y=5.0, z=6.0)
    v3 = Vertex(x=7.0, y=8.0, z=9.0)

    v_ave = sm.average_position((v1, v2, v3))
    assert v_ave.x == 4.0
    assert v_ave.y == 5.0
    assert v_ave.z == 6.0


def test_add():
    """Unit test for the addition of Vertex v1 and Vertex v2."""
    v1 = Vertex(x=1.0, y=2.0, z=3.0)
    v2 = Vertex(x=4.0, y=7.0, z=1.0)
    vv = sm.add(v1=v1, v2=v2)
    assert vv.x == 5.0
    assert vv.y == 9.0
    assert vv.z == 4.0


def test_subtract():
    """Unit test for the subtraction of Vertex v2 from Vertex v1."""
    v1 = Vertex(x=1.0, y=2.0, z=3.0)
    v2 = Vertex(x=4.0, y=7.0, z=1.0)
    vv = sm.subtract(v1=v1, v2=v2)
    assert vv.x == -3.0
    assert vv.y == -5.0
    assert vv.z == 2.0


def test_scale():
    """Unit test for the scale function."""
    v1 = Vertex(x=1.0, y=2.0, z=3.0)
    ss = 10.0
    result = sm.scale(vertex=v1, scale_factor=ss)
    assert result.x == 10.0
    assert result.y == 20.0
    assert result.z == 30.0


def test_xyz():
    """Unit test to assure the (x, y, z) coordinate tuple is returned
    correctly.
    """
    vv = Vertex(x=1.1, y=2.2, z=3.3)
    gold = (1.1, 2.2, 3.3)
    result = sm.xyz(vv)
    assert result == gold


def test_smoothing_neighbors():
    """Given the Double X test problem with completely made up
    node hierarchy, assure that `smoothing_neighbors` returns
    the correct neighbors.
    """
    ex = examples.double_x
    neighbors = ex.neighbors  # borrow the neighbor connections

    node_hierarchy = (
        Hierarchy.INTERIOR,
        Hierarchy.BOUNDARY,
        Hierarchy.PRESCRIBED,
        Hierarchy.PRESCRIBED,
        Hierarchy.BOUNDARY,
        Hierarchy.INTERIOR,
        Hierarchy.INTERIOR,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.INTERIOR,
        Hierarchy.INTERIOR,
        Hierarchy.INTERIOR,
    )

    result = sm.smoothing_neighbors(
        neighbors=neighbors, node_hierarchy=node_hierarchy
    )
    gold_smoothing_neighbors = (
        (2, 4, 7),
        (3, 5, 8),
        (),
        (),
        (2, 4),
        (3, 5, 12),
        (1, 8, 10),
        (2, 9),
        (3, 8),
        (4, 7, 11),
        (5, 8, 10, 12),
        (6, 9, 11),
    )

    assert result == gold_smoothing_neighbors


def test_laplace_hierarchical_bracket():
    """Unit test for Laplace smoothing with hierarhical control
    on the Bracket example."""
    bracket = examples.bracket

    node_hierarchy = bracket.node_hierarchy
    neighbors = bracket.neighbors
    node_hierarchy = bracket.node_hierarchy

    # If a node is PRESCRIBED, then it has no smoothing neighbors
    smoothing_neighbors = sm.smoothing_neighbors(
        neighbors=neighbors, node_hierarchy=node_hierarchy
    )
    gold_smoothing_neighbors = (
        (),  # 1
        (),  # 2
        (),  # 3
        (),  # 4
        (),  # 5
        (),  # 6
        (2, 6, 8, 12, 28),  # 7
        (3, 7, 9, 13, 29),  # 8
        (4, 8, 10, 14, 30),  # 9
        (),  # 10
        (),  # 11
        (7, 11, 13, 17, 33),  # 12
        (8, 12, 14, 18, 34),  # 13
        (9, 13, 15, 35),  # 14
        (),  # 15
        (),  # 16
        (12, 16, 18, 20, 38),  # 17
        (13, 17, 21, 39),  # 18
        (),  # 19
        (),  # 20
        (),
        (),  # 22
        (),
        (),  # 24
        (),
        (),  # 26
        (),
        (7, 23, 27, 29, 33),  # 28
        (8, 24, 28, 30, 34),  # 29
        (9, 25, 29, 31, 35),  # 30
        (),  # 31
        (),  # 32
        (12, 28, 32, 34, 38),  # 33
        (13, 29, 33, 35, 39),  # 34
        (14, 30, 34, 36),  # 35
        (),  # 36
        (),  # 37
        (17, 33, 37, 39, 41),  # 38
        (18, 34, 38, 42),  # 39
        (),  # 40
        (),  # 41
        (),  # 42
    )

    assert smoothing_neighbors == gold_smoothing_neighbors

    # specific test with lambda = 0.3 and num_iters = 10
    scale_lambda_test = 0.3
    num_iters_test = 10

    result = sm.smooth(
        vv=bracket.vertices,
        nn=bracket.neighbors,
        node_hierarchy=bracket.node_hierarchy,
        prescribed_nodes=bracket.prescribed_nodes,
        scale_lambda=scale_lambda_test,
        num_iters=num_iters_test,
        algorithm=bracket.algorithm,
    )

    gold_vertices_10_iter = (
        Vertex(x=0, y=0, z=0),
        Vertex(x=1, y=0, z=0),
        Vertex(x=2, y=0, z=0),
        Vertex(x=3, y=0, z=0),
        Vertex(x=4, y=0, z=0),
        Vertex(x=0, y=1, z=0),
        Vertex(
            x=0.9974824535030984, y=0.9974824535030984, z=0.24593434133370803
        ),
        Vertex(
            x=1.9620726956646117, y=1.0109475009958278, z=0.2837944855813176
        ),
        Vertex(
            x=2.848322987789396, y=1.1190213008349328, z=0.24898414051620496
        ),
        Vertex(x=3.695518130045147, y=1.5307337294603591, z=0),
        Vertex(x=0, y=2, z=0),
        Vertex(
            x=1.0109475009958275, y=1.9620726956646117, z=0.2837944855813176
        ),
        Vertex(
            x=1.9144176939366933, y=1.9144176939366933, z=0.3332231502067546
        ),
        Vertex(
            x=2.5912759493290007, y=1.961874667390146, z=0.29909606343914835
        ),
        Vertex(x=2.8284271247461903, y=2.82842712474619, z=0),
        Vertex(x=0, y=3, z=0),
        Vertex(
            x=1.119021300834933, y=2.848322987789396, z=0.24898414051620493
        ),
        Vertex(
            x=1.9618746673901462, y=2.5912759493290007, z=0.29909606343914835
        ),
        Vertex(x=0, y=4, z=0),
        Vertex(x=1.5307337294603593, y=3.695518130045147, z=0),
        Vertex(x=2.8284271247461903, y=2.82842712474619, z=0),
        Vertex(x=0, y=0, z=1),
        Vertex(x=1, y=0, z=1),
        Vertex(x=2, y=0, z=1),
        Vertex(x=3, y=0, z=1),
        Vertex(x=4, y=0, z=1),
        Vertex(x=0, y=1, z=1),
        Vertex(
            x=0.9974824535030984, y=0.9974824535030984, z=0.7540656586662919
        ),
        Vertex(
            x=1.9620726956646117, y=1.0109475009958278, z=0.7162055144186824
        ),
        Vertex(x=2.848322987789396, y=1.119021300834933, z=0.7510158594837951),
        Vertex(x=3.695518130045147, y=1.5307337294603591, z=1),
        Vertex(x=0, y=2, z=1),
        Vertex(
            x=1.0109475009958275, y=1.9620726956646117, z=0.7162055144186824
        ),
        Vertex(
            x=1.9144176939366933, y=1.9144176939366933, z=0.6667768497932453
        ),
        Vertex(
            x=2.591275949329001, y=1.9618746673901462, z=0.7009039365608517
        ),
        Vertex(x=2.8284271247461903, y=2.82842712474619, z=1),
        Vertex(x=0, y=3, z=1),
        Vertex(x=1.1190213008349328, y=2.848322987789396, z=0.751015859483795),
        Vertex(
            x=1.9618746673901462, y=2.5912759493290007, z=0.7009039365608516
        ),
        Vertex(x=0, y=4, z=1),
        Vertex(x=1.5307337294603593, y=3.695518130045147, z=1),
        Vertex(x=2.8284271247461903, y=2.82842712474619, z=1),
    )

    assert result == gold_vertices_10_iter


def test_laplace_smoothing_double_x():
    """Unit test for Laplace smoothing with all dofs as BOUNDARY
    on the Double X example."""
    vv: Vertices = (
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
    )

    nn: Neighbors = (
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
    )

    nh: NodeHierarchy = (
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
        Hierarchy.BOUNDARY,
    )

    scale_lambda: Final[float] = 0.3  # lambda for Laplace smoothing

    # iteration 1
    num_iters = 1  # single iteration of smoothing

    algo = SmoothingAlgorithm.LAPLACE

    aa = sm.smooth(
        vv=vv,
        nn=nn,
        node_hierarchy=nh,
        prescribed_nodes=None,
        scale_lambda=scale_lambda,
        num_iters=num_iters,
        algorithm=algo,
    )
    cc: Final[float] = scale_lambda / 3.0  # delta corner
    ee: Final[float] = scale_lambda / 4.0  # delta edge
    # define the gold standard fiducial
    gold = (
        Vertex(x=cc, y=cc, z=cc),  # node 1, corner
        Vertex(x=1.0, y=ee, z=ee),  # node 2, edge
        Vertex(x=2.0 - cc, y=cc, z=cc),  # node 3, corner
        #
        Vertex(x=cc, y=1.0 - cc, z=cc),  # node 4, corner
        Vertex(x=1.0, y=1.0 - ee, z=ee),  # node 5, edge
        Vertex(x=2.0 - cc, y=1.0 - cc, z=cc),  # node 6, corner
        #
        Vertex(x=cc, y=cc, z=1 - cc),  # node 7, corner
        Vertex(x=1.0, y=ee, z=1 - ee),  # node 8, edge
        Vertex(x=2.0 - cc, y=cc, z=1 - cc),  # node 9, corner
        #
        Vertex(x=cc, y=1.0 - cc, z=1 - cc),  # node 10, corner
        Vertex(x=1.0, y=1.0 - ee, z=1 - ee),  # node 11, edge
        Vertex(x=2.0 - cc, y=1.0 - cc, z=1 - cc),  # node 12, corner
    )
    assert aa == gold

    # iteration 2
    num_iters = 2  # overwrite, double iteration of smoothing

    aa2 = sm.smooth(
        vv=vv,
        nn=nn,
        node_hierarchy=nh,
        prescribed_nodes=None,
        scale_lambda=scale_lambda,
        num_iters=num_iters,
        algorithm=algo,
    )
    # define the gold standard fiducial
    gold2 = (
        (0.19, 0.1775, 0.1775),
        (1.0, 0.1425, 0.1425),
        (1.8099999999999998, 0.1775, 0.1775),
        (0.19, 0.8225, 0.1775),
        (1.0, 0.8575, 0.1425),
        (1.8099999999999998, 0.8225, 0.1775),
        (0.19, 0.1775, 0.8225),
        (1.0, 0.1425, 0.8575),
        (1.8099999999999998, 0.1775, 0.8225),
        (0.19, 0.8225, 0.8225),
        (1.0, 0.8575, 0.8575),
        (1.8099999999999998, 0.8225, 0.8225),
    )
    assert aa2 == gold2


def test_pair_ordered():
    """Unit test for pair ordered."""

    # small toy example
    given = ((3, 1), (2, 1))
    found = sm.pair_ordered(given)
    gold = ((1, 2), (1, 3))
    assert found == gold

    # example from 12 edges of a hex element
    given = (
        (1, 2),
        (2, 5),
        (4, 1),
        (5, 4),
        (7, 8),
        (8, 11),
        (11, 10),
        (10, 7),
        (1, 7),
        (2, 8),
        (5, 11),
        (4, 10),
    )  # overwrite
    gold = (
        (1, 2),
        (1, 4),
        (1, 7),
        (2, 5),
        (2, 8),
        (4, 5),
        (4, 10),
        (5, 11),
        (7, 8),
        (7, 10),
        (8, 11),
        (10, 11),
    )  # overwrite
    found = sm.pair_ordered(given)  # overwrite
    assert found == gold
    # breakpoint()


def test_edge_pairs():
    """Units test to assure edge pairs are computed correctly."""
    elements = (
        (1, 2, 5, 4, 7, 8, 11, 10),
        (2, 3, 6, 5, 8, 9, 12, 11),
    )
    found = sm.edge_pairs(hexes=elements)
    gold = (
        (1, 2),
        (1, 4),
        (1, 7),
        (2, 3),
        (2, 5),
        (2, 8),
        (3, 6),
        (3, 9),
        (4, 5),
        (4, 10),
        (5, 6),
        (5, 11),
        (6, 12),
        (7, 8),
        (7, 10),
        (8, 9),
        (8, 11),
        (9, 12),
        (10, 11),
        (11, 12),
    )
    assert found == gold
