"""This module demonstrates how to create a voxelized sphere and export
it as a .npy file.
"""

import matplotlib.pyplot as plt
from matplotlib.colors import LightSource
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
from pathlib import Path
from typing import Final


def sphere(radius: int, dtype=np.uint8) -> np.ndarray:
    """Generate a 3D voxelized representation of a sphere.

    Parameters
    ----------
    radius: int
        The radius of the sphere.  Minimum value is 1.

    dtype: data-type, optional
        The data type of the output array.  Default is np.uint8.

    Returns
    -------
    np.ndarray
        A 3D numpy array of shape (2*radius+1, 2*radius+1, 2*radius+1)
        representing the voxelized sphere.  Voxels within the sphere are
        set to 1, and those outside are set to 0.

    Raises
    ------
    ValueError
        If the radius is less than 1.

    Example
    -------
    >>> sphere(radius=1) returns
        array(
            [
                [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
                [[0, 1, 0], [1, 1, 1], [0, 1, 0]],
                [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
            ],
            dtype=uint8
        )

    Reference
    ---------
    Adapted from:
    https://github.com/scikit-image/scikit-image/blob/v0.24.0/skimage/morphology/footprints.py#L763-L833
    """
    if radius < 1:
        raise ValueError("Radius must be >= 1")

    n_voxels_per_side = 2 * radius + 1
    vox_z, vox_y, vox_x = np.mgrid[
        -radius:radius:n_voxels_per_side * 1j,
        -radius:radius:n_voxels_per_side * 1j,
        -radius:radius:n_voxels_per_side * 1j,
    ]
    voxel_radius_squared = vox_x**2 + vox_y**2 + vox_z**2
    result = np.array(voxel_radius_squared <= radius * radius, dtype=dtype)
    return result


# User input begin

spheres = {
    "sphere_1": sphere(radius=1),
    "sphere_3": sphere(radius=3),
    "sphere_5": sphere(radius=5),
}

aa = Path(__file__)
bb = aa.with_suffix(".png")

# Visualize the elements.
# width, height = 8, 4
width, height = 6, 3
fig = plt.figure(figsize=(width, height))
# fig = plt.figure(figsize=(8, 8))

el, az, roll = 63, -110, 0
cmap = plt.get_cmap(name="tab10")
num_colors = len(spheres)
voxel_alpha: Final[float] = 0.9

colors = cmap(np.linspace(0, 1, num_colors))
lightsource = LightSource(azdeg=325, altdeg=45)  # azimuth, elevation
# lightsource = LightSource(azdeg=325, altdeg=90)  # azimuth, elevation
dpi: Final[int] = 300  # resolution, dots per inch
visualize: Final[bool] = False  # turn to True to show the figure on screen
serialize: Final[bool] = False  # turn to True to save .png and .npy files
# User input end


idx = 1
for title, struc in spheres.items():
    # for index (key, value) in enumerate(spheres.items()):
    ax = fig.add_subplot(1, 3, idx, projection=Axes3D.name)
    ax.voxels(
        struc,
        facecolors=colors[idx-1],
        edgecolor=colors[idx-1],
        alpha=voxel_alpha,
        lightsource=lightsource)
    ax.set_title(title)
    idx += 1

    # Set labels for the axes
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("z")

    # Set the camera view
    ax.set_aspect("equal")
    ax.view_init(elev=el, azim=az, roll=roll)

    if serialize:
        cc = aa.with_stem("spheres_" + title)
        dd = cc.with_suffix(".npy")
        # Save the data in .npy format
        np.save(dd, struc)
        print(f"Saved: {dd}")

fig.tight_layout()
if visualize:
    plt.show()

if serialize:
    fig.savefig(bb, dpi=dpi)
    print(f"Saved: {bb}")
