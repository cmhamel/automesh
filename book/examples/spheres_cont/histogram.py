import matplotlib.pyplot as plt
import numpy as np

from pathlib import Path

# Generate some data
filenames = [
    "res_2_iter_05.csv",
    "res_3_iter_05.csv",
    "res_4_iter_05.csv",
]

for filename in filenames:

    with open(filename, "r") as file:
        data = file.read()

    aa = data.strip().split("\n")
    bb = [float(x) for x in aa]

    # Create the histogram
    plt.hist(bb, bins=20, color='blue', alpha=0.7, log=True)

    # Add labels and title
    plt.xlabel("Minimum Scaled Jacobian (MSJ)")
    plt.ylabel("Frequency")
    plt.title(f"{filename}")

    # Show the plot
    # plt.show()

    # Save the plot
    fn = Path(filename).stem + "_msj" + ".png"
    plt.savefig(fn)
    print(f"Saved file: {fn}")

    # Clear the current figure
    plt.clf()
