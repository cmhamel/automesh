# Sandbox

`Sandbox` is a folder within the `Automesh` module used for experimentation and development of some `matplotlib` figures, used for documentation.

## Configuration

```bash
# if any virtual environment is active, deactivate it
deactivate

# remove any preexisting virtual environment
cd ~/autotwin/automesh/sandbox/
rm -rf .venv/

# install a new virtual environment
python3.11 -m venv .venv

# activate the virtual environment
source .venv/bin/activate       # for bash shell
source .venv/bin/activate.csh   # for c shell
source .venv/bin/activate.fish  # for fish shell
.\.venv\Scripts/activate        # for powershell

# upgrade the base install
pip install --upgrade pip setuptools
```

Create and update the `automesh/sandbox/pyproject.toml` file as necessary.

```bash
# install as editable
python -m pip install -e .
```

Run the test:

```bash
python -m pytest test_hello.py
```

## Workflow

```bash
pre-commit run --all-files
```

## mdbook

Below are temporary notes, Chad to reorganize later.

```bash
mdbook build
# output: automesh/book/build

# interactive mode
mdbook serve
# on local machine, with Firefox, open the index.html file, e.g.,
# file:///Users/chovey/autotwin/automesh/book/build/index.html
```
