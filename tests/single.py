"""This module demonstrates creating a pixel slice in the (x, y)
plane, and a single layer in the z axis, to create a single
voxel, as a precursor for a single hexahedral finite element.

This module assumes the virtual virtual environment has been loaded:

Example:

    cd ~/autotwin/automesh
    source .venv/bin/activate
    python tests/single.py
"""

# standard library
from pathlib import Path
from typing import Final

# third-party libary
import numpy as np

# module library
# none


def main():
    """The main program."""

    # user input begin
    output_file: Final[str] = "single.npy"
    output_dir: Final[str] = "~/scratch"
    # user input end

    # constants
    # n_slices: Final[int] = (
    #     1  # The number of times (x, y) matrix is repeated along the z axis
    # )

    # if the output directory does not already exist, create it
    output_path = Path(output_dir).expanduser()
    if not output_path.exists():
        print(f"Could not find existing output directory: {output_path}")
        Path.mkdir(output_path)
        print(f"Created: {output_path}")
        assert output_path.exists()

    # the x-axis courses on the inner brackets, and then the y-axis
    # wraps all the x-axis courses, it is important that the data
    # type is `np.uint8`
    single_pixel = np.array(
        [
            [
                1,
            ],
        ],
        dtype=np.uint8,
    )

    # we stack the (x, y) plane along the z-axis
    single_voxel = np.array(
        [
            single_pixel,
        ]
    )

    # save the numpy data as a .npy file
    fout = output_path.joinpath(output_file)
    np.save(fout, single_voxel)
    print(f"Saved file: {fout}")

    # to load the array back from the .npy file,
    # use the numpy.load function:
    loaded_array = np.load(fout)

    # verify the loaded array
    print(loaded_array)

    assert loaded_array == single_voxel

    # now that the .npy file has been created and verified,
    # move it to the repo at ~/autotwin/automesh/tests/input


if __name__ == "__main__":
    main()
