# Development

## Prerequisites

* [Git](https://git-scm.com/)

## Optional

* [VS Code](https://code.visualstudio.com/) with the following extensions
  * [Python](https://marketplace.visualstudio.com/items?itemName=ms-python.python)
  * [Python Debugger](https://marketplace.visualstudio.com/items?itemName=ms-python.debugpy)
  * [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* [GitHub CLI](https://cli.github.com)

## Development Cycle Overview

* Branch
* Develop
  * `cargo build`
  * develop:
    * tests
    * implementation
  * document:
    * `mdbook build`
       * output: automesh/book/build
    * `mdbook serve`
      * interactive mode
      * on local machine, with Firefox, open the `index.html` file., e.g.,
      * `file:///Users/chovey/autotwin/automesh/book/build/index.html`
  * test:
    * `cargo test`
    * `cargo run`  // test without required input and output flags
    * `cargo run --release -- -i tests/input/f.npy -o foo.exo`
    * `cargo run -- --help`
  * precommit:
    * `pre-commit run --all-files`
  * `cargo doc --open`
* Test
  * `maturin develop --release --features python`
* Merge request
