"""Python library and tools for Zissou."""

import pathlib

__version__ = open(pathlib.Path(__file__).parent / "VERSION").read().strip()

del pathlib
