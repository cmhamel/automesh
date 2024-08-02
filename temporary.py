from automesh import Spn
import numpy as np
import matplotlib.pyplot as plt


spn = Spn.from_npy('tests/input/f.npy')
exo = spn.as_exodus()

fig = plt.figure()
ax = fig.add_subplot(projection='3d')

ax.voxels(spn.data, edgecolor='k')

for i, (x, y, z) in enumerate(exo.nodal_coordinates):
    ax.scatter(z, y, x, color='k')
    ax.text(z, y, x, str(i + 1))

ax.yaxis.set_inverted(True)
plt.xlabel('z')
plt.ylabel('y')
plt.show()
