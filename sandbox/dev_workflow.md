# Dev Workflow

*Work in progress.*

## Configuration

* GitHub CLI https://cli.github.com
* VS Code
  * TODO Rust Analyzer extension

## Development Cycle

* Branch
* Develop
  * `cargo build`
  * develop tests
  * develop implementation
  * test:
    * `cargo test`
    * `cargo run`  // test without required input and output flags
    * `cargo run --release -- -i tests/input/f.npy -o foo.exo`
    * `cargo run -- --help`
  * precommit: `pre-commit run --all-files`
  * `cargo doc --open`
* Test
  * `maturin develop --release --features python`
* Merge request
