"""This module builds on the `spheres.py` module to create high resolution,
three-material, concentric spheres and export the voxelization as a .npy
file.
"""

from pathlib import Path
from typing import Final

from matplotlib.colors import LightSource
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np


def sphere(resolution: int, dtype=np.uint8) -> np.ndarray:
    """Generate a 3D voxelized representation of three concentric spheres
        of 10, 11, and 12 cm, at a given resolution.

        Parameters
        ----------
        resolution : int
            The resolution as voxels per centimeter.  Minimum value is 1.

        dtype: data-type, optional
            The data type of the output array.  Default is np.uint8.

        Returns
        -------
        np.ndarray
            A 3D numpy array representing the voxelized spheres.  Voxels within
            the inner sphere are set to 1, the intermediate shell are set to 2,
            and the outer shell are set to 3.  Voxels outside the spheres are
            set to 0.

        Raises
        ------
        ValueError
            If the resolution is less than 1.

        Example
        -------
        >>> sphere(resolution=2) returns
            array(
                [
    #                [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    #                [[0, 1, 0], [1, 1, 1], [0, 1, 0]],
    #                [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
                ],
                dtype=uint8
            )
    """
    if resolution < 1:
        raise ValueError("Resolution must be >= 1")

    r10 = 10  # cm
    r11 = 11  # cm
    r12 = 12  # cm

    # We change the algorithm a bit here so we can exactly match the radius:
    # number of voxels per side length (nvps)
    # nvps = 2 * r12 * resolution + 1
    nvps = 2 * r12 * resolution
    vox_z, vox_y, vox_x = np.mgrid[
        -r12 : r12 : nvps * 1j,
        -r12 : r12 : nvps * 1j,
        -r12 : r12 : nvps * 1j,
    ]
    domain = vox_x**2 + vox_y**2 + vox_z**2
    mask_10_in = np.array(domain <= r10 * r10, dtype=dtype)
    mask_11_in = np.array(domain <= r11 * r11, dtype=dtype)
    mask_12_in = np.array(domain <= r12 * r12, dtype=dtype)

    mask_10_11 = mask_11_in - mask_10_in
    mask_11_12 = mask_12_in - mask_11_in

    shell_10_11 = 2 * mask_10_11
    shell_11_12 = 3 * mask_11_12

    result = mask_10_in + shell_10_11 + shell_11_12
    # breakpoint()
    return result


# User input begin
spheres = {
    "resolution_1": sphere(resolution=1),
    # "resolution_2": sphere(resolution=2),
}

aa = Path(__file__)
bb = aa.with_suffix(".png")

# vox_01_per_cm = sphere(resolution=1)
# vox_02_per_cm = sphere(resolution=2)
# vox_04_per_cm = sphere(resolution=4)
# vox_10_per_cm = sphere(resolution=10)


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
visualize: Final[bool] = True  # turn to True to show the figure on screen
serialize: Final[bool] = True  # turn to True to save .png and .npy files
# User input end

# breakpoint()
N_SUBPLOTS = len(spheres)
IDX = 1
for title, struc in spheres.items():
    # for index (key, value) in enumerate(spheres.items()):
    ax = fig.add_subplot(1, N_SUBPLOTS, IDX, projection=Axes3D.name)
    ax.voxels(
        struc,
        facecolors=colors[IDX-1],
        edgecolor=colors[IDX-1],
        alpha=voxel_alpha,
        lightsource=lightsource)
    ax.set_title(title)
    IDX += 1

    # Set labels for the axes
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("z")

    # ax.set_xlim([0, 10])

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
