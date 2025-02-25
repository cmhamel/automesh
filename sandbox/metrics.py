"""This script is a quality control tool for the metrics of a mesh."""

from typing import Final

import numpy as np

nodal_coordinates = (
    (-0.2, 1.2, -0.1),  # single_valence_04_noise2.inp begin
    (1.180501, 0.39199, 0.3254445),
    (0.1, 0.2, 0.3),
    (-0.001, -0.021, 1.002),
    (1.2, -0.1, 1.1),
    (1.03, 1.102, -0.25),
    (0.0, 1.0, 1.0),
    (1.01, 1.02, 1.03),  # single_valence_04_noise2.inp end
    (0.0, 0.0, 1.0),  # one_facet.stl begin
    (0.0, 0.0, 0.0),
    (1.0, 0.0, 0.0),  # one_facet.stl end
    (-2.0, 0.0, 0.0),  # equilateral with edge length 4.0 start
    (2.0, 0.0, 0.0),
    (0.0, 2.0 * np.sqrt(3.0), 0.0),  # equilateral with edge length 4.0 end
    (-0.5, 0.0, 0.0),  # equilateral with edge length 1.0 start
    (0.5, 0.0, 0.0),
    (0.0, np.sqrt(3.0) / 2.0, 0.0),  # equilateral with edge length 1.0 end
)

element_node_connectivity = (
    (1, 2, 3),  # single_valence_04_noise2.inp begin
    (4, 2, 5),
    (1, 6, 2),
    (4, 3, 2),
    (4, 1, 3),
    (4, 7, 1),
    (2, 8, 5),
    (6, 8, 2),
    (7, 8, 6),
    (1, 7, 6),
    (4, 5, 8),
    (7, 4, 8),  # single_valence_04_noise2.inp end
    (9, 10, 11),  # one_facet.stl
    (12, 13, 14),  # equilateral triangle with side length 4.0
    (15, 16, 17),  # equilateral triangle with side length 1.0
)

NODE_NUMBERING_OFFSET: Final[int] = 1

mesh_element_max_edge_lengths = []
mesh_element_edge_ratios = []
mesh_element_minimum_angles = []
mesh_element_maximum_skews = []
mesh_element_areas = []
mesh_element_jacobians = []


def angle(a: np.ndarray, b: np.ndarray) -> float:
    """Given two vectors, find the angle between them."""
    dot_product = np.dot(a, b)
    norm_a = np.linalg.norm(a)
    norm_b = np.linalg.norm(b)

    cos_theta = dot_product / (norm_a * norm_b)

    angle_radians = np.arccos(cos_theta)
    angle_degees = np.degrees(angle_radians)

    return angle_degees


for element in element_node_connectivity:
    print(f"element with nodes: {element}")
    path = element + (element[0],)
    # print(f"  node path {path}")
    pairs = tuple(zip(element, element[1:] + (element[0],)))
    print(f"  node pairs {pairs}")
    element_edge_ratios = []
    element_minimum_angles = []
    edge_vectors = ()
    # edge ratios
    for pair in pairs:
        print(f"    pair {pair}")
        aa, bb = pair
        edge = np.array(nodal_coordinates[bb - NODE_NUMBERING_OFFSET]) - np.array(
            nodal_coordinates[aa - NODE_NUMBERING_OFFSET]
        )
        edge_vectors = edge_vectors + (edge,)
        edge_length = np.linalg.norm(edge)
        # print(f"    lens {edge_length}")
        element_edge_ratios.append(edge_length)

    print(f"  edge vectors {edge_vectors}")

    # print(f"  edge ratios {element_edge_ratios}")

    # edge ratios
    len_max = max(element_edge_ratios)
    # print(f"  max edge ratio {len_max}")
    mesh_element_max_edge_lengths.append(len_max)

    len_min = min(element_edge_ratios)
    # print(f"  min edge ratio {len_min}")
    ratio = len_max / len_min
    mesh_element_edge_ratios.append(ratio)

    # edge vectors and then angles
    edge_vectors_pairs = tuple(
        zip(edge_vectors, edge_vectors[1:] + (edge_vectors[0],))
    )
    # print(f"  edge vectors pairs {edge_vectors_pairs}")

    for item in edge_vectors_pairs:
        # print(f"    edge vectors pair {item}")
        # flip the direction of the first vector so that it shares an origin
        # with the secon vector
        angle_degrees = angle(-1.0 * item[0], item[1])
        # print(f"    angle {angle_degrees}")
        element_minimum_angles.append(angle_degrees)
    # print(f"  minimum angles {element_minimum_angles}")
    angle_min = min(element_minimum_angles)
    # print(f"  min angle {angle_min}")
    mesh_element_minimum_angles.append(angle_min)

    skew_max = (60.0 - angle_min) / 60.0
    mesh_element_maximum_skews.append(skew_max)

    # Compute areas only for triangles for now.
    if len(element) == 3:
        # area of a triangle
        aa = np.linalg.norm(edge_vectors[0])
        bb = np.linalg.norm(edge_vectors[1])
        cc = np.linalg.norm(edge_vectors[2])
        # Calculate the semi-perimeter
        ss = (aa + bb + cc) / 2.0
        # Use Heron's formula to calcuate the area
        area = np.sqrt(ss * (ss - aa) * (ss - bb) * (ss - cc))
        mesh_element_areas.append(area)

        # Interpretation:
        # The absolute value of the determinant of the Jacobian
        # represents the area of the triangle in the physical space
        # when integrating over the reference triangle.
        msj = (2.0 * np.sqrt(3.0) * area) / (3.0 * len_max)
        mesh_element_jacobians.append(msj)

print(f"\nmesh element max edge lengths: {mesh_element_max_edge_lengths}")
print(f"\nmesh element edge ratios: {mesh_element_edge_ratios}")
print(f"\nmesh element minimum angles: {mesh_element_minimum_angles}")
print(f"\nmesh element maximum skews: {mesh_element_maximum_skews}")
print(f"\nmesh element areas: {mesh_element_areas}")
print(f"\nmesh minimum scaled Jacobians: {mesh_element_jacobians}")
