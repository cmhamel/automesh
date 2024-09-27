# Command Line Interface

```sh
automesh --help
<!-- cmdrun automesh --help -->
```

## Example

Convert a Numpy segmentation file to an Abaqus input file:

```sh
automesh --input single.npy --output single.inp
```

<!-- cmdrun wget https://github.com/autotwin/automesh/raw/main/tests/input/single.npy -O ../../target/single_for_cli.npy -->
<!-- cmdrun automesh --input ../../target/single_for_cli.npy --output ../../target/single_from_cli.inp -->

The resulting Abaqus input file:

```sh
<!-- cmdrun cat ../../target/single_from_cli.inp -->
```
