"""This module shows how testing works in the sandbox.

Example:
    To run
    cd ~/autotwin/automesh
    activate the virutal environment, for example:
    source .venv/bin/activate.fish

    python -m pytest sandbox/test_hello.py -v  # -v is for verbose

    to run just a single test in this module, for example
    python -m pytest sandbox/test_hello.py::test_hello
"""
import sandbox.hello as hh


def test_hello():
    """This is a minimum working example (MWE) test."""
    assert hh.hello() == "Hello World!"
