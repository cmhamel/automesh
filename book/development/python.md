# Python Development

[Install Rust](rust.md) prior to completing items below.

## Install Python

[Install](https://www.python.org/downloads/) a Python version [supported](https://github.com/autotwin/automesh/blob/main/pyproject.toml) by `automesh`.

## Create a Virtual Environment

Note: If a virtual environment folder `automesh/.venv` already exists from previous installs, then remove it as follows:

```sh
cd automesh                     # from the automesh directory
(.venv) deactivate              # if the virtual environment is currently active
rm -rf .venv                    # remove the virtual environment folder
                                # with `rm -rf .venv/`.

python -m venv .venv            # create the virtual environment

# activate the venv with one of the following:
source .venv/bin/activate       # for bash shell
source .venv/bin/activate.csh   # for c shell
source .venv/bin/activate.fish  # for fish shell
.\.venv\Scripts/activate        # for powershell

pip install --upgrade pip

pip install maturin
```

## Build and Test the Source Code

```sh
maturin develop --features python --extras dev

pytest
```

## Lint the Source Code

```sh
cargo clippy --features python

pycodestyle --exclude=.venv,book,docs .
```

## Build and Open the API Documentation

```sh
pdoc automesh --math --no-show-source --template-directory docs/
```
