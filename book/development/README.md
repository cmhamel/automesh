# Development

## Prerequisites

* [Git](https://git-scm.com/)

## Optional

* [VS Code](https://code.visualstudio.com/) with the following extensions
  * [Python](https://marketplace.visualstudio.com/items?itemName=ms-python.python)
  * [Python Debugger](https://marketplace.visualstudio.com/items?itemName=ms-python.debugpy)
  * [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* [GitHub CLI](https://cli.github.com)

## Development Cycle (to be refactored soon)

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
