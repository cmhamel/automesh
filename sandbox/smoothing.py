"""This module illustrates test cases for smoothing algorithms.
Example:
--------
source ~/autotwin/automesh/.venv/bin/activate
python smoothing.py
"""

import datetime
from enum import Enum
from pathlib import Path
from typing import Final, NamedTuple

from matplotlib.colors import LightSource
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np

import smoothing_types as ty

# Type alias for functional style methods
# https://docs.python.org/3/library/typing.html#type-aliases
DofSet = ty.DofSet
Elements = ty.Elements
Neighbors = ty.Neighbors
Vertex = ty.Vertex
Vertices = ty.Vertices

# class Vertex(NamedTuple):
#     """A general 3D vertex with x, y, and z coordinates."""
#
#     x: float
#     y: float
#     z: float


# Vertices = tuple[Vertex, ...]
# Element = tuple[int, int, int, int, int, int, int, int]  # only hex elements
# Elements = tuple[Element, ...]
# Dof = tuple[int, int, int]
# DofSet = tuple[Dof, ...]  # analong to a SideSet or a NodeSet
# Neighbor = tuple[int, ...]
# Neighbors = tuple[Neighbor, ...]


# class DofType(Enum):
#     """All degrees of freedom must belong to one, and only one, of the
#     following smoothing categories.
#     """
#
#     PRESCRIBED_HOMOGENEOUS = 0
#     PRESCRIBED_INHOMOGENEOUS = 1
#     FREE_EXTERIOR = 2
#     FREE_INTERFACE = 3
#     FREE_INTERIOR = 4


# class SmoothingAlgorithm(Enum):
#     """The type of smoothing algorithm."""
#
#     LAPLACE = 0
#     TAUBIN = 1


def average_position(vv: Vertices) -> Vertex:
    """Give a list of vertices, returns the average position of the
    vertices."""

    n_vertices = len(vv)
    assert n_vertices > 0, "Error: number of vertices must be positive."
    xs = [v.x for v in vv]
    ys = [v.y for v in vv]
    zs = [v.z for v in vv]
    x_ave = sum(xs) / n_vertices
    y_ave = sum(ys) / n_vertices
    z_ave = sum(zs) / n_vertices

    return Vertex(x=x_ave, y=y_ave, z=z_ave)


# def test_average_position():
#     """Unit test for average_position"""
#     v1 = Vertex(x=1.0, y=2.0, z=3.0)
#     v2 = Vertex(x=4.0, y=5.0, z=6.0)
#     v3 = Vertex(x=7.0, y=8.0, z=9.0)
#
#     v_ave = average_position((v1, v2, v3))
#     assert v_ave.x == 4.0
#     assert v_ave.y == 5.0
#     assert v_ave.z == 6.0


def add(v1: Vertex, v2: Vertex) -> Vertex:
    """Returns the Vertex addition of (v1 + v2)."""
    dx = v1.x + v2.x
    dy = v1.y + v2.y
    dz = v1.z + v2.z
    return Vertex(x=dx, y=dy, z=dz)


# def test_add():
#     """Unit test for the addition of Vertex v1 and Vertex v2."""
#     v1 = Vertex(x=1.0, y=2.0, z=3.0)
#     v2 = Vertex(x=4.0, y=7.0, z=1.0)
#     vv = add(v1=v1, v2=v2)
#     assert vv.x == 5.0
#     assert vv.y == 9.0
#     assert vv.z == 4.0


def subtract(v1: Vertex, v2: Vertex) -> Vertex:
    """Returns the Vertex subtraction of (v1 - v2)."""
    dx = v1.x - v2.x
    dy = v1.y - v2.y
    dz = v1.z - v2.z
    return Vertex(x=dx, y=dy, z=dz)


# def test_subtract():
#     """Unit test for the subtraction of Vertex v2 from Vertex v1."""
#     v1 = Vertex(x=1.0, y=2.0, z=3.0)
#     v2 = Vertex(x=4.0, y=7.0, z=1.0)
#     vv = subtract(v1=v1, v2=v2)
#     assert vv.x == -3.0
#     assert vv.y == -5.0
#     assert vv.z == 2.0


def scale(vertex: Vertex, scale_factor: float) -> Vertex:
    """Scales a vertex by a scale factor."""
    x = scale_factor * vertex.x
    y = scale_factor * vertex.y
    z = scale_factor * vertex.z
    return Vertex(x=x, y=y, z=z)


