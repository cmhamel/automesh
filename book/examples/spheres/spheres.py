"""This module demonstrates how to create a voxelized sphere and export
it as a .npy file.

Items that need to be installed into the virtual environment:
    pip install scikit-image

"""

import matplotlib.pyplot as plt
from matplotlib.colors import LightSource
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
from pathlib import Path
from typing import Final

from skimage.morphology import ball
spheres = {
    "ball_1": ball(radius=1),
    "ball_3": ball(radius=3),
    "ball_5": ball(radius=5),
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

idx = 1
for title, struc in spheres.items():
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

    cc = aa.with_stem("spheres_" + title + "_")
    dd = cc.with_suffix(".npy")
    # Save the data in .npy format
    np.save(dd, struc)
    print(f"Saved: {dd}")

fig.tight_layout()
plt.show()

# plt.show()
fig.savefig(bb, dpi=dpi)
print(f"Saved: {bb}")
