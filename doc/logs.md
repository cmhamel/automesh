# logs

## 2024-07-03

* [Exodus II file format](exodus.md)
* weekly interval pair programming Wed 1100-1300 EST (0900-1100 MST)
* repo updates
* iterators are great, https://doc.rust-lang.org/std/iter/trait.Iterator.html
* pre-commit, prevent a local from commiting prior to push
* PyO3 is the rust package for Python binding in Rust
* muturin is the packager

```bash
python -m pip install --upgrade pip
pip install maturin
maturin develop --release --extras dev
# pip install pre-commit # already installed with maturin
pre-commit install
```

**Decision:** Pause use of PyO3 to wrap Rust and expose as a Python function.  Develop a pure Rust command line program, and use as a `subprocess`, e.g.,

```bash
# example
import subprocess

result = subprocess.run([MD5_BINARY, fin], check=False, stdout=subprocess.PIPE)
        output = result.stdout.decode("utf-8")
```

## 2024-07-05

* [Maturin demo](https://github.com/hovey/mwe/tree/main/maturin)