# def test_scale():
#     """Unit test for the scale function."""
#     v1 = Vertex(x=1.0, y=2.0, z=3.0)
#     ss = 10.0
#     result = scale(vertex=v1, scale_factor=ss)
#     assert result.x == 10.0
#     assert result.y == 20.0
#     assert result.z == 30.0


def smooth(vv: Vertices, nn: Neighbors, ds: DofSet, sf: float) -> Vertices:
    """Given an initial position of vertices, the vertex neighbors, and the
    dof classification of each vertex, perform Laplace smoothing, and return
    the updated coordinates."""
    vertices_new = []
    for vertex, neighborhood, dof in zip(vv, nn, ds):
        print(f"vertex {vertex}, dof {dof}, neighborhood {neighborhood}")
        # for now, no hierarchical smoohing, assume all dofs are FREE_INTERIOR

        # account for zero-index instead of 1-index:
        neighbor_vertices = tuple(
            vv[x - 1] for x in neighborhood
        )  # zero index

        neighbor_average = average_position(neighbor_vertices)
        delta = subtract(v1=neighbor_average, v2=vertex)
        lambda_delta = scale(vertex=delta, scale_factor=sf)
        vertex_new = add(v1=vertex, v2=lambda_delta)
        vertices_new.append(vertex_new)
        # breakpoint()

    # breakpoint()
    return tuple(vertices_new)


# def test_laplace_smoothing(
#     vv: Vertices,
#     nn: Neighbors,
#     ds: DofSet,
#     sf_lambda: float,
# ):
#     """Unit test for Laplace smoothing with all dofs as FREE_INTERIOR."""
#     aa = smooth(vv=vv, nn=nn, ds=ds, sf=sf_lambda)
#     cc: Final[float] = sf_lambda / 3.0  # delta corner
#     ee: Final[float] = sf_lambda / 4.0  # delta edge
#     # define the gold standard fiducial
#     gold = (
#         Vertex(x=cc, y=cc, z=cc),  # node 1, corner
#         Vertex(x=1.0, y=ee, z=ee),  # node 2, edge
#         Vertex(x=2.0 - cc, y=cc, z=cc),  # node 3, corner
#         #
#         Vertex(x=cc, y=1.0 - cc, z=cc),  # node 4, corner
#         Vertex(x=1.0, y=1.0 - ee, z=ee),  # node 5, edge
#         Vertex(x=2.0 - cc, y=1.0 - cc, z=cc),  # node 6, corner
#         #
#         Vertex(x=cc, y=cc, z=1 - cc),  # node 7, corner
#         Vertex(x=1.0, y=ee, z=1 - ee),  # node 8, edge
#         Vertex(x=2.0 - cc, y=cc, z=1 - cc),  # node 9, corner
#         #
#         Vertex(x=cc, y=1.0 - cc, z=1 - cc),  # node 10, corner
#         Vertex(x=1.0, y=1.0 - ee, z=1 - ee),  # node 11, edge
#         Vertex(x=2.0 - cc, y=1.0 - cc, z=1 - cc),  # node 12, corner
#     )
#     assert aa == gold


# def test_taubin_smoothing(
#     vv: Vertices,
#     nn: Neighbors,
#     ds: DofSet,
#     sf_lambda: float,
#     sf_mu: float
# ):
#     """Unit test for Taubin smoothing with all dofs as FREE_INTERIOR."""
#     aa = smooth(vv=vv, nn=nn, ds=ds, sf=sf_lambda)
#     bb = smooth(vv=aa, nn=nn, ds=ds, sf=sf_mu)
#     breakpoint()
#     cc: Final[float] = sf / 3.0  # delta corner
#     ee: Final[float] = sf / 4.0  # delta edge
#     # define the gold standard fiducial
#     gold = (
#         Vertex(x=cc, y=cc, z=cc),  # node 1, corner
#         Vertex(x=1.0, y=ee, z=ee),  # node 2, edge
#         Vertex(x=2.0-cc, y=cc, z=cc),  # node 3, corner
#         #
#         Vertex(x=cc, y=1.0-cc, z=cc),  # node 4, corner
#         Vertex(x=1.0, y=1.0-ee, z=ee),  # node 5, edge
#         Vertex(x=2.0-cc, y=1.0-cc, z=cc),  # node 6, corner
#         #
#         Vertex(x=cc, y=cc, z=1-cc),  # node 7, corner
#         Vertex(x=1.0, y=ee, z=1-ee),  # node 8, edge
#         Vertex(x=2.0-cc, y=cc, z=1-cc),  # node 9, corner
#         #
#         Vertex(x=cc, y=1.0-cc, z=1-cc),  # node 10, corner
#         Vertex(x=1.0, y=1.0-ee, z=1-ee),  # node 11, edge
#         Vertex(x=2.0-cc, y=1.0-cc, z=1-cc),  # node 12, corner
#     )
#     assert bb == gold

