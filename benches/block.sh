#!/bin/bash

python -c 'import numpy as np; np.save("benches/block_8.npy", np.ones((8, 8, 8)).astype("uint8"))'
python -c 'import numpy as np; np.save("benches/block_16.npy", np.ones((16, 16, 16)).astype("uint8"))'
python -c 'import numpy as np; np.save("benches/block_32.npy", np.ones((32, 32, 32)).astype("uint8"))'

cargo run -qr -- convert -i benches/block_8.npy -o benches/block_8.spn
cargo run -qr -- convert -i benches/block_16.npy -o benches/block_16.spn
cargo run -qr -- convert -i benches/block_32.npy -o benches/block_32.spn
