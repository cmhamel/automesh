"""This module contains the core computations for smoothing algorithms.
"""

# import sandbox.smoothing_types as ty
import smoothing_types as ty


# Type alias for functional style methods
# https://docs.python.org/3/library/typing.html#type-aliases
# DofSet = ty.DofSet
Hexes = ty.Hexes
Hierarchy = ty.Hierarchy
Neighbors = ty.Neighbors
NodeHierarchy = ty.NodeHierarchy
PrescribedNodes = ty.PrescribedNodes
Vertex = ty.Vertex
Vertices = ty.Vertices
SmoothingAlgorithm = ty.SmoothingAlgorithm


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


def add(v1: Vertex, v2: Vertex) -> Vertex:
    """Returns the Vertex addition of (v1 + v2)."""
    dx = v1.x + v2.x
    dy = v1.y + v2.y
    dz = v1.z + v2.z
    return Vertex(x=dx, y=dy, z=dz)


def subtract(v1: Vertex, v2: Vertex) -> Vertex:
    """Returns the Vertex subtraction of (v1 - v2)."""
    dx = v1.x - v2.x
    dy = v1.y - v2.y
    dz = v1.z - v2.z
    return Vertex(x=dx, y=dy, z=dz)


def scale(vertex: Vertex, scale_factor: float) -> Vertex:
    """Scales a vertex by a scale factor."""
    x = scale_factor * vertex.x
    y = scale_factor * vertex.y
    z = scale_factor * vertex.z
    return Vertex(x=x, y=y, z=z)


def xyz(v1: Vertex) -> tuple[float, float, float]:
    """Given a vertex, returns the coordinates as (x, y, z)."""
    aa, bb, cc = v1.x, v1.y, v1.z
    return (aa, bb, cc)


def smoothing_neighbors(neighbors: Neighbors, node_hierarchy: NodeHierarchy):
    """Given an original neighbors structure, defined from connectivity
    of the mesh, and given a node_hierarchy, return the neighbors that are
    used for smoothing, which will be a subset of the original neighbors
    structure."""
    neighbors_new = ()

    for node, level in enumerate(node_hierarchy):
        nei_old = neighbors[node]
        # print(f"Processing node {node+1}, neighbors: {nei_old}")
        # node_level = level.value
        levels = [int(node_hierarchy[x - 1].value) for x in nei_old]
        nei_new = ()

        # breakpoint()
        match level:
            case Hierarchy.INTERIOR:
                # print("INTERIOR node")
                nei_new = nei_old
            case Hierarchy.BOUNDARY:
                # print("BOUNDARY node")
                for nn, li in zip(nei_old, levels):
                    if li >= level.value:
                        nei_new += (nn,)
            case Hierarchy.PRESCRIBED:
                # print("PRESCRIBED node")
                nei_new = ()
            case _:
                raise ValueError("Hierarchy value must be in [0, 1, 2]")

        neighbors_new += (nei_new,)

    return neighbors_new


def smooth(
    vv: Vertices,
    nn: Neighbors,
    node_hierarchy: NodeHierarchy,
    prescribed_nodes: PrescribedNodes,
    scale_lambda: float,
    num_iters: int,
    algorithm: SmoothingAlgorithm,
) -> Vertices:
    """Given an initial position of vertices, the vertex neighbors,
    and the dof classification of each vertex, perform Laplace
    smoothing for num_iter iterations, and return the updated
    coordinates.
    """
    print(f"Smoothing algorithm: {algorithm.value}")

    assert num_iters >= 1, "`num_iters` must be 1 or greater"

    # if the node_hierarchy contains a Hierarchy.PRESCRIBED type; or
    # the the PrescribedNodes must not be None
    if Hierarchy.PRESCRIBED in node_hierarchy:
        info = "Smoothing algorithm with hierarchical control"
        info += " and PRESCRIBED node positions."
        print(info)
        estr = "Error, NodeHierarchy desigates PRESCRIBED nodes, but no values"
        estr += " for (x, y, z) prescribed positions were given."
        assert prescribed_nodes is not None, estr

        n_nodes_prescribed = node_hierarchy.count(Hierarchy.PRESCRIBED)
        n_prescribed_xyz = len(prescribed_nodes)
        estr = f"Error: number of PRESCRIBED nodes: {n_nodes_prescribed}"
        estr += " must match the number of"
        estr += f" prescribed Vertices(x, y, z): {n_prescribed_xyz}"
        assert n_nodes_prescribed == n_prescribed_xyz, estr

        # update neighbors
        nn = smoothing_neighbors(
            neighbors=nn,
            node_hierarchy=node_hierarchy
        )  # overwrite

        # update vertex positions
        vv_list = list(vv)  # make mutable
        for (node_id, node_xyz) in prescribed_nodes:
            # print(f"Update node {node_id}")
            # print(f"  from {vv_list[node_id-1]}")
            # print(f"  to {node_xyz}")
            vv_list[node_id - 1] = node_xyz  # zero index, overwrite xyz

        # revert to immutable
        vv = tuple(vv_list)  # overwrite

    vertices_old = vv

    # breakpoint()
    for k in range(num_iters):

        print(f"Iteration: {k+1}")
        vertices_new = []

        for vertex, neighbors in zip(vertices_old, nn):
            # debug vertex by vertex
            # print(f"vertex {vertex}, neighbors {neighbors}")

            # account for zero-index instead of 1-index:
            neighbor_vertices = tuple(
                vertices_old[i - 1] for i in neighbors
            )  # zero index

            if len(neighbor_vertices) > 0:
                neighbor_average = average_position(neighbor_vertices)
                delta = subtract(v1=neighbor_average, v2=vertex)
                lambda_delta = scale(vertex=delta, scale_factor=scale_lambda)
                vertex_new = add(v1=vertex, v2=lambda_delta)
            elif len(neighbor_vertices) == 0:
                # print("Prescribed node, no smoothing update.")
                vertex_new = vertex
            else:
                estr = "Error: neighbor_vertices negative length"
                raise ValueError(estr)

            vertices_new.append(vertex_new)
            # breakpoint()

        vertices_old = vertices_new  # overwrite for new k loop

    # breakpoint()
    return tuple(vertices_new)


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

    # for a in firsts:
    #     print(f"a = {a}")

    # for b in seconds:
    #     print(f"b = {b}")

    result = tuple(sorted(ab_ordered))
    # breakpoint()
    return result


def edge_pairs(hexes: Hexes):
    """Returns all the line pairs from hex element connectivity, for use
    with drawing edges of elements."""

    # almost perfect with collecting unique pairs, but there are some
    # overlapping pairs, not a big dealbptt
    pairs = ()
    for ee in hexes:
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
