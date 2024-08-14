"""This module demonstrates creating a pixel slice in the (x, y)
plane, and a single layer in the z axis, to create a single
voxel, as a precursor for a single hexahedral finite element.

This module assumes the virtual environment has been loaded.

Example:

    cd ~/autotwin/automesh
    source .venv/bin/activate
    python tests/single.py

Ouput:
    The `output_npy` file data structure
    The `output_png` file visualization
"""

# standard library
from pathlib import Path
from typing import Final

# third-party libary
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
from PIL import Image

# module library
# none


def main():
    """The main program."""

    # user input begin
    # computation
    output_dir: Final[str] = "~/scratch"
    file_stem = "single"
    output_npy: Final[Path] = (
        Path(output_dir).expanduser().joinpath(file_stem + ".npy")
    )

    # visualization
    visualize: bool = True  # True performs post-processing visualization
    dpi: Final[int] = 150  # resolution, dots per inch
    output_png: Final[Path] = (
        Path(output_dir).expanduser().joinpath(file_stem + ".png")
    )
    el, az, roll = 25, -115, 0
    # user input end

    # io: if the output directory does not already exist, create it
    output_path = Path(output_dir).expanduser()
    if not output_path.exists():
        print(f"Could not find existing output directory: {output_path}")
        Path.mkdir(output_path)
        print(f"Created: {output_path}")
        assert output_path.exists()

    # the x-axis courses on the inner brackets, and then the y-axis
    # wraps all the x-axis courses, it is important that the data
    # type is `np.uint8`
    pixels = np.array(
        [
            [
                1,
            ],
        ],
        dtype=np.uint8,
    )

    # we stack the (x, y) plane along the z-axis
    voxels = np.array(
        [
            pixels,
        ]
    )
    assert voxels.shape == (1, 1, 1)
    nelz, nely, nelx = voxels.shape

    # save the numpy data as a .npy file
    np.save(output_npy, voxels)
    print(f"Saved: {output_npy}")

    # to load the array back from the .npy file,
    # use the numpy.load function:
    loaded_array = np.load(output_npy)

    # verify the loaded array
    print(loaded_array)

    assert loaded_array == voxels

    # now that the .npy file has been created and verified,
    # move it to the repo at ~/autotwin/automesh/tests/input

    if not visualize:
        return

    # visualization

    # Define the dimensions of the lattice
    nx, ny, nz = (nelx + 1, nely + 1, nelz + 1)

    # Create a figure and a 3D axis
    fig = plt.figure()
    ax = fig.add_subplot(111, projection="3d")

    # Generate the lattice points
    x = []
    y = []
    z = []
    labels = []

    lattice_number = 0
    for k in range(nz):
        for j in range(ny):
            for i in range(nx):
                lattice_number += 1
                x.append(i)
                y.append(j)
                z.append(k)
                labels.append(f"{lattice_number}: ({i},{j},{k})")

    # Plot the lattice coordinates
    ax.scatter(x, y, z, c="blue", marker="o")

    # Label the lattice coordinates
    for idx, label in enumerate(labels):
        ax.text(x[idx], y[idx], z[idx], label, color="red")

    # Set labels for the axes
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("z")

    x_ticks = list(range(nx))
    y_ticks = list(range(ny))
    z_ticks = list(range(nz))

    ax.set_xticks(x_ticks)
    ax.set_yticks(y_ticks)
    ax.set_zticks(z_ticks)

    ax.set_xlim(x_ticks)
    ax.set_ylim(y_ticks)
    ax.set_zlim(z_ticks)

    # Set the camera view
    ax.set_aspect("equal")
    ax.view_init(elev=el, azim=az, roll=roll)

    # Set the title
    ax.set_title("Lattice Global Node Numbers and Coordinates")

    # Show the plot
    plt.show()

    # plt.show()
    fig.savefig(output_png, dpi=dpi)
    print(f"Saved: {output_png}")


if __name__ == "__main__":
    main()
