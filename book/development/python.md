# Python Development

Prior to completing items below, [install Rust](rust.md).

## Python Installation

[Install](https://www.python.org/downloads/) a Python version [supported](https://github.com/autotwin/automesh/blob/main/pyproject.toml) by `Autotwin`.

## Create a Virtual Environment

Note: If a virtual environment folder `automesh/.venv` already exists from previous installs, then remove it as follows:

```bash
(.venv) deactivate            # if the virtual environment is currently active
rm -rf automesh/.venv  # remove the virtual environment folder with `rm -rf .venv/`.

python -m venv .venv  # create the virtual environment

# activate the venv with one of the following:
source .venv/bin/activate       # for bash shell
source .venv/bin/activate.csh   # for c shell
source .venv/bin/activate.fish  # for fish shell
.\.venv\Scripts/activate        # for powershell

pip install --upgrade pip

pip install maturin

# [deprecated] pip install -e .[dev]  # install the module as editable with development optional dependencies

maturin develop --features python --extras dev
```

## Build and Test the Source Code

```bash
pytest -v
```
