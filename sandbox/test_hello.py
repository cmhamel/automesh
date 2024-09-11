"""This module shows how testing works in the sandbox.

Example:
    cd ~/autotwin/automesh

    Activate the venv with one of the following:
    source .venv/bin/activate       # for bash shell
    source .venv/bin/activate.csh   # for c shell
    source .venv/bin/activate.fish  # for fish shell
    .\.venv\Scripts\activate        # for powershell

    python -m pytest sandbox/test_hello.py -v  # -v is for verbose

    to run just a single test in this module, for example
    python -m pytest sandbox/test_hello.py::test_hello
"""

import sandbox.hello as hh


def test_hello():
    """This is a minimum working example (MWE) test."""
    assert hh.hello() == "Hello World!"