# Input record begin
# Reference: DoubleX unit test
# https://autotwin.github.io/automesh/examples/unit_tests/index.html#double-x


vertices: Vertices = (
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

dofset: DofSet = (
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
    (4, 4, 4),
)

elements: Elements = (
    (1, 2, 5, 4, 7, 8, 11, 10),
    (2, 3, 6, 5, 8, 9, 12, 11),
)


def pair_ordered(ab: tuple[tuple[int, int], ...]) -> tuple:
    """Given a tuple of form ((a, b), (c, d), ...) orders all the subpairs
    such that the first index drives global order, and the second index
    drives secondary order.
    """
    firsts, seconds = zip(*ab)

    ab_ordered = ()

    for a, b in zip(firsts, seconds):
        if a < b:
            ab_ordered += ((a, b),)
        else:
            ab_ordered += ((b, a),)

    for a in firsts:
        print(f"a = {a}")

    for b in seconds:
        print(f"b = {b}")

    result = tuple(sorted(ab_ordered))
    # breakpoint()
    return result


# def test_pair_ordered():
#     """Unit test for pair ordered."""
#
#     # small toy example
#     given = ((3, 1), (2, 1))
#     found = pair_ordered(given)
#     gold = ((1, 2), (1, 3))
#     assert found == gold
#
#     # example from 12 edges of a hex element
#     given = (
#         (1, 2),
#         (2, 5),
#         (4, 1),
#         (5, 4),
#         (7, 8),
#         (8, 11),
#         (11, 10),
#         (10, 7),
#         (1, 7),
#         (2, 8),
#         (5, 11),
#         (4, 10),
#     )  # overwrite
#     gold = (
#         (1, 2),
#         (1, 4),
#         (1, 7),
#         (2, 5),
#         (2, 8),
#         (4, 5),
#         (4, 10),
#         (5, 11),
#         (7, 8),
#         (7, 10),
#         (8, 11),
#         (10, 11),
#     )  # overwrite
#     found = pair_ordered(given)  # overwrite
#     assert found == gold
#     # breakpoint()


def edge_pairs(ees: Elements):
    """Returns all the line pairs from element connectivity, for use
    with drawing edges of elements."""

    # almost perfect with collecting unique pairs, but there are some
    # overlapping pairs, not a big dealbptt
    pairs = ()
    for ee in ees:
        # bottom_face = tuple(sorted(list(zip(ee[0:4], ee[1:4] + (ee[0],)))))
        bottom_face = pair_ordered(tuple(zip(ee[0:4], ee[1:4] + (ee[0],))))
        # top_face = tuple(list(zip(ee[4:8], ee[5:8] + (ee[4],))))
        top_face = pair_ordered(tuple(zip(ee[4:8], ee[5:8] + (ee[4],))))
        verticals = pair_ordered(
            (
                (ee[0], ee[4]),
                (ee[1], ee[5]),
                (ee[2], ee[6]),
                (ee[3], ee[7]),
            )
        )
        t3 = bottom_face + top_face + verticals
        pairs = pairs + tuple(t3)
        # breakpoint()

    return tuple(sorted(set(pairs)))


# def test_edge_pairs():
#     """Units test to assure edge pairs are computed correctly."""
#     elements = (
#         (1, 2, 5, 4, 7, 8, 11, 10),
#         (2, 3, 6, 5, 8, 9, 12, 11),
#     )
#     found = edge_pairs(ees=elements)
#     gold = ((1, 2), (1, 4), (1, 7), (2, 3), (2, 5), (2, 8), (3, 6), (3, 9), (4, 5), (4, 10), (5, 6), (5, 11), (6, 12), (7, 8), (7, 10), (8, 9), (8, 11), (9, 12), (10, 11), (11, 12))
#     breakpoint()
#     assert found == gold


neighbors: Neighbors = (
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

SCALE_LAMBDA: Final[float] = 0.3  # lambda parameter for Laplace smoothing
SCALE_MU: Final[float] = -0.4  # mu parameter for Taubin smoothing

# Visualization
# width, height = 8, 4
width, height = 6, 3
fig = plt.figure(figsize=(width, height))
# fig = plt.figure(figsize=(8, 8))

el, az, roll = 63, -110, 0
cmap = plt.get_cmap(name="tab10")
# num_colors = len(spheres)
num_colors = 10
voxel_alpha: Final[float] = 0.9

colors = cmap(np.linspace(0, 1, num_colors))
lightsource = LightSource(azdeg=325, altdeg=45)  # azimuth, elevation
# lightsource = LightSource(azdeg=325, altdeg=90)  # azimuth, elevation
# OUTPUT_DIR: Final[Path] = Path(__file__).parent
DPI: Final[int] = 300  # resolution, dots per inch
SHOW: Final[bool] = False  # turn to True to show the figure on screen
SAVE: Final[bool] = False  # turn to True to save .png and .npy files

# output_png_short = ex.file_stem + ".png"
# output_png: Path = (
#     Path(output_dir).expanduser().joinpath(output_png_short)
# )

aa = Path(__file__)
bb = aa.with_suffix(".png")

nx, ny, nz = 2, 1, 1
nzp, nyp, nxp = nz + 1, ny + 1, nx + 1

# Input record end

# Process data
# test_average_position()
# test_add()
# test_subtract()
# test_scale()
# test_laplace_smoothing(
#     vv=vertices,
#     nn=neighbors,
#     ds=dofset,
#     sf_lambda=SCALE_LAMBDA,
# )
# test_pair_ordered()
# test_edge_pairs()

# test_taubin_smoothing(
#     vv=vertices,
#     nn=neighbors,
#     ds=dofset,
#     sf_lambda=SCALE_LAMBDA,
#     sf_mu=SCALE_MU,
# )

vertices_laplce = smooth(vv=vertices, nn=neighbors, ds=dofset, sf=SCALE_LAMBDA)

ax = fig.add_subplot(1, 2, 1, projection="3d")  # r1, c2, 1st subplot
ax2 = fig.add_subplot(1, 2, 2, projection="3d")  # r1, c2, 2nd subplot

xs = [v.x for v in vertices]
ys = [v.y for v in vertices]
zs = [v.z for v in vertices]

xs_l = [v.x for v in vertices_laplce]
ys_l = [v.y for v in vertices_laplce]
zs_l = [v.z for v in vertices_laplce]

# draw edge lines, #TODO
# xl =

ax.scatter(
    xs,
    ys,
    zs,
    s=20,
    facecolors="blue",
    edgecolors="none",
)

# repeat with lighter color on second axis
ax2.scatter(
    xs,
    ys,
    zs,
    s=20,
    facecolors="blue",
    edgecolors="none",
    alpha=0.5,
)

ax2.scatter(
    xs_l,
    ys_l,
    zs_l,
    s=20,
    facecolors="red",
    edgecolors="none",
)

# Set labels for the axes
ax.set_xlabel("x")
ax.set_ylabel("y")
ax.set_zlabel("z")
# repeat for the 2nd axis
ax2.set_xlabel("x")
ax2.set_ylabel("y")
ax2.set_zlabel("z")

x_ticks = list(range(nxp))
y_ticks = list(range(nyp))
z_ticks = list(range(nzp))

ax.set_xticks(x_ticks)
ax.set_yticks(y_ticks)
ax.set_zticks(z_ticks)
# repeat for the 2nd axis
ax2.set_xticks(x_ticks)
ax2.set_yticks(y_ticks)
ax2.set_zticks(z_ticks)

ax.set_xlim(float(x_ticks[0]), float(x_ticks[-1]))
ax.set_ylim(float(y_ticks[0]), float(y_ticks[-1]))
ax.set_zlim(float(z_ticks[0]), float(z_ticks[-1]))
# repeat for the 2nd axis
ax2.set_xlim(float(x_ticks[0]), float(x_ticks[-1]))
ax2.set_ylim(float(y_ticks[0]), float(y_ticks[-1]))
ax2.set_zlim(float(z_ticks[0]), float(z_ticks[-1]))


# Set the camera view
ax.set_aspect("equal")
ax.view_init(elev=el, azim=az, roll=roll)
# repeat for the 2nd axis
ax2.set_aspect("equal")
ax2.view_init(elev=el, azim=az, roll=roll)

# Add a footnote
# Get the current date and time in UTC
now_utc = datetime.datetime.now(datetime.UTC)
# Format the date and time as a string
timestamp_utc = now_utc.strftime("%Y-%m-%d %H:%M:%S UTC")
fn = f"Figure: {bb.name} "
fn += f"created with {__file__}\non {timestamp_utc}."
fig.text(0.5, 0.01, fn, ha="center", fontsize=8)

# fig.tight_layout()  # don't use as it clips the x-axis label
if SHOW:
    plt.show()

    if SAVE:
        fig.savefig(bb, dpi=DPI)
        print(f"Saved: {bb}")

print("End of script.")
