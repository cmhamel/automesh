import matplotlib.pyplot as plt
import numpy as np


numb = ()
data = ()

with open('benches/compare/automesh_block.out', 'r') as file:
    for line in file:
        input = line.strip().split(sep=': ')
        numb += (int(input[0])**3,)
        count = 0
        sum = 0
        for entry in input[1].split():
            count += 1
            sum += float(entry)
        data += (sum / count,)


np.savetxt('benches/compare/compare.csv', np.vstack((numb, data)).T)

plt.plot(numb, data, numb, [data[0] + 185e-9 * (n - numb[0]) for n in numb])
plt.show()
