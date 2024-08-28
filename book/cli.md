# Command Line Interface

```sh
automesh --help

     @@@@@@@@@@@@@@@@
      @@@@  @@@@@@@@@@
     @@@@  @@@@@@@@@@@
    @@@@  @@@@@@@@@@@@    Automesh: Automatic mesh generation
      @@    @@    @@      Chad B. Hovey <chovey@sandia.gov>
      @@    @@    @@      Michael R. Buche <mrbuche@sandia.gov>
    @@@@@@@@@@@@  @@@
    @@@@@@@@@@@  @@@@     Notes:
    @@@@@@@@@@ @@@@@ @    - Input/output file types are inferred.
     @@@@@@@@@@@@@@@@     - Scaling is applied before translation.


Usage: automesh [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>            Name of the NumPy (.npy) or SPN (.spn) input file
  -o, --output <OUTPUT>          Name of the Exodus (.exo) or Abaqus (.inp) output file
  -x, --nelx <NELX>              Number of voxels in the x-direction [default: 0]
  -y, --nely <NELY>              Number of voxels in the y-direction [default: 0]
  -z, --nelz <NELZ>              Number of voxels in the z-direction [default: 0]
      --xscale <XSCALE>          Scaling in the x-direction [default: 1]
      --yscale <YSCALE>          Scaling in the y-direction [default: 1]
      --zscale <ZSCALE>          Scaling in the z-direction [default: 1]
      --xtranslate <XTRANSLATE>  Translation in the x-direction [default: 0]
      --ytranslate <YTRANSLATE>  Translation in the y-direction [default: 0]
      --ztranslate <ZTRANSLATE>  Translation in the z-direction [default: 0]
  -h, --help                     Print help
  -V, --version                  Print version
```
